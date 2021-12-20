use tui::layout::{Alignment};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Paragraph};
use crate::app::config::config_render::ConfigRender;
use crate::BaseLayout;

pub struct LayoutHelp {
    config_render: ConfigRender
}

impl LayoutHelp {
    pub fn new(config_render: ConfigRender) -> LayoutHelp {
        LayoutHelp {
            config_render
        }
    }
}

impl BaseLayout for LayoutHelp {

    fn render_content(&self) -> Paragraph{
        let home = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Bienvenido")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("al")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "cliente gráfico FTP-CLI",
                Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD),
            )]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Hola mundo como estaś")]),
        ])
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
            );

        home
    }


}


