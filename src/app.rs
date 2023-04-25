use crate::add_event;
use crate::calendar;
use crate::database;
use crate::events;
use iced::theme::Theme;
use iced::widget::{container, scrollable};
use iced::{executor, Application, Command, Element, Length};

/// Application struct.
pub struct SinceWhen {
    day: u32,
    month: u32,
    year: i32,
    current_page: Page,
    calendar: calendar::Calendar,
    events: events::EventsPage,
    add_event: add_event::AddEvent,
}

/// Application messages.
#[derive(Debug, Clone)]
pub enum AppMessage {
    NextMonth,
    PreviousMonth,
    DayClicked(u32, u32, i32),
    AddEvent,
    UpdateEvent,
    DeleteEvent,
    CalendarWindow,
    EventsWindow,
    TextEvent(String),
}

/// Application pages.
#[derive(Debug, Clone, Copy)]
pub enum Page {
    Calendar,
    Events,
    AddEvent,
}

/// The SinceWhen application.
impl Application for SinceWhen {
    type Executor = executor::Default;
    type Message = AppMessage;
    type Theme = Theme;
    type Flags = ();

    /// Creates a new app.
    ///
    /// # Arguments
    /// - flags: `()`
    ///
    /// # Returns
    /// - `(Self, Command<AppMessage>)`
    fn new(_flags: ()) -> (Self, Command<AppMessage>) {
        let conn = database::setup_connection();
        database::setup_tables(&conn);
        // database::insert_test_event(&conn);
        (
            Self {
                day: 0,
                month: 0,
                year: 0,
                current_page: Page::Events,
                calendar: calendar::Calendar::new(),
                events: events::EventsPage::new(),
                add_event: add_event::AddEvent::new(),
            },
            Command::none(),
        )
    }

    /// The title of the application.
    ///
    /// # Returns
    /// - String
    fn title(&self) -> String {
        String::from("Since When?")
    }

    /// The update function.
    ///
    /// # Arguments
    /// - message: `AppMessage`
    ///
    /// # Returns
    /// - `Command<AppMessage>`
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            AppMessage::NextMonth => {
                let _ = self.calendar.update(AppMessage::NextMonth);
            }
            AppMessage::PreviousMonth => {
                let _ = self.calendar.update(AppMessage::PreviousMonth);
            }
            AppMessage::DayClicked(day, month, year) => {
                if day == 0 {
                    return Command::none();
                }
                self.day = day;
                self.month = month;
                self.year = year;
                self.current_page = Page::AddEvent;
            }
            AppMessage::UpdateEvent => {
                let _ =
                    self.add_event
                        .update(AppMessage::UpdateEvent, self.day, self.month, self.year);
            }
            AppMessage::AddEvent => {
                let _ =
                    self.add_event
                        .update(AppMessage::AddEvent, self.day, self.month, self.year);
            }
            AppMessage::DeleteEvent => {
                let _ =
                    self.add_event
                        .update(AppMessage::DeleteEvent, self.day, self.month, self.year);
            }
            AppMessage::TextEvent(event) => {
                let _ = self.add_event.update(
                    AppMessage::TextEvent(event),
                    self.day,
                    self.month,
                    self.year,
                );
            }
            AppMessage::CalendarWindow => {
                self.current_page = Page::Calendar;
            }
            AppMessage::EventsWindow => {
                self.current_page = Page::Events;
            }
        }
        Command::none()
    }

    /// The view function.
    ///
    /// # Returns
    /// - `Element<'static, Self::Message>`
    fn view(&self) -> Element<'static, Self::Message> {
        let content = match self.current_page {
            Page::Calendar => self.calendar.view(),
            Page::Events => self.events.view(),
            Page::AddEvent => self.add_event.view(self.day, self.month, self.year),
        };
        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    /// The application theme.
    ///
    /// # Returns
    /// - `Self::Theme`
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}
