use crate::AppMessage;
use chrono::NaiveDate;
use iced::alignment::Horizontal;
use iced::widget::{button, text, vertical_space, Column, Row, Text};
use iced::Alignment;
use iced::Element;
use rusqlite::{Connection, Result};
use std::collections::HashMap;

const TEXT_SIZE: u16 = 50;
const SPACING: u16 = 20;
const PADDING: u16 = 5;

/// Event state.
#[derive(Debug, Clone)]
pub struct EventOccurrence {
    name: String,
    date: String,
}

/// Get events from the database.
pub fn get_events(conn: &Connection) -> Result<Vec<EventOccurrence>> {
    println!("Retrieving Records.");
    // Get all events and occurrences.
    let mut stmt = conn.prepare(
        "\
        SELECT name, date \
        FROM events \
        JOIN occurrences \
        ON events.id = occurrences.event_id \
        ORDER BY date DESC;",
    )?;
    let event_iter = stmt.query_map([], |row| {
        Ok(EventOccurrence {
            name: row.get(0)?,
            date: row.get(1)?,
        })
    })?;
    let mut events = Vec::new();
    for event in event_iter {
        events.push(event.unwrap());
    }
    Ok(events)
}

/// Events page struct.
#[derive(Debug, Clone)]
pub struct EventsPage {}

///Events page implementation.
impl<'a> EventsPage {
    pub fn new() -> EventsPage {
        Self {}
    }

    /// Get the days since today for each occurrence for each event.
    pub fn get_days_since_now(events: &[EventOccurrence]) -> HashMap<String, Vec<i32>> {
        let mut days_since_now: HashMap<String, Vec<i32>> = HashMap::new();
        for event in events.iter() {
            // Calculate the days between the events and the current date.
            let date = NaiveDate::parse_from_str(&event.date, "%Y-%m-%d").unwrap();
            let now = chrono::Local::now().naive_local().date();
            let days = now.signed_duration_since(date).num_days() as i32;
            println!("event: {:?}, days {}", &event.name, days);
            if days_since_now.contains_key(&event.name) {
                let days_vec = days_since_now.get_mut(&event.name).unwrap();
                days_vec.push(days);
            } else {
                days_since_now.insert(event.name.clone(), vec![days]);
            };
        }
        days_since_now
    }

    /// Get the elapsed days between each occurrence for each event.
    pub fn get_elapsed_days(days_since: &HashMap<String, Vec<i32>>) -> HashMap<String, Vec<i32>> {
        let mut elapsed: HashMap<String, Vec<i32>> = HashMap::new();
        for item in days_since.iter() {
            if item.1.len() > 1 {
                for i in 1..item.1.len() {
                    let days = item.1[i] - item.1[i - 1];
                    if let std::collections::hash_map::Entry::Vacant(e) =
                        elapsed.entry(item.0.clone())
                    {
                        e.insert(vec![days]);
                    } else {
                        let days_vec = elapsed.get_mut(&item.0.clone()).unwrap();
                        days_vec.push(days);
                    };
                }
            }
        }
        elapsed
    }

    /// Get the average elapsed days between occurrences for each event.
    pub fn get_averages(elapsed: HashMap<String, Vec<i32>>) -> HashMap<String, Vec<i32>> {
        let mut averages: HashMap<String, Vec<i32>> = HashMap::new();
        for item in elapsed.iter() {
            let mut sum = 0;
            for i in item.1.iter() {
                sum += i;
            }
            let average = sum / item.1.len() as i32;
            if let std::collections::hash_map::Entry::Vacant(e) = averages.entry(item.0.clone()) {
                e.insert(vec![average]);
            } else {
                let average_vec = averages.get_mut(&item.0.clone()).unwrap();
                average_vec.push(average);
            };
        }
        averages
    }

    /// View the events page.
    pub fn view(&self) -> Element<'a, AppMessage> {
        // Open the database.
        let conn = Connection::open("since_when.db").unwrap();
        // Get the events.
        let events = get_events(&conn).unwrap();
        // Calculate the days since each event.
        let days_since_now = Self::get_days_since_now(&events);
        // Calculate the elapsed days between event occurrences.
        let elapsed = Self::get_elapsed_days(&days_since_now);
        // Calculate the average elapsed days between occurrences.
        let averages = Self::get_averages(elapsed);
        // Create the columns.
        let mut event_column = Column::new()
            .spacing(SPACING)
            .width(333)
            .align_items(Alignment::End);
        let mut date_column = Column::new()
            .spacing(SPACING)
            .width(333)
            .align_items(Alignment::Center);
        let mut avg_column = Column::new()
            .spacing(SPACING)
            .width(333)
            .align_items(Alignment::Center);

        // Create the column headers.
        let event_text = text("Events").size(TEXT_SIZE);
        let date_text = text("Days Since").size(TEXT_SIZE);
        let avg_text = text("Average Days").size(TEXT_SIZE);
        event_column = event_column.push(event_text);
        date_column = date_column.push(date_text);
        avg_column = avg_column.push(avg_text);

        // Create the event rows.
        for days_since in days_since_now.iter() {
            let mut plural = String::new();
            let days = days_since.1[0];
            if days > 1 {
                plural = String::from("s");
            }
            if averages.contains_key(&days_since.0.clone()) {
                let average = averages.get(&days_since.0.clone()).unwrap()[0];
                let plural = if average > 1 { "s" } else { "" };
                let average_text = format!("{} day{} avg", average, plural);
                let average_text = Text::new(average_text).size(TEXT_SIZE);
                avg_column = avg_column.push(average_text);
            } else {
                let average_text = Text::new(format!("{} day{} avg", days, plural)).size(TEXT_SIZE);
                avg_column = avg_column.push(average_text);
            }
            let event_button = Text::new(days_since.0.clone())
                .size(TEXT_SIZE)
                .horizontal_alignment(Horizontal::Left)
                .style(iced::Color::from_rgb8(150, 0, 200));
            let days_text = format!("{} day{} ago", days, plural);
            let row_date = Text::new(days_text).size(TEXT_SIZE);
            event_column = event_column.push(event_button);
            date_column = date_column.push(row_date);
        }
        // Layout the buttons and text.
        let calendar_button = button(text("Add/Update Event").size(TEXT_SIZE))
            .padding(PADDING)
            .on_press(AppMessage::CalendarWindow);
        let button_row = Row::new()
            .push(calendar_button)
            .align_items(Alignment::Center);
        let event_row = Row::new()
            .push(event_column)
            .push(date_column)
            .push(avg_column)
            .spacing(SPACING)
            .align_items(Alignment::Center);
        let height = 20 * events.len() as u16;
        let content = Column::new()
            .push(vertical_space(50))
            .push(event_row)
            .push(button_row)
            .push(vertical_space(height))
            .align_items(Alignment::Center)
            .spacing(SPACING);
        content.into()
    }
}
