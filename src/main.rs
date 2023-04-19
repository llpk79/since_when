/*
Written in rust, using iced for the gui, and rusqlite for the database.
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
         - A button labeled "Add" allows the user to add the event to the database.
         - Clicking the "Update" button adds an Occurrence of the event to the database.
         - Clicking the "Delete" button removes the event from the database.
     - The main page showing all events tracked by the app and elapsed time since the event.
         - A list of events tracked for the day is displayed.
         - Elapsed time in days since the last event and average time between events are
         displayed to the left of the event name.
         - A button labeled Calendar is displayed at the bottom of the page.
*/

mod add_event;
mod calendar;
mod events;
use iced::theme::Theme;
use iced::widget::{container, scrollable};
use iced::{executor, Application, Command, Element, Length, Settings};
use rusqlite::{params, Connection, Result};

/// Setup rusqlite connection.
pub fn setup_connection() -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("since_when.db")?;
    Ok(conn)
}

/// Setup the database tables.
pub fn setup_tables(conn: &Connection) {
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL UNIQUE
                  );",
        params![],
    ) {
        Ok(created) => {
            println!("Created table: {}", created);
        }
        Err(e) => {
            println!("Error creating table: {}", e);
        }
    }
    match conn.execute(
        "CREATE TABLE IF NOT EXISTS occurrences (
                  event_id        INTEGER,
                  date            TEXT NOT NULL,
                  FOREIGN KEY(event_id) REFERENCES events(id)
                  );",
        params![],
    ) {
        Ok(created) => {
            println!("Created table: {}", created);
        }
        Err(e) => {
            println!("Error creating table: {}", e);
        }
    }
}

pub fn insert_test_event(conn: &Connection) {
    match conn.execute(
        "INSERT INTO events (name) VALUES (?1), (?2);",
        params!["Pooper empty", "Propane tank full"],
    ) {
        Ok(inserted) => {
            println!("Record inserted: {}", inserted);
            // Insert test occurrence.
            match conn.execute(
                "INSERT INTO occurrences (event_id, date) VALUES (?1, ?2), (?3, ?4), (?5, ?6), (?7, ?8);",
                params![
                    1i32,
                    "2023-04-01".to_string(),
                    2i32,
                    "2023-04-12".to_string(),
                    1i32,
                    "2023-04-06".to_string(),
                    1i32,
                    "2023-04-11".to_string(),
                ],
            ) {
                Ok(inserted) => println!("Record inserted: {}", inserted),
                Err(e) => println!("Error inserting record: {}", e),
            }
        }
        Err(e) => println!("Error inserting record: {}", e),
    }
}
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
    UpdateEvent,
    DeleteEvent,
    CalendarWindow,
    EventsWindow,
    AddEvent,
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
    fn new(_flags: ()) -> (Self, Command<AppMessage>) {
        let conn = setup_connection().unwrap();
        setup_tables(&conn);
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
    fn title(&self) -> String {
        String::from("Since When?")
    }

    /// The update function.
    ///
    /// This function is called when a message is received.
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        // println!("Message: {:?}", message);
        match message {
            AppMessage::NextMonth => {
                let _ = self.calendar.update(AppMessage::NextMonth);
            }
            AppMessage::PreviousMonth => {
                let _ = self.calendar.update(AppMessage::PreviousMonth);
            }
            AppMessage::DayClicked(day, month, year) => {
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
    /// This function is called when the application needs to be drawn.
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
}

/// The main function.
pub fn main() -> iced::Result {
    SinceWhen::run(Settings::default())
}
