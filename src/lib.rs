#[allow(dead_code)]
mod util;

use crate::util::{
    event::{Event, Events},
    StatefulList,
};
use std::{error::Error, io};
use termion::{event::Key, input::MouseTerminal, raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

enum AppMode {
    ChannelList,
    VideoList,
}
struct App<'a> {
    channels: StatefulList<(&'a str, usize)>,
    videos: StatefulList<(&'a str, usize)>,
    app_mode: AppMode,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            app_mode: AppMode::ChannelList,
            channels: StatefulList::with_items(vec![
                ("Item0", 1),
                ("Item1", 2),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 4),
                ("Item6", 1),
                ("Item7", 3),
                ("Item8", 1),
                ("Item9", 6),
                ("Item10", 1),
                ("Item11", 3),
                ("Item12", 1),
                ("Item13", 2),
                ("Item14", 1),
                ("Item15", 1),
                ("Item16", 4),
                ("Item17", 1),
                ("Item18", 5),
                ("Item19", 4),
                ("Item20", 1),
                ("Item21", 2),
                ("Item22", 1),
                ("Item23", 3),
                ("Item24", 1),
            ]),
            videos: StatefulList::with_items(vec![("Hm", 1), ("Yep", 2)]),
        }
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    // Create a new app with some exapmle state
    let mut app = App::new();

    loop {
        terminal.draw(|frame| {
            // Create two chunks with equal horizontal screen space
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
                .split(frame.size());

            // Iterate through all elements in the `items` app and append some debug text to it.
            let items: Vec<ListItem> = app
                .channels
                .items
                .iter()
                .map(|i| {
                    let lines = vec![Spans::from(format!("{} ({})", i.0, i.1))];
                    ListItem::new(lines).style(Style::default())
                })
                .collect();

            // Create a List from all list items and highlight the currently selected one
            let items = List::new(items)
                .block(Block::default().borders(Borders::ALL).title("List"))
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

            // We can now render the item list
            frame.render_stateful_widget(items, chunks[0], &mut app.channels.state);

            // Iterate through all elements in the `items` app and append some debug text to it.
            let vids: Vec<ListItem> = app
                .videos
                .items
                .iter()
                .map(|i| {
                    let lines = vec![Spans::from(format!("{} ({})", i.0, i.1))];
                    ListItem::new(lines).style(Style::default())
                })
                .collect();

            // Create a List from all list items and highlight the currently selected one
            let items = List::new(vids)
                .block(Block::default().borders(Borders::ALL).title("Videos"))
                .highlight_style(
                    Style::default()
                        .bg(Color::DarkGray)
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("> ");

            // We can now render the item list
            frame.render_stateful_widget(items, chunks[1], &mut app.videos.state);
        })?;

        match events.next()? {
            Event::Input(input) => match app.app_mode {
                AppMode::ChannelList => match input {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Left => {
                        app.channels.unselect();
                    }
                    Key::Down => {
                        app.channels.next();
                    }
                    Key::Up => {
                        app.channels.previous();
                    }
                    Key::Right => {
                        app.app_mode = AppMode::VideoList;
                        app.videos.next();
                    }
                    _ => {}
                },
                AppMode::VideoList => match input {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Left => {
                        app.videos.unselect();
                        app.app_mode = AppMode::ChannelList;
                    }

                    Key::Down => {
                        app.videos.next();
                    }
                    Key::Up => {
                        app.videos.previous();
                    }
                    _ => {}
                },
            },
            Event::Tick => {}
        }
    }

    Ok(())
}
