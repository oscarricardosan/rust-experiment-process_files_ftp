use tui::layout::{Alignment};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, Paragraph};
use crate::app::config::config_render::ConfigRender;
use crate::app::resources::layout::base::BaseLayout;

pub struct LayoutMain{
    config_render: ConfigRender
}

impl LayoutMain {

    pub fn new (config_render: ConfigRender)-> LayoutMain {
        LayoutMain{
            config_render
        }
    }
}


impl BaseLayout for LayoutMain {

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
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Para salir presione Alt + s.")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![
                Span::raw("Para acceder a una opción presione Alt + "),
                Span::styled(
                    "la letra subrayada",
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::raw(" como muestra el menú superior"),
            ]),
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

