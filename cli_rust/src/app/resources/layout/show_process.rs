use std::io::Stdout;
use chrono::NaiveDateTime;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Cell, Gauge, List, ListItem, ListState, Paragraph, Row, Table, TableState};
    use crate::app::config::config_render::ConfigRender;
use crate::app::resources::layout::base::BaseLayout;
use crate::{Action, render_footer, render_tabs};
use crate::database::get_connection_postgres;

pub struct LayoutShowProcess{
    executions_in_db: Vec<postgres::Row>,
    files_in_db: Vec<postgres::Row>,
    current_action: Option<Action>,
    selected_index_list: usize,
    ratio_progress: f64,
    files_processed: i64,
}

impl LayoutShowProcess {

    pub fn new ()-> LayoutShowProcess {
        LayoutShowProcess{
            executions_in_db: Vec::new(),
            files_in_db: Vec::new(),
            current_action: None,
            selected_index_list: 0,
            ratio_progress: 0.0,
            files_processed: 0,
        }
    }

    pub fn set_current_action(&mut self, current_action: Option<Action>) {
        self.current_action= current_action;
    }

    pub fn render_special_content(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>) {

        let config_render= ConfigRender::new();

        let size = frame.size();
        let panels = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Length(config_render.height_header),
                    Constraint::Min(config_render.min_height_main),
                    Constraint::Length(config_render.height_footer),
                ].as_ref(),
            )
            .split(size);
        frame.render_widget(render_tabs(&config_render), panels[0]);

        let sub_panels = Layout::default()
            .direction(Direction::Horizontal)
            .horizontal_margin(0)
            .vertical_margin(config_render.height_header)
            .constraints([Constraint::Percentage(26), Constraint::Percentage(94)].as_ref())
            .split(frame.size());

        self.render_list(&sub_panels, frame);
        self.render_table(&sub_panels, frame);

        frame.render_widget(render_footer(), panels[2]);
    }

    fn render_list(&mut self, sub_panels: &Vec<Rect>,  frame: &mut Frame<CrosstermBackend<Stdout>>) {

        let mut items = Vec::new();
        self.executions_in_db= get_connection_postgres().query(
        "Select \
                id, start_at, end_at, total_files, files_processed_successfully,
         	    coalesce(
                    ROUND(
                        EXTRACT(epoch FROM end_at - start_at):: decimal
                    , 2 )
                    , 0
                )::character varying as total_seconds
            from executions \
            order by id desc \
            limit 60", &[]
        ).unwrap();

        for row in &self.executions_in_db {
            let end_at:Result<NaiveDateTime, postgres::error::Error>= row.try_get("end_at");
            let id:i32 = row.get("id");

            let start_at: NaiveDateTime = row.get("start_at");
            let start_at= start_at.format("%d/%m %H:%M:%S").to_string();
            let end_at_str:String = match &end_at {
                Ok(date)  => date.format("%H:%M:%S").to_string(),
                Err(_e) => String::from("__/__"),
            };

            let total_files: i32 = row.get("total_files");
            let label:Spans = match &end_at {
                Ok(_date) =>
                    Spans::from(vec![
                        Span::styled(String::from("✔️ "), Style::default().fg(Color::Cyan)),
                        Span::raw(format!("{}) {} - {} [{}]", id, start_at, end_at_str, total_files))
                    ]),
                Err(_e) =>
                    Spans::from(vec![
                        Span::styled(String::from("⏳️"), Style::default().fg(Color::Red)),
                        Span::raw(format!("{}) {} - {} [{}]", id, start_at, end_at_str, total_files))
                    ]),
            };
            items.push(ListItem::new(label));
        }

        let list= List::new(items)
            .block(
                Block::default()
                    .title("Ejecuciones")
                    .borders(Borders::ALL)
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().bg(Color::Green))
            .highlight_symbol(">>");

        let mut list_state= ListState::default();

        match self.current_action {
            Some(action)=> {
                match action {
                    Action::KeyDown=> {
                        if self.selected_index_list < self.executions_in_db.len() -1{
                            self.selected_index_list= self.selected_index_list+1
                        }
                    }
                    Action::KeyUp=> {
                        if self.selected_index_list >0{
                            self.selected_index_list= self.selected_index_list-1
                        }
                    }
                    _=> {}
                }
            }
            None=> {}
        }
        list_state.select(Some(self.selected_index_list));

        frame.render_stateful_widget(list, sub_panels[0], &mut list_state);
    }

    fn render_table(&mut self, sub_panels: &Vec<Rect>,  frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let execution_id: i32 = self.executions_in_db[self.selected_index_list].get(0);

        self.files_in_db= get_connection_postgres().query("\
            Select id, start_at, end_at, name_file \
            from files \
            where execution_id = $1
            order by id desc limit 60\
        ", &[&execution_id]).unwrap();

        let mut items= Vec::new();
        for row in &self.files_in_db {
            let id: i64 = row.get(0);
            let start_at: NaiveDateTime = row.get(1);
            let start_at= start_at.format("%H:%M:%S").to_string();
            let end_at: Result<NaiveDateTime, postgres::error::Error> = row.try_get("end_at");
            let name_file: String = row.get("name_file");

            let end_at_str:String = match &end_at {
                Ok(date)  => date.format("%H:%M:%S").to_string(),
                Err(_e) => String::from("__/__"),
            };

            let icon:String = match &end_at {
                Ok(_date)  => "1".to_string(),
                Err(_e) => "0".to_string(),
            };

            items.push(vec![
                icon, id.to_string(), start_at, end_at_str, name_file
            ]);
        }
        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);
        let header_cells = ["", "ID", "Inicio", "Fin", "Archivo"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);
        let rows = items.iter().map(|item| {
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.iter().enumerate().map(|value_cell|{
                let cell = if value_cell.0 == (0 as usize) {
                    match value_cell.1.as_str() {
                        "1"=> {
                            Cell::from("✔️ ").style(Style::default().fg(Color::Green))
                        }
                        "0"=> {
                            Cell::from("⏳").style(Style::default().fg(Color::Red))
                        }
                        _=> {Cell::from("--")}
                    }
                }else{
                    Cell::from(value_cell.1.to_string())
                };
                cell
            });
            Row::new(cells).height(height as u16).bottom_margin(0)
        });
        let mut state= TableState::default();
        state.select(None);

        // Two vertical panels
        let grid_vertical_subpanel_1 = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Length(3)].as_ref())
            .split(sub_panels[1]);

        let grid_horizontal_subpanel_1_2 = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Length(25)].as_ref())
            .split(grid_vertical_subpanel_1[0]);


        let table= Table::new(rows)
            .header(header)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Log")
            )
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Min(2),
                Constraint::Min(8),
                Constraint::Min(10),
                Constraint::Min(10),
                Constraint::Min(10),
            ]);
        frame.render_stateful_widget(table, grid_horizontal_subpanel_1_2[0], &mut state);

        self.render_progress_bar(&grid_vertical_subpanel_1, frame);
        self.render_info_panel(&grid_horizontal_subpanel_1_2, frame);
    }

    pub fn render_progress_bar(&mut self, grid_vertical_subpanel_1: &Vec<Rect>,  frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let execution_id: i32 = self.executions_in_db[self.selected_index_list].get(0);

        let progress_db= get_connection_postgres().query_one("\
            with execution as (
                select total_files from executions where id = $1
            )
            select
                (
                    count(*)::float / (select execution.total_files from execution)::float
                ):: float as progress,
                count(*) as files_processed
            from files
            where end_at is not null and execution_id = $1
        ", &[&execution_id]).unwrap();

        let ratio:Result<f64, postgres::error::Error> = progress_db.try_get("progress");
        self.ratio_progress = match ratio {
            Ok(ratio) => ratio,
            Err(_e) => 0.0,
        };
        let files_processed:Result<i64, postgres::error::Error> = progress_db.try_get("files_processed");
        self.files_processed = match files_processed {
            Ok(files_processed) => files_processed,
            Err(_e) => 0,
        };
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Yellow))
            .ratio(self.ratio_progress)
            .block(
                Block::default()
                    .style(Style::default().bg(Color::Black))
                    .borders(Borders::ALL)
            )
            .label(format!("Progreso {:.1$} %", self.ratio_progress*100 as f64, 2))
            .use_unicode(true);
        frame.render_widget(gauge, grid_vertical_subpanel_1[1]);
    }

    pub fn render_info_panel(&self, grid_horizontal_subpanel_1_2: &Vec<Rect>,  frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let execution = &self.executions_in_db[self.selected_index_list];
        let id: i32 = execution.get("id");
        let end_at: Result<NaiveDateTime, postgres::error::Error>= execution.try_get("end_at");
        let start_at: NaiveDateTime = execution.get("start_at");
        let start_at= start_at.format("%Y-%m-%d %H:%M:%S").to_string();

        let total_seconds:String= execution.get("total_seconds");
        let total_files: i32 = execution.get("total_files");

        let end_at_str:String = match &end_at {
            Ok(date)  => date.format("%Y-%m-%d %H:%M:%S").to_string(),
            Err(_e) => String::from("__/__"),
        };

        let state_span:Span = match &end_at {
            Ok(_date)  => Span::styled("  Finalizado  ",Style::default().bg(Color::Green)),
            Err(_e) => Span::styled("  En proceso  ",Style::default().bg(Color::Red).add_modifier(Modifier::REVERSED)),
        };

        let paragraph= Paragraph::new(
        vec![
                Spans::from(vec![Span::styled("Lote ID:",Style::default().add_modifier(Modifier::BOLD))]),
                Spans::from(vec![Span::raw(format!("   {}", id))]),
                Spans::from(vec![Span::styled("Total archivos:",Style::default().add_modifier(Modifier::BOLD))]),
                Spans::from(vec![Span::raw(format!("   {}", total_files))]),
                Spans::from(vec![Span::raw("")]),
                Spans::from(vec![Span::styled("Archivos procesados:",Style::default().add_modifier(Modifier::BOLD))]),
                Spans::from(vec![Span::raw(format!("   {} ( {:.2$} %)", self.files_processed, self.ratio_progress * 100 as f64, 2))]),
                Spans::from(vec![Span::raw("")]),
                Spans::from(vec![Span::styled("Inicio ejecución:",Style::default().add_modifier(Modifier::BOLD))]),
                Spans::from(vec![Span::raw(format!("{}", start_at))]),
                Spans::from(vec![Span::raw("")]),
                Spans::from(vec![Span::styled("Fin ejecución:",Style::default().add_modifier(Modifier::BOLD))]),
                Spans::from(vec![Span::raw(format!("{}", end_at_str))]),
                Spans::from(vec![Span::raw("")]),
                Spans::from(vec![Span::styled("Estado:  ",Style::default().add_modifier(Modifier::BOLD))]),
                Spans::from(vec![state_span]),
                Spans::from(vec![Span::raw("")]),
                Spans::from(vec![Span::styled("Tiempo de ejecución:",Style::default().add_modifier(Modifier::BOLD))]),
                Spans::from(vec![Span::raw(format!("   {} segundos", total_seconds))]),
            ]
        )
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Datos de ejecución")
            )
            .alignment(Alignment::Left);
        frame.render_widget(paragraph, grid_horizontal_subpanel_1_2[1]);
    }
}


impl BaseLayout for LayoutShowProcess {

    fn render_content(&self) -> Paragraph{
        let content= Paragraph::new("a");
        content
    }

}

