use std::{borrow::BorrowMut, default, io, time::Duration};

use hidapi::{DeviceInfo, HidApi};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Alignment,
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, StatefulWidget,
        Widget,
    },
    Frame,
};

use super::Tui;

#[derive(Debug, Default)]
enum State {
    #[default]
    LoadingDevices,
    FoundDevice(DeviceList),
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    state: State,
    counter: u8,
}

#[derive(Debug)]
pub struct DeviceList {
    items: Vec<DeviceInfo>,
    state: ListState,
}

impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            // instead of doing action in the same thread, maybe start new
            // threads to do the actions
            self.process();
            self.handle_events()?;
        }

        Ok(())
    }

    fn process(&mut self) {
        match self.state {
            State::LoadingDevices => {
                if let Ok(api) = HidApi::new() {
                    self.state = State::FoundDevice(DeviceList {
                        items: api
                            .device_list()
                            .map(|dev| dev.to_owned())
                            // .map(|dev| dev.manufacturer_string())
                            // .filter(|dev| dev.is_some() && !dev.unwrap().is_empty())
                            // .map(|dev| dev.unwrap().to_string())
                            .collect(),
                        state: ListState::default(),
                    })
                }
            }
            State::FoundDevice(_) => {}
        }
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn select_next(&mut self) {
        match &mut self.state {
            State::FoundDevice(ref mut device_list) => {
                device_list.state.select_next();
            }
            _ => {}
        }
    }

    fn select_previous(&mut self) {
        match &mut self.state {
            State::FoundDevice(ref mut device_list) => {
                device_list.state.select_previous();
            }
            _ => {}
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Down => self.select_next(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn instructions_widget(&self) -> Title {
        Title::from(Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]))
    }
}

impl Widget for &mut App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        // Paragraph::new(counter_text)
        //     .centered()
        //     .block(block)
        //     .render(area, buf);

        self.state.render(area, buf);
    }
}

impl Widget for &mut State {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Title::from(" RC Car Controller ".bold());
        let block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .title(
                Title::from(Line::from(vec![" Quit ".into(), "<Q> ".yellow().bold()]))
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .border_set(border::ROUNDED)
            .padding(Padding::proportional(1));

        match self {
            State::LoadingDevices => Paragraph::new("Loading the USB devices")
                .centered()
                .block(block)
                .render(area, buf),
            State::FoundDevice(ref mut devices) => {
                let list = List::new(
                    devices
                        .items
                        .iter()
                        .map(|dev| dev.manufacturer_string().unwrap())
                        .map(|dev| ListItem::from(dev.to_owned()))
                        .collect::<Vec<ListItem>>(),
                )
                .block(block)
                .highlight_style(
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .bg(Color::Yellow)
                        .fg(Color::Black),
                )
                .highlight_symbol(">")
                .highlight_spacing(HighlightSpacing::Always);

                StatefulWidget::render(list, area, buf, &mut devices.state)
            }
        }
    }
}
