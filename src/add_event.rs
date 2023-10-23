use iced::alignment::Horizontal;
use iced::widget::{column, row, text, text_input};
use iced::{Alignment, Command, Element};
use log::info;

use crate::{
    app::AppMessage,
    database::{add_event, delete_event, update_event},
    settings::Settings,
    utils::{get_date, new_button},
};

/// AddEvent state.
#[derive(Debug, Clone)]
pub struct AddEvent {
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
            event: String::new(),
        }
    }

    /// Add, Update or Delete Events.
    ///
    /// ### Arguments
    /// - message: `AppMessage` - The message to process.
    /// - day: `u32` - The day of the date to add.
    /// - month: `u32` - The month of the date to add.
    /// - year: `i32` - The year of the date to add.
    ///
    /// ### Returns
    /// - `Command<AppMessage>` - The command to execute.
    pub fn update(
        &mut self,
        message: AppMessage,
        day: u32,
        month: u32,
        year: i32,
    ) -> Command<AppMessage> {
        match message {
            AppMessage::AddEvent => {
                if self.event.is_empty() {
                    return Command::none();
                }
                add_event(&self.event, year, month, day);
            }
            AppMessage::UpdateEvent => {
                if self.event.is_empty() {
                    return Command::none();
                }
                update_event(&self.event, year, month, day);
            }
            AppMessage::DeleteEvent => {
                if self.event.is_empty() {
                    return Command::none();
                }
                delete_event(&self.event);
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
    /// ### Arguments
    /// - day: `u32` - The day of the date to display.
    /// - month: `u32` - The month of the date to display.
    /// - year: `i32` - The year of the date to display.
    ///
    /// ### Returns
    /// - `Element<'a, AppMessage>` - The AddEvent page.
    pub fn view(&self, day: u32, month: u32, year: i32) -> Element<'a, AppMessage> {
        let settings = Settings::new();
        // Date and event input.
        let date = get_date(year, month, day);
        let date_text = text(date.format("%A, %B %e, %Y").to_string())
            .horizontal_alignment(Horizontal::Center)
            .size(settings.text_size())
            .width(500);
        let input = text_input("Event Title", &self.event)
            .on_input(AppMessage::TextEvent)
            .size(settings.text_size())
            .width(500);
        // Action buttons.
        let add_button = new_button(
            AppMessage::AddEvent,
            text("Add Event"),
            settings.add_button_size(),
        );
        let update_button = new_button(
            AppMessage::UpdateEvent,
            text("Update Event"),
            settings.add_button_size(),
        );
        let delete_button = new_button(
            AppMessage::DeleteEvent,
            text("Delete Event"),
            settings.add_button_size(),
        );
        let action_row = row![add_button, update_button, delete_button]
            .align_items(Alignment::Center)
            .spacing(settings.spacing());
        // Navigation buttons.
        let event_button = new_button(
            AppMessage::EventsWindow,
            text("Events"),
            settings.add_button_size(),
        );
        let calendar_button = new_button(
            AppMessage::CalendarWindow,
            text("Calendar"),
            settings.add_button_size(),
        );
        let nav_row = row![calendar_button, event_button]
            .align_items(Alignment::Center)
            .spacing(settings.spacing());
        let content = column![date_text, input, action_row, nav_row]
            .align_items(Alignment::Center)
            .spacing(settings.spacing());
        content.into()
    }
}
