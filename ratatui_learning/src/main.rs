use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal);
    ratatui::restore();
    result
}

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press =>   match  key.code {
            KeyCode::Char('q') => return Ok(true),
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}

fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()>{
    loop {
        terminal.draw(|frame|draw(frame))?;
        if handle_events()?{
            break Ok(());
        }
    }
}

fn draw(frame: &mut Frame) {
    let areas = Layout::vertical([Constraint::Length(1); 4]).split(frame.area());

    let line = Line::from(vec![
        Span::raw("Hello "),
        Span::styled(
            "World",
            Style::new()
                .fg(Color::Green)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        "!".red().on_light_yellow().italic(),
    ]);
    frame.render_widget(line, areas[0]);

    // using the short-hand syntax and implicit conversions
    let paragraph = Paragraph::new("Hello World!".red().on_white().bold());
    frame.render_widget(paragraph, areas[1]);

    // style the whole widget instead of just the text
    let paragraph = Paragraph::new("Hello World!").style(Style::new().red().on_white());
    frame.render_widget(paragraph, areas[2]);

    // use the simpler short-hand syntax
    let paragraph = Paragraph::new("Hello World!").blue().on_yellow();
    frame.render_widget(paragraph, areas[3]);
}