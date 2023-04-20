use crate::AppMessage;
use iced::alignment::Horizontal;
use iced::widget::{button, column, text, text_input, Row};
use iced::{theme, Alignment, Command, Element};
use rusqlite::{Connection, Result, params};

const TEXT_SIZE: u16 = 40;
const SPACING: u16 = 20;
const ADD_BUTTON_SIZE: u16 = 225;

/// AddEvent state.
#[derive(Debug, Clone)]
pub struct AddEvent {
    date: chrono::NaiveDate,
    event: String,
}

/// AddEvent implementation.
impl<'a> AddEvent {
    pub fn new() -> AddEvent {
        Self {
            date: chrono::NaiveDate::from_ymd_opt(1, 3, 3).unwrap(),
            event: String::new(),
        }
    }

    /// Perform a SQL insert with variable parameters.
    ///
    /// # Arguments
    /// - conn: &rusqlite::Connection - The data_base connection.
    /// - id: (i32, bool) - The id of the event to insert.
    /// - date: (&str, bool) - The date of the occurrence to insert.
    /// - event: (&str, bool) - The name of the event to insert.
    ///     - The bool portion of the tuple is a flag to determine if the parameter should be used.
    /// - sql: &str - The SQL statement to execute.
    ///
    /// # Returns
    /// - Result<i32, rusqlite::Error> - bool success flag.
    fn sql_insert(
        &self,
        conn: &Connection,
        id: (i32, bool),
        date: (&str, bool),
        event: (&str, bool),
        sql: &str,
    ) -> Result<i32, rusqlite::Error> {
        let mut stmt = conn.prepare(sql).unwrap();
        // Match on the flags to determine which parameters to use.
        match (id.1, date.1, event.1) {
            // Add occurrence.
            (true, true, false) => match stmt.execute(params![id.0, date.0]) {
                Ok(success) => success,
                Err(e) => {
                    println!("Error: {:?}", e);
                    0
                }
            },
            // Add event.
            (false, false, true) => match stmt.execute(params![event.0]) {
                Ok(success) => success,
                Err(e) => {
                    println!("Error: {:?}", e);
                    0
                }
            },
            // Update or delete event.
            (true, false, false) => match stmt.execute(params![id.0]) {
                Ok(success) => success,
                Err(e) => {
                    println!("Error: {:?}", e);
                    0
                }
            },
            _ => 0, // This should never happen.
        };
        Ok(id.0)
    }

    /// Get the id of the event.
    ///
    /// # Arguments
    /// - conn: &rusqlite::Connection - The data_base connection.
    ///
    /// # Returns
    /// - i32 - The id of the event.
    fn get_event_id(&self, conn: &Connection) -> i32 {
        struct ID {
            id: i32,
        }
        println!("Getting event id for {:?}", &self.event);
        let mut id_stmt = conn
            .prepare("SELECT id FROM events WHERE name = ?1;")
            .unwrap();
        let ID { id } =
            match id_stmt.query_row(params![&self.event], |row| Ok(ID { id: row.get(0)? })) {
                Ok(id) => id,
                Err(e) => {
                    println!("Error: {:?}", e);
                    ID { id: 0 }
                }
            };
        id
    }


    /// Update the state of the AddEvent page.
    ///
    /// # Arguments
    /// - message: AppMessage - The message to process.
    /// - day: u32 - The day of the date to add.
    /// - month: u32 - The month of the date to add.
    /// - year: i32 - The year of the date to add.
    ///
    /// # Returns
    /// - Command<AppMessage> - The command to execute.
    pub fn update(
        &mut self,
        message: AppMessage,
        day: u32,
        month: u32,
        year: i32,
    ) -> Command<AppMessage> {
        let conn = Connection::open("since_when.db").unwrap();
        self.date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap();
        match message {
            AppMessage::AddEvent => {
                println!("Adding Event {:?} on {:?}", &self.event, &self.date);
                // Add the event to the data_base.
                match Self::sql_insert(
                    self,
                    &conn,
                    (0, false),
                    ("", false),
                    (&self.event.clone(), true),
                    "INSERT INTO events (name) VALUES (?1);",
                ) {
                    Ok(_) => {
                        println!("Event added: {:?}", &self.event);
                        let id = self.get_event_id(&conn);
                        // Add the occurrence to the data_base.
                        match Self::sql_insert(
                            self,
                            &conn,
                            (id, true),
                            (&self.date.to_string(), true),
                            ("", false),
                            "INSERT INTO occurrences (event_id, date) VALUES (?1, ?2);",
                        ) {
                            Ok(_) => {
                                println!("Occurrence added: {}, {}", &self.event, &self.date);
                            }
                            Err(e) => {
                                println!("Error: {:?}", e);
                            }
                        };
                    }
                    // If the event already exists, do not add the occurrence.
                    Err(e) => {
                        println!("Error: {:?}", e);
                    }
                };
            }
            AppMessage::UpdateEvent => {
                let id = self.get_event_id(&conn);
                // Add the occurrence to the data_base.
                match Self::sql_insert(
                    self,
                    &conn,
                    (id, true),
                    (&self.date.to_string(), true),
                    ("", false),
                    "INSERT INTO occurrences (event_id, date) VALUES (?1, ?2);",
                ) {
                    Ok(_) => {
                        println!("Occurrence added: {} on {}", &self.event, &self.date);
                    }
                    Err(e) => {
                    println!("Error: {:?}", e);
                    },
                };
            }
            AppMessage::DeleteEvent => {
                let id = self.get_event_id(&conn);
                // Delete occurrence.
                match Self::sql_insert(
                    self,
                    &conn,
                    (id, true),
                    ("", false),
                    ("", false),
                    "DELETE FROM occurrences WHERE event_id = ?1;",
                ) {
                    Ok(_) => {
                        println!("Occurrences deleted.");
                        // Delete event.
                        match Self::sql_insert(
                            self,
                            &conn,
                            (0, false),
                            ("", false),
                            (&self.event.clone(), true),
                            "DELETE FROM events WHERE name = ?1;",
                        ) {
                            Ok(_) => {
                                println!("Event deleted: {}", &self.event);
                            }
                            Err(e) => {
                                println!("Error: {:?}", e);
                            }
                        }
                        id
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        0
                    }
                };
            }
            AppMessage::TextEvent(s) => {
                self.event = s;
                println!("TextEvent: {:?}", self.event);
            }
            _ => (),
        }
        Command::none()
    }

    /// View for AddEvent.
    ///
    /// # Arguments
    /// - day: u32 - The day of the date to display.
    /// - month: u32 - The month of the date to display.
    /// - year: i32 - The year of the date to display.
    ///
    /// # Returns
    /// - Element<'a, AppMessage> - The view.
    pub fn view(&self, day: u32, month: u32, year: i32) -> Element<'a, AppMessage> {
        let date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let date_text = text(date.format("%A, %B %e, %Y").to_string())
            .horizontal_alignment(Horizontal::Center)
            .size(TEXT_SIZE)
            .width(500);
        let input = text_input("Event Title", &self.event)
            .on_input(AppMessage::TextEvent)
            .size(TEXT_SIZE)
            .width(500);
        let add_button = button(
            text("Add Event")
                .size(TEXT_SIZE)
                .horizontal_alignment(Horizontal::Center)
        )
        .width(ADD_BUTTON_SIZE)
        .style(theme::Button::Secondary)
        .on_press(AppMessage::AddEvent);
        let update_button = button(
            text("Update Event")
                .size(TEXT_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(ADD_BUTTON_SIZE)
        .style(theme::Button::Secondary)
        .on_press(AppMessage::UpdateEvent);
        let delete_button = button(
            text("Delete Event")
                .size(TEXT_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(ADD_BUTTON_SIZE)
        .style(theme::Button::Secondary)
        .on_press(AppMessage::DeleteEvent);
        let add_update_row = Row::new()
            .push(add_button)
            .push(update_button)
            .push(delete_button)
            .align_items(Alignment::Center)
            .spacing(SPACING);
        let event_button = button(
            text("Events")
                .size(TEXT_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(ADD_BUTTON_SIZE)
        .style(theme::Button::Secondary)
        .on_press(AppMessage::EventsWindow);
        let calendar_button = button(
            text("Calendar")
                .size(TEXT_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(ADD_BUTTON_SIZE)
        .style(theme::Button::Secondary)
        .on_press(AppMessage::CalendarWindow);
        let button_row = Row::new()
            .push(calendar_button)
            .push(event_button)
            .align_items(Alignment::Center)
            .spacing(SPACING);
        let content = column![date_text, input, add_update_row, button_row]
            .align_items(Alignment::Center)
            .spacing(SPACING);
        content.into()
    }
}
