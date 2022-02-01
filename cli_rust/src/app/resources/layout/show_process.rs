use std::io::Stdout;
use chrono::NaiveDateTime;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Cell, Gauge, List, ListItem, ListState, Paragraph, Row, Table, TableState};
use crate::app::config::config_render::ConfigRender;
use crate::app::resources::layout::base::BaseLayout;
use crate::{render_footer, render_tabs};
use crate::database::get_connection_postgres;

pub struct LayoutShowProcess{
}

impl LayoutShowProcess {

    pub fn new ()-> LayoutShowProcess {
        LayoutShowProcess{
        }
    }

    pub fn render_special_content(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {

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
            .constraints([Constraint::Percentage(30), Constraint::Percentage(50)].as_ref())
            .split(frame.size());

        self.render_list(&sub_panels, frame);
        self.render_table(&sub_panels, frame);

        frame.render_widget(render_footer(), panels[2]);
    }

    fn render_list(&self, sub_panels: &Vec<Rect>,  frame: &mut Frame<CrosstermBackend<Stdout>>) {

        let mut items = Vec::new();
        for row in get_connection_postgres().query(
            "Select id, start_at, end_at, total_files from executions order by id desc limit 20", &[]
        ).unwrap() {
            let id: i32 = row.get(0);
            let start_at: NaiveDateTime = row.get(1);
            let start_at= start_at.format("%d/%m %H:%M:%S").to_string();
            let end_at: NaiveDateTime = row.get(2);
            let end_at= end_at.format("%H:%M:%S").to_string();
            let total_files: i32 = row.get(3);
            items.push(ListItem::new(format!("{}) {} - {} [{}]", id, start_at, end_at, total_files)))
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
        list_state.select(Some(0));
        frame.render_stateful_widget(list, sub_panels[0], &mut list_state);
    }

    fn render_table(&self, sub_panels: &Vec<Rect>,  frame: &mut Frame<CrosstermBackend<Stdout>>) {

        let mut items= vec![
            vec!["Row11", "Row12", "Row13"],
            vec!["Row21", "Row22", "Row23"],
            vec!["Row31", "Row32", "Row33"],
            vec!["Row41", "Row42", "Row43"],
            vec!["Row51", "Row52", "Row53"],
            vec!["Row61", "Row62\nTest", "Row63"],
            vec!["Row71", "Row72", "Row73"],
            vec!["Row81", "Row82", "Row83"],
            vec!["Row91", "Row92", "Row93"],
            vec!["Row101", "Row102", "Row103"],
            vec!["Row111", "Row112", "Row113"],
            vec!["Row121", "Row122", "Row123"],
            vec!["Row131", "Row132", "Row133"],
            vec!["Row141", "Row142", "Row143"],
            vec!["Row151", "Row152", "Row153"],
            vec!["Row161", "Row162", "Row163"],
            vec!["Row171", "Row172", "Row173"],
            vec!["Row181", "Row182", "Row183"],
            vec!["Row191", "Row192", "Row193"],
        ];

        let right_container = Block::default()
            .borders(Borders::ALL)
            .title("Log");

        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Blue);
        let header_cells = ["Header1", "Header2", "Header3"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
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
            let cells = item.iter().map(|c| Cell::from(*c));
            Row::new(cells).height(height as u16).bottom_margin(1)
        });
        let mut state= TableState::default();
        state.select(Some(1));
        let t = Table::new(rows)
            .header(header)
            .block(right_container)
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Min(10),
            ]);
        frame.render_stateful_widget(t, sub_panels[1], &mut state);

        let gauge = Gauge::default()
            .block(Block::default().title("Gauge3").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Yellow))
            .ratio(0.10 as f64)
            .label("Progreso")
            .use_unicode(true);
        frame.render_widget(gauge, sub_panels[1]);
    }
}


impl BaseLayout for LayoutShowProcess {

    fn render_content(&self) -> Paragraph{
        let content= Paragraph::new("a");
        content
    }

}

