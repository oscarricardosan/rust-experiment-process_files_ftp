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
            Spans::from(vec![Span::raw("Para salir presione Ctrl + c. Para acceder a alguna opción presione Ctrl + la letra con subrayado que indica en el menú")]),
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

