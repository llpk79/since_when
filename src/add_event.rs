use crate::{database, utils, settings::Settings, app::AppMessage};
use chrono::NaiveDate;
use iced::alignment::Horizontal;
use iced::widget::{button, column, text, text_input, Row};
use iced::{theme, Alignment, Command, Element};
use log::{info, error};

/// AddEvent state.
#[derive(Debug, Clone)]
pub struct AddEvent {
    date: NaiveDate,
    event: String,
}

/// Default AddEvent implementation.
impl Default for AddEvent {
    fn default() -> Self {
        AddEvent::new()
    }
}

/// AddEvent implementation.
impl<'a> AddEvent {
    pub fn new() -> AddEvent {
        Self {
            date: match NaiveDate::from_ymd_opt(1, 2, 3) {
                Some(date) => date,
                None => {
                    panic!("Error creating date");
                }
            },
            event: String::new(),
        }
    }

    /// Update the state of the AddEvent page.
    ///
    /// # Arguments
    /// - message: `AppMessage` - The message to process.
    /// - day: `u32` - The day of the date to add.
    /// - month: `u32` - The month of the date to add.
    /// - year: `i32` - The year of the date to add.
    ///
    /// # Returns
    /// - `Command<AppMessage>` - The command to execute.
    pub fn update(
        &mut self,
        message: AppMessage,
        day: u32,
        month: u32,
        year: i32,
    ) -> Command<AppMessage> {
        let conn = database::setup_connection();
        self.date = utils::get_date(year, month, day);
        match message {
            AppMessage::AddEvent => {
                info!("Adding Event {:?} on {:?}", &self.event, &self.date);
                // Add the event to the data_base.
                if self.event.is_empty() {
                    return Command::none();
                }
                match database::sql_insert(
                    &conn,
                    (0, false),
                    ("", false),
                    (&self.event.clone(), true),
                    "INSERT INTO events (name) VALUES (?1);",
                ) {
                    Ok(_) => {
                        info!("Event added: {:?}", &self.event);
                        let id = database::get_event_id(&conn, &self.event);
                        // Add the occurrence to the data_base.
                        match database::sql_insert(
                            &conn,
                            (id, true),
                            (&self.date.to_string(), true),
                            ("", false),
                            "INSERT INTO occurrences (event_id, date) VALUES (?1, ?2);",
                        ) {
                            Ok(_) => {
                                info!("Occurrence added: {}, {}", &self.event, &self.date);
                            }
                            Err(e) => {
                                error!("Error: {:?}", e);
                            }
                        };
                    }
                    // If the event already exists, do not add the occurrence.
                    Err(e) => {
                        error!("Error: {:?}", e);
                    }
                };
            }
            AppMessage::UpdateEvent => {
                let id = database::get_event_id(&conn, &self.event);
                // Add the occurrence to the data_base.
                match database::sql_insert(
                    &conn,
                    (id, true),
                    (&self.date.to_string(), true),
                    ("", false),
                    "INSERT INTO occurrences (event_id, date) VALUES (?1, ?2);",
                ) {
                    Ok(_) => {
                        info!("Occurrence added: {} on {}", &self.event, &self.date);
                    }
                    Err(e) => {
                        error!("Error: {:?}", e);
                    }
                };
            }
            AppMessage::DeleteEvent => {
                let id = database::get_event_id(&conn, &self.event);
                // Delete occurrence.
                match database::sql_insert(
                    &conn,
                    (id, true),
                    ("", false),
                    ("", false),
                    "DELETE FROM occurrences WHERE event_id = ?1;",
                ) {
                    Ok(_) => {
                        info!("Occurrences deleted.");
                        // Delete event.
                        match database::sql_insert(
                            &conn,
                            (0, false),
                            ("", false),
                            (&self.event.clone(), true),
                            "DELETE FROM events WHERE name = ?1;",
                        ) {
                            Ok(_) => {
                                info!("Event deleted: {}", &self.event);
                            }
                            Err(e) => {
                                error!("Error: {:?}", e);
                            }
                        }
                        id
                    }
                    Err(e) => {
                        error!("Error: {:?}", e);
                        0
                    }
                };
            }
            AppMessage::TextEvent(s) => {
                self.event = s;
                info!("TextEvent: {:?}", self.event);
            }
            _ => (),
        }
        Command::none()
    }

    /// View for AddEvent.
    ///
    /// # Arguments
    /// - day: `u32` - The day of the date to display.
    /// - month: `u32` - The month of the date to display.
    /// - year: `i32` - The year of the date to display.
    ///
    /// # Returns
    /// - `Element<'a, AppMessage>` - The view.
    pub fn view(&self, day: u32, month: u32, year: i32) -> Element<'a, AppMessage> {
        let settings = Settings::new();
        let date = utils::get_date(year, month, day);
        let date_text = text(date.format("%A, %B %e, %Y").to_string())
            .horizontal_alignment(Horizontal::Center)
            .size(settings.text_size())
            .width(500);
        let input = text_input("Event Title", &self.event)
            .on_input(AppMessage::TextEvent)
            .size(settings.text_size())
            .width(500);
        let add_button = button(
            text("Add Event")
                .size(settings.text_size())
                .horizontal_alignment(Horizontal::Center),
        )
        .width(settings.add_button_size())
        .style(theme::Button::Secondary)
        .on_press(AppMessage::AddEvent);
        let update_button = button(
            text("Update Event")
                .size(settings.text_size())
                .horizontal_alignment(Horizontal::Center),
        )
        .width(settings.add_button_size())
        .style(theme::Button::Secondary)
        .on_press(AppMessage::UpdateEvent);
        let delete_button = button(
            text("Delete Event")
                .size(settings.text_size())
                .horizontal_alignment(Horizontal::Center),
        )
        .width(settings.add_button_size())
        .style(theme::Button::Secondary)
        .on_press(AppMessage::DeleteEvent);
        let add_update_row = Row::new()
            .push(add_button)
            .push(update_button)
            .push(delete_button)
            .align_items(Alignment::Center)
            .spacing(settings.spacing());
        let event_button = button(
            text("Events")
                .size(settings.text_size())
                .horizontal_alignment(Horizontal::Center),
        )
        .width(settings.add_button_size())
        .style(theme::Button::Secondary)
        .on_press(AppMessage::EventsWindow);
        let calendar_button = button(
            text("Calendar")
                .size(settings.text_size())
                .horizontal_alignment(Horizontal::Center),
        )
        .width(settings.add_button_size())
        .style(theme::Button::Secondary)
        .on_press(AppMessage::CalendarWindow);
        let button_row = Row::new()
            .push(calendar_button)
            .push(event_button)
            .align_items(Alignment::Center)
            .spacing(settings.spacing());
        let content = column![date_text, input, add_update_row, button_row]
            .align_items(Alignment::Center)
            .spacing(settings.spacing());
        content.into()
    }
}
