use crate::{app::AppMessage, database, settings::Settings, utils};
use iced::alignment::Horizontal;
use iced::widget::{button, text, vertical_space, Column, Row, Text};
use iced::Alignment;
use iced::Element;
use log::error;

/// Event state.
#[derive(Debug, Clone)]
pub struct EventOccurrence {
    pub name: String,
    pub date: String,
}

/// Events page struct.
#[derive(Debug, Clone)]
pub struct EventsPage {}

///Events page implementation.
impl<'a> EventsPage {
    pub fn new() -> EventsPage {
        Self {}
    }

    /// View the events page.
    ///
    /// # Arguments
    /// - `&self`
    ///
    /// # Returns
    /// - `Element<'a, AppMessage>`
    pub fn view(&self) -> Element<'a, AppMessage> {
        let settings = Settings::new();
        // Open the data_base.
        let conn = database::setup_connection();
        // Get the events.
        let events = match database::get_events(&conn) {
            Ok(events) => events,
            Err(e) => {
                error!("Error: {}", e);
                vec![]
            }
        };
        // Calculate the days since each event.
        let days_since_now = utils::get_days_since_now(&events);
        // Calculate the elapsed days between event occurrences.
        let elapsed = utils::get_elapsed_days(&days_since_now);
        // Calculate the average elapsed days between occurrences.
        let averages = utils::get_averages(elapsed);
        // Create the columns.
        let mut event_column = Column::new()
            .spacing(settings.spacing())
            .width(333)
            .align_items(Alignment::Center);
        let mut date_column = Column::new()
            .spacing(settings.spacing())
            .width(333)
            .align_items(Alignment::Center);
        let mut avg_column = Column::new()
            .spacing(settings.spacing())
            .width(333)
            .align_items(Alignment::Center);

        // Create the column headers.
        let event_header = text("Event").size(settings.text_size());
        let event_sep = text("_".repeat(34)).size(settings.text_size() / 4);
        let date_header = text("Days  Since").size(settings.text_size());
        let date_sep = text("_".repeat(56)).size(settings.text_size() / 4);
        let avg_header = text("Avg").size(settings.text_size());
        let avg_sep = text("_".repeat(24)).size(settings.text_size() / 4);
        event_column = event_column.push(event_header);
        event_column = event_column.push(event_sep);
        date_column = date_column.push(date_header);
        date_column = date_column.push(date_sep);
        avg_column = avg_column.push(avg_header);
        avg_column = avg_column.push(avg_sep);

        // // Sort the events by days since.
        let mut sorted_events = Vec::new();
        for event in days_since_now.iter() {
            sorted_events.push((event.0.clone(), event.1[0]));
        }
        sorted_events.sort_by(|a, b| a.1.cmp(&b.1));

        // Create the event rows.
        for event in sorted_events.iter() {
            let days = event.1;
            let mut plural = String::new();
            if days != 1 {
                plural = String::from("s");
            }
            if averages.contains_key(&event.0.clone()) {
                let average = averages.get(&event.0.clone()).unwrap_or(&0);
                let plural = if average > &1 { "s" } else { "" };
                let average_text =
                    Text::new(format!("{} day{}", average, plural)).size(settings.text_size());
                avg_column = avg_column.push(average_text);
            } else {
                let average_text = Text::new("---").size(settings.text_size());
                avg_column = avg_column.push(average_text);
            }
            let event_text = Text::new(event.0.clone())
                .size(settings.text_size())
                .horizontal_alignment(Horizontal::Center);
            let date_text =
                Text::new(format!("{} day{} ago", days, plural)).size(settings.text_size());
            event_column = event_column.push(event_text);
            date_column = date_column.push(date_text);
        }
        // Layout the buttons and text.
        let calendar_button = button(text("Add/Update Event").size(settings.text_size()))
            .padding(settings.padding())
            .on_press(AppMessage::CalendarWindow)
            .style(iced::theme::Button::Secondary);
        let button_row = Row::new()
            .push(calendar_button)
            .align_items(Alignment::Center);
        let event_row = Row::new()
            .push(event_column)
            .push(date_column)
            .push(avg_column)
            .spacing(settings.spacing())
            .align_items(Alignment::Center);
        let height = 20 * events.len() as u16;
        let content = Column::new()
            .push(vertical_space(50))
            .push(event_row)
            .push(button_row)
            .push(vertical_space(height))
            .align_items(Alignment::Center)
            .spacing(settings.spacing() + 40);
        content.into()
    }
}
