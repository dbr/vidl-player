#[allow(dead_code)]
mod util;

use crate::util::{
    event::{Event, Events},
    StatefulList,
};
use std::{error::Error, io, path::PathBuf};
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
    channels: StatefulList<(String, usize)>,
    videos: StatefulList<(&'a str, usize)>,
    app_mode: AppMode,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        let mut data = list_videos("/mnt/freenas_misc/vidl".into());
        data.sort_videos();

        let mut channels = vec![];
        for (chan_title, chan_info) in &data.channels {
            channels.push((chan_title.clone(), chan_info.videos.len()));
        }

        channels.sort_by(|a, b| {a.partial_cmp(&b).unwrap()});

        App {
            app_mode: AppMode::ChannelList,
            channels: StatefulList::with_items(channels),
            videos: StatefulList::with_items(vec![("Hm", 1), ("Yep", 2)]),
        }
    }
}

use std::collections::HashMap;

#[derive(Debug)]
struct Data {
    channels: HashMap<String, Channel>,
}
impl Data {
    fn sort_videos(&mut self) {
        for (t, c) in &mut self.channels {
            c.videos.sort_by(|a, b| a.path.partial_cmp(&b.path).unwrap());
        }
    }
}
#[derive(Debug)]
struct Video {
    title: String,
    path: PathBuf,
}
#[derive(Debug)]
struct Channel {
    videos: Vec<Video>,
}

fn list_videos(path: std::path::PathBuf) -> Data {
    let mut ret = Data {
        channels: HashMap::new(),
    };

    let mut files = vec![];
    for f in std::fs::read_dir(path).unwrap() {
        files.push(f.unwrap());
    }

    files.sort_by(|a, b| a.path().partial_cmp(&b.path()).unwrap());

    for info in files {
        if info.file_type().unwrap().is_file() {
            let raw_filename = info.file_name();
            let filename = raw_filename.to_str().unwrap();
            let (chan, title) = match filename.find("__") {
                Some(idx) => filename.split_at(idx),
                None => continue,
            };
            let title = title.split_at(3).1;
            ret.channels
                .entry(chan.into())
                .or_insert(Channel { videos: vec![] })
                .videos
                .push(Video{title: title.into(), path: info.path()});
        }
    }

    ret
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
                .block(Block::default().borders(Borders::ALL).title("Channels"))
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
