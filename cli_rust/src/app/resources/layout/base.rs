use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Paragraph, Tabs};
use crate::app::config::config_render::ConfigRender;

pub trait BaseLayout{

    fn render_content(&self) -> Paragraph;

    fn render(&self, frame: &mut Frame<CrosstermBackend<Stdout>>) {

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
        frame.render_widget(self.render_content(), panels[1]);
        frame.render_widget(render_footer(), panels[2]);
    }

}

pub fn render_footer() ->Paragraph<'static> {

    let copyright = Paragraph::new("Savne SAS 2021 - Todos los derechos reservados")
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Copyright")
                .border_type(BorderType::Plain),
        );
    copyright
}




pub fn render_tabs(config_render: &ConfigRender)->Tabs {
    let tabs = Tabs::new(render_menu(config_render))
        .select(1)
        .block(Block::default().title("Menu").borders(Borders::ALL))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Yellow))
        .divider(Span::raw("|"));
    tabs
}

fn render_menu(config_render: &ConfigRender)->Vec<Spans>{
    let menu= config_render.get_main_titles()
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    menu
}
