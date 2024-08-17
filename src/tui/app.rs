use std::{borrow::BorrowMut, default, io, time::Duration};

use hidapi::{DeviceInfo, HidApi, HidDevice};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Alignment,
    style::{Color, Modifier, Style, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, HighlightSpacing, List, ListDirection, ListItem, ListState, Padding, Paragraph,
        StatefulWidget, Widget,
    },
    Frame,
};

use super::Tui;

const WAIT_TIME: Duration = Duration::from_millis(16);
const BUF_SIZE: usize = 32;

#[derive(Debug, Default)]
enum State {
    #[default]
    LoadingDevices,
    FoundDevice(DeviceList),
    UseDevice(DeviceInfo),
    OpenedDevice(Device),
}

type DeviceData = [u8; BUF_SIZE];

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    state: State,
}

#[derive(Debug)]
struct Device {
    device: HidDevice,
    data: Option<DeviceData>,
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

    // TODO: make this async or multi-threaded
    fn process(&mut self) {
        match &mut self.state {
            State::LoadingDevices => {
                if let Ok(api) = HidApi::new() {
                    self.state = State::FoundDevice(DeviceList {
                        items: api.device_list().map(|dev| dev.to_owned()).collect(),
                        state: ListState::default(),
                    })
                }
            }
            State::UseDevice(chosen_device) => {
                if let Ok(api) = HidApi::new() {
                    match chosen_device.open_device(&api) {
                        Ok(device) => {
                            self.state = State::OpenedDevice(Device { device, data: None })
                        }
                        // FIX: this is a very ugly, should handle failing to open device
                        Err(e) => panic!("got error while opening device: {}", e),
                    }
                } else {
                    panic!("Couldn't open hidapi.");
                }
            }
            State::OpenedDevice(ref mut device) => {
                if let Some(ref mut buf) = device.data {
                    device
                        .device
                        .read_timeout(buf, WAIT_TIME.as_millis() as i32)
                        .unwrap();
                } else {
                    let mut buf = [0; BUF_SIZE];
                    device
                        .device
                        .read_timeout(&mut buf, WAIT_TIME.as_millis() as i32)
                        .unwrap();
                    device.data = Some(buf)
                }
            }
            _ => {}
        }
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn select_next(&mut self) {
        if let State::FoundDevice(ref mut device_list) = &mut self.state {
            device_list.state.select_next();
        }
    }

    fn select_previous(&mut self) {
        if let State::FoundDevice(ref mut device_list) = &mut self.state {
            device_list.state.select_previous();
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

    fn choose_selected_device(&mut self) {
        if let State::FoundDevice(list) = &self.state {
            if let Some(dev_index) = list.state.selected() {
                self.state = State::UseDevice(list.items[dev_index].clone());
            }
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => self.select_previous(),
            KeyCode::Down => self.select_next(),
            KeyCode::Enter => self.choose_selected_device(),
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
            State::OpenedDevice(device) => {
                if let Some(data) = device.data {
                    Paragraph::new(format!("Received data: {:?}", data[2]))
                        .block(block)
                        .render(area, buf);
                } else {
                    Paragraph::new("No data yet").block(block).render(area, buf);
                }
            }
            _ => Paragraph::new(format!("Currently in state: {:?}", self)).render(area, buf),
        }
    }
}
