use std::io::Stdout;
use tui::backend::CrosstermBackend;
use tui::Frame;
use tui::layout::{Alignment, Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, BorderType, List, ListItem, Paragraph};
use crate::app::config::config_render::ConfigRender;
use crate::app::resources::layout::base::BaseLayout;
use crate::{render_footer, render_tabs};

pub struct LayoutShowProcess{
    config_render: ConfigRender
}

impl LayoutShowProcess {

    pub fn new (config_render: ConfigRender)-> LayoutShowProcess {
        LayoutShowProcess{
            config_render
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

        let items = [ListItem::new(">> Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
        let list= List::new(items)
            .block(
                Block::default()
                    .title(vec![
                        Span::styled("With", Style::default().fg(Color::Yellow)),
                        Span::from(" background"),
                    ])
                    .borders(Borders::ALL)
            )
            .style(Style::default().fg(Color::White))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
            .highlight_symbol(">>");

        frame.render_widget(list, sub_panels[0]);

        // Top left inner block with green background
        let right_container = Block::default()
            .title(vec![
                Span::styled("With", Style::default().fg(Color::Yellow)),
                Span::from(" background---"),
            ])
            .style(Style::default().bg(Color::Red));
        frame.render_widget(right_container, sub_panels[1]);

        frame.render_widget(render_footer(), panels[2]);
    }
}


impl BaseLayout for LayoutShowProcess {

    fn render_content(&self) -> Paragraph{
        let content = Paragraph::new(vec![
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("Bienvenido")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::raw("al")]),
            Spans::from(vec![Span::raw("")]),
            Spans::from(vec![Span::styled(
                "cliente gráfico sa dsa dFTP-CLI",
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

        content
    }

}

