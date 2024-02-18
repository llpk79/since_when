use iced::alignment::Horizontal;
use iced::widget::{row, text, Column, Text};
use iced::Alignment;

use crate::{app::AppMessage, settings::Settings, utils};

/// Event state.
#[derive(Debug, Clone)]
pub struct EventOccurrence {
    pub name: String,
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

/// Events page struct.
#[derive(Debug, Clone)]
pub struct EventsPage {}

/// Default EventsPage implementation.
impl Default for EventsPage {
    fn default() -> Self {
        EventsPage::new()
    }
}

///Events page implementation.
impl<'a> EventsPage {
    pub fn new() -> EventsPage {
        Self {}
    }

    /// Create columns with header for events page.
    ///
    /// ### Arguments
    /// - label: `&str` - The label for the column.
    ///
    /// ### Returns
    /// - `Column<AppMessage>` - The column with header.
    fn make_column(label: &str) -> Column<AppMessage> {
        let settings = Settings::new();
        let mut column = Column::new()
            .spacing(settings.spacing())
            .width(333)
            .align_items(Alignment::Center);
        let header = text(label).size(settings.text_size());
        let sep = text("_".repeat((label.len() * 5) + 5)).size(settings.text_size() / 4);
        column = column.push(header);
        column = column.push(sep);
        column
    }

    /// Create the event columns.
    ///
    /// ### Returns
    /// - (`Column<'a, AppMessage>`, `Column<'a, AppMessage>`, `Column<'a, AppMessage>`, u16)
    /// - The event column, date column, average column, and number of events.
    fn event_columns() -> (
        Column<'a, AppMessage>,
        Column<'a, AppMessage>,
        Column<'a, AppMessage>,
    ) {
        let settings = Settings::new();
        // Create the columns.
        let mut event_column = Self::make_column("Event");
        let mut days_since_column = Self::make_column("Days  Since");
        let mut avg_column = Self::make_column("Avg");
        // Create the event rows.
        // event_details is a vector of tuples (event_name, days_since, average).
        for (name, days_since, avg) in utils::event_details().iter() {
            // Text for the event name.
            let event_text = Text::new(name.clone())
                .size(settings.text_size())
                .horizontal_alignment(Horizontal::Center);
            event_column = event_column.push(event_text);
            // Text for the days since.
            let plural = if *days_since != 1 { "s" } else { "" };
            let days_since_text =
                Text::new(format!("{} day{} ago", days_since, plural)).size(settings.text_size());
            days_since_column = days_since_column.push(days_since_text);
            // Text for the average.
            if *avg != 0 {
                let plural = if *avg > 1 { "s" } else { "" };
                let average_text =
                    Text::new(format!("{} day{}", avg, plural)).size(settings.text_size());
                avg_column = avg_column.push(average_text);
            } else {
                let average_text = Text::new("---").size(settings.text_size());
                avg_column = avg_column.push(average_text);
            }
        }
        (event_column, days_since_column, avg_column)
    }

    /// View the events page.
    ///
    /// ### Arguments
    /// - `&self`
    ///
    /// ### Returns
    /// - `Element<'a, AppMessage>` - The events page.
    pub fn view(&self) -> Column<'a, AppMessage> {
        let settings = Settings::new();
        // Get the event details and create the columns.
        let (event_column, days_since_column, avg_column) = Self::event_columns();
        // Align the columns into a row.
        let event_row = row![event_column, days_since_column, avg_column]
            .spacing(settings.spacing())
            .align_items(Alignment::Center);
        // Button for adding/updating events.
        let calendar_button = utils::new_button(
            AppMessage::CalendarWindow,
            text("Add/Update Event"),
            settings.add_button_size() + 100,
        );
        // Arrange the content.
        let content = Column::new()
            .push(event_row)
            .push(calendar_button)
            .align_items(Alignment::Center)
            .spacing(settings.spacing() + 40);
        content
    }
}
