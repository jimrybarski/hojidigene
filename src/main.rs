use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph, Tabs, Wrap},
    Frame, Terminal,
};

#[derive(PartialEq, Debug)]
enum Mode {
    Quit,
    SequenceViewer,
}

struct App {
    mode: Mode,
}

impl App {
    fn new() -> App {
        App {
            mode: Mode::SequenceViewer,
        }
    }
}

fn render_sequence_viewer<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().style(Style::default().bg(Color::Black).fg(Color::White));
    f.render_widget(block, size);
    let paragraph = Paragraph::new("ACGGATCACTGATCGGTTTACGTACGTACGTACGATCGATCGTACGTACGATCGATCGATCGTACGTACGTACGTACGTACGTACGTACGTACGTACGACGGGGAGCGGCCGCTATAATATATTACGCAGACTATCGGCGCTATACTATATCGATGCTACGTACGTACGTACGATCGATCGTACGATCGTACGTACGTACGTACGATATATCATCGTACGTACGT")
        .style(Style::default().bg(Color::Black).fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, chunks[1]);
}

fn action_sequence_viewer(app: &mut App, key: KeyCode) -> Mode {
    assert_eq!(app.mode, Mode::SequenceViewer);
    match key {
        KeyCode::Esc => Mode::Quit,
        _ => Mode::SequenceViewer,
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    // Start up in Sequence Viewer mode as the default
    terminal.draw(|f| render_sequence_viewer(f, &app))?;

    loop {
        if let Event::Key(key) = event::read()? {
            // use the keypress to decide whether to edit the state
            // of the App and/or change the Mode
            app.mode = match app.mode {
                Mode::SequenceViewer => action_sequence_viewer(app, key.code),
                Mode::Quit => Mode::Quit,
            };
            let render_func = match app.mode {
                Mode::SequenceViewer => render_sequence_viewer,
                Mode::Quit => return Ok(()),
            };

            terminal.draw(|f| render_func(f, &app))?;
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // run app
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // shut down terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // in case there was an error, print the raw error message
    // this should effectively never happen
    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}
