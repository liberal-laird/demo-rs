use color_eyre::Result;
use crossterm::event::{self, Event};
use ratatui::{
    DefaultTerminal, Frame,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);

    ratatui::restore();

    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let b = Block::default()
        .title(Line::from("Left Title").left_aligned())
        .title(Line::from("Middle Title").centered())
        .title(Line::from("Right Title").right_aligned())
        .borders(Borders::ALL);

    frame.render_widget(b.clone(), frame.area());

    let content = Paragraph::new("Hello").alignment(ratatui::layout::Alignment::Center);

    let inner = b.inner(frame.area());

    frame.render_widget(content, inner);
}
