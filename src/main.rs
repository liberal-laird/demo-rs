use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::style::Color;

use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::widgets::Block;
use ratatui::widgets::Gauge;
use ratatui::widgets::Widget;
use ratatui::{DefaultTerminal, Frame, text::Line};

use std::io;

pub struct App {
    exit: bool,
    progress_bar_color: Color,
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App {
        exit: false,
        progress_bar_color: Color::Green,
    };

    let result = app.run(&mut terminal);

    ratatui::restore();

    result
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            match crossterm::event::read()? {
                crossterm::event::Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => {}
            }
            terminal.draw(|f| self.draw(f))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true
        } else if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('c') {
            if self.progress_bar_color == Color::Green {
                self.progress_bar_color = Color::LightYellow
            } else {
                self.progress_bar_color = Color::Green
            }
        }
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let vertical_layout =
            Layout::vertical([Constraint::Percentage(20), Constraint::Percentage(80)]);
        let [title_area, gauge_area] = vertical_layout.areas(area);
        Line::from("Process overview")
            .bold()
            .render(title_area, buf);

        let instructions = Line::from(vec![
            "Change color".into(),
            "<C>".blue().bold(),
            "Quit".into(),
            "<Q>".blue().bold(),
        ])
        .centered();

        let block = Block::bordered()
            .title(Line::from(" Background processes"))
            .title_bottom(instructions)
            .border_set(border::THICK);

        let progress_bar = Gauge::default()
            .gauge_style(Style::default().fg(self.progress_bar_color))
            .block(block)
            .label(format!("Process 1: 50%"))
            .ratio(0.5);

        progress_bar.render(
            Rect {
                x: gauge_area.left(),
                y: gauge_area.top(),
                width: gauge_area.width,
                height: 3,
            },
            buf,
        );
    }
}
