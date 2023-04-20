/*
Written in rust, using iced for the gui, and rusqlite for the data_base.
This app is to track the time since an Event has happened, expressed in days.
For example, the time since you last changed your oil, or the time since you last had a haircut.
The app is designed to be simple, and easy to use.
The app has three windows;
     - A window displaying a Calendar.
         - The Calendar is displayed in a grid of 7 columns and 6 rows.
         - Each cell in the grid is a button labeled with the day of the month.
         - The month and year are displayed above the Calendar.
         - Arrow buttons allow the user to navigate between months.
     - Clicking a Calendar date opens a page for entering new events.
         - A text box allows the user to enter a new event.
         - A button labeled "Add" allows the user to add the event to the data_base.
         - Clicking the "Update" button adds an Occurrence of the event to the data_base.
         - Clicking the "Delete" button removes the event from the data_base.
     - The main page showing all events tracked by the app and elapsed time since the event.
         - A list of events tracked for the day is displayed.
         - Elapsed time in days since the last event and average time between events are
         displayed to the left of the event name.
         - A button labeled Calendar is displayed at the bottom of the page.
*/

extern crate core;

mod add_event;
mod calendar;
mod events;
mod database;
mod utils;
use iced::theme::Theme;
use iced::widget::{container, scrollable};
use iced::{executor, Application, Command, Element, Length, Settings};


/// Application struct.
struct SinceWhen {
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
    /// - flags: ()
    ///
    /// # Returns
    /// - (Self, Command<AppMessage>)
    fn new(_flags: ()) -> (Self, Command<AppMessage>) {
        let conn = database::setup_connection();
        database::setup_tables(&conn);
        // insert_test_event(&conn);
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
    /// - message: AppMessage
    ///
    /// # Returns
    /// - Command<AppMessage>
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
            AppMessage::CalendarWindow => {
                self.current_page = Page::Calendar;
            }
            AppMessage::EventsWindow => {
                self.current_page = Page::Events;
            }
            AppMessage::AddEvent => {
                self.current_page = Page::AddEvent;
                let _ =
                    self.add_event
                        .update(AppMessage::AddEvent, self.day, self.month, self.year);
            }
            AppMessage::TextEvent(event) => {
                let _ = self.add_event.update(
                    AppMessage::TextEvent(event),
                    self.day,
                    self.month,
                    self.year,
                );
            }
            AppMessage::DeleteEvent => {
                let _ =
                    self.add_event
                        .update(AppMessage::DeleteEvent, self.day, self.month, self.year);
            }
        }
        Command::none()
    }

    /// The view function.
    ///
    /// # Returns
    /// - Element<'static, Self::Message>
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
    /// - Self::Theme
    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

/// The main function.
pub fn main() -> iced::Result {
    SinceWhen::run(Settings::default())
}
