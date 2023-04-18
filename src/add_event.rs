use crate::AppMessage;
use iced::alignment::Horizontal;
use iced::widget::{button, column, text, text_input, Row};
use iced::{theme, Alignment, Command, Element};
use rusqlite::params;

const TEXT_SIZE: u16 = 40;
const SPACING: u16 = 20;
const ADD_BUTTON_SIZE: u16 = 225;

// AddEvent state.
#[derive(Debug, Clone)]
pub struct AddEvent {
    date: chrono::NaiveDate,
    event: String,
}

// AddEvent implementation.
impl<'a> AddEvent {
    pub fn new() -> AddEvent {
        Self {
            date: chrono::NaiveDate::from_ymd_opt(1, 3, 3).unwrap(),
            event: String::new(),
        }
    }

    pub fn update(
        &mut self,
        message: AppMessage,
        day: u32,
        month: u32,
        year: i32,
    ) -> Command<AppMessage> {
        let conn = rusqlite::Connection::open("since_when.db").unwrap();
        self.date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap();
        match message {
            AppMessage::AddEvent => {
                println!("Add Event {:?} on {:?}", &self.event, &self.date);
                // Add the event to the database.
                let mut stmt = conn
                    .prepare("INSERT INTO events (name) VALUES (?1);")
                    .unwrap();
                match stmt.execute([&self.event]) {
                    Ok(id) => {
                        println!("Event added: {}", id);
                        // Get the id of the event we just added.
                        let mut id_stmt = conn
                            .prepare("SELECT id FROM events WHERE name = ?1;")
                            .unwrap();
                        let id: i32 = id_stmt
                            .query_row(params![&self.event], |row| row.get(0))
                            .unwrap();
                        // Add the occurrence to the database.
                        let mut occur_stmt = conn
                            .prepare("INSERT INTO occurrences (event_id, date) VALUES (?1, ?2);")
                            .unwrap();
                        match occur_stmt.execute(params![id, self.date.to_string(),]) {
                            Ok(id) => {
                                println!("Occurrence added: {}", id);
                                id
                            }
                            Err(e) => {
                                println!("Error: {:?}", e);
                                0
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
                let mut id_stmt = conn
                    .prepare("SELECT id FROM events WHERE name = ?1;")
                    .unwrap();
                let id: i32 = id_stmt
                    .query_row(params![&self.event], |row| row.get(0))
                    .unwrap();
                // Add the occurrence to the database.
                let mut occur_stmt = conn
                    .prepare("INSERT INTO occurrences (event_id, date) VALUES (?1, ?2);")
                    .unwrap();
                match occur_stmt.execute(params![id, self.date.to_string(),]) {
                    Ok(id) => {
                        println!("Occurrence added: {}", id);
                        id
                    }
                    Err(e) => {
                        println!("Error: {:?}", e);
                        0
                    }
                };
            }
            AppMessage::DeleteEvent => {
                // Get id of event.
                let mut id_stmt = conn
                    .prepare("SELECT id FROM events WHERE name = ?1;")
                    .unwrap();
                let id: i32 = id_stmt
                    .query_row(params![&self.event], |row| row.get(0))
                    .unwrap();
                // Delete occurrence.
                match conn.execute("DELETE FROM occurrences WHERE event_id = ?1;", params![id]) {
                    Ok(id) => {
                        println!("Occurrence deleted: {}", id);
                        match conn.execute("DELETE FROM events WHERE name = ?1;", params![&self.event]) {
                            Ok(id) => {
                                println!("Event deleted: {}", id);
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
    // View for AddEvent.
    pub fn view(&self, day: u32, month: u32, year: i32) -> Element<'a, AppMessage> {
        let date = chrono::NaiveDate::from_ymd_opt(year, month, day).unwrap();
        let date_text = text(date.format("%A, %B %e, %Y").to_string())
            .horizontal_alignment(Horizontal::Center)
            .size(TEXT_SIZE)
            .width(500);
        let input = text_input("Event Title", &self.event)
            .on_input(AppMessage::TextEvent)
            // .on_submit(AppMessage::AddEvent)
            .size(TEXT_SIZE)
            .width(500);
        let add_button = button(
            text("Add Event")
                .size(TEXT_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(ADD_BUTTON_SIZE)
        .style(theme::Button::Primary)
        .on_press(AppMessage::AddEvent);
        let update_button = button(
            text("Update Event")
                .size(TEXT_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(ADD_BUTTON_SIZE)
        .style(theme::Button::Primary)
        .on_press(AppMessage::UpdateEvent);
        let delete_button = button(
            text("Delete Event")
                .size(TEXT_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(ADD_BUTTON_SIZE)
        .style(theme::Button::Primary)
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
        .style(theme::Button::Primary)
        .on_press(AppMessage::EventsWindow);
        let calendar_button = button(
            text("Calendar")
                .size(TEXT_SIZE)
                .horizontal_alignment(Horizontal::Center),
        )
        .width(ADD_BUTTON_SIZE)
        .style(theme::Button::Primary)
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
