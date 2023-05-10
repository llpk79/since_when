use std::collections::HashMap;

use chrono::NaiveDate;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, text, Button, Row};
use iced::{theme, Renderer};
use log::error;

use crate::app::AppMessage;
use crate::database::{setup_connection, get_events};
use crate::events::EventOccurrence;
use crate::settings::Settings;

/// Get the date from the day, month, and year.
///
/// ### Arguments
/// - year: `i32`
/// - month: `u32`
/// - day: `u32`
///
/// ### Returns
/// - `NaiveDate`
pub fn get_date(year: i32, month: u32, day: u32) -> NaiveDate {
    match NaiveDate::from_ymd_opt(year, month, day) {
        Some(date) => date,
        None => {
            panic!("Invalid date");
        }
    }
}

/// Get the days since today for each occurrence for each event.
///
/// ### Arguments
/// - events - `&[EventOccurrence]` - The events to process.
///
/// ### Returns
/// - `HashMap<String, Vec<i32>>` - The days since today for each occurrence for each event.
pub fn get_days_since_now(events: &[EventOccurrence]) -> HashMap<String, Vec<i32>> {
    let mut days_since_now: HashMap<String, Vec<i32>> = HashMap::new();
    let now = chrono::Local::now().naive_local().date();
    for event in events.iter() {
        // Calculate the days between the events and the current date.
        let date = match NaiveDate::parse_from_str(&event.date, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                error!("Error parsing date");
                continue;
            }
        };
        let days = now.signed_duration_since(date).num_days() as i32;
        if days_since_now.contains_key(&event.name) {
            let days_vec = match days_since_now.get_mut(&event.name) {
                Some(days_vec) => days_vec,
                None => {
                    error!("Error getting days vector");
                    continue;
                }
            };
            days_vec.push(days);
        } else {
            days_since_now.insert(event.name.clone(), vec![days]);
        };
    }
    days_since_now
}

/// Get the elapsed days between each occurrence for each event.
///
/// ### Arguments
/// - days_since - `&HashMap<String, Vec<i32>>` - The days since today for each occurrence for each event.
///
/// ### Returns
/// - `HashMap<String, Vec<i32>>` - The elapsed days between each occurrence for each event.
///
/// ### Example
/// ```
/// # use std::collections::HashMap;
/// # use since_when_lib::utils::get_elapsed_days;
/// let mut days_since = HashMap::new();
/// let times_0 = vec![1, 11, 22, 33, 44];
/// let times_1 = vec![1, 6, 8, 16, 20];
/// days_since.insert("event_0".to_string(), times_0);
/// days_since.insert("event_1".to_string(), times_1);
///
/// let mut expected = HashMap::new();
/// let expected_vec_0 = vec![10, 11, 11, 11];
/// let expected_vec_1 = vec![5, 2, 8, 4];
/// expected.insert("event_0".to_string(), expected_vec_0);
/// expected.insert("event_1".to_string(), expected_vec_1);
///
/// assert_eq!(get_elapsed_days(&days_since), expected);
/// ```
pub fn get_elapsed_days(days_since: &HashMap<String, Vec<i32>>) -> HashMap<String, Vec<i32>> {
    let mut elapsed: HashMap<String, Vec<i32>> = HashMap::new();
    for items in days_since.iter() {
        let mut days_vec: Vec<i32> = Vec::new();
        if items.1.len() > 1 {
            for item in 1..items.1.len() {
                let days = items.1[item] - items.1[item - 1];
                days_vec.push(days);
            }
            elapsed.entry(items.0.clone()).or_insert(days_vec);
        }
    }
    elapsed
}

/// Get the average elapsed days between occurrences for each event.
///
/// ### Arguments
/// elapsed - `&HashMap<String, Vec<i32>>` - The elapsed days between occurrences for each event.
///
/// ### Returns
/// `HashMap<String, i32>` - The average elapsed days between occurrences for each event.
///
/// ### Example
/// ```
/// # use std::collections::HashMap;
/// # use since_when_lib::utils::get_averages;
/// let mut averages = HashMap::new();
/// let times_0 = vec![1, 11, 22, 33, 44];
/// let times_1 = vec![1, 6, 11, 16, 21];
/// averages.entry("foo".to_string()).or_insert(times_0);
/// averages.entry("bar".to_string()).or_insert(times_1);
///
/// let mut expected = HashMap::new();
/// expected.entry("foo".to_string()).or_insert(22);
/// expected.entry("bar".to_string()).or_insert(11);
///
/// assert_eq!(get_averages(&averages), expected);
/// ```
pub fn get_averages(elapsed: &HashMap<String, Vec<i32>>) -> HashMap<String, i32> {
    let mut averages: HashMap<String, i32> = HashMap::new();
    for item in elapsed.iter() {
        let average = item.1.iter().sum::<i32>() / item.1.len() as i32;
        averages.entry(item.0.to_string()).or_insert(average);
    }
    averages
}

/// Sort events by days since now.
///
/// ### Arguments
/// - events - `&HashMap<String, Vec<i32>>` - The days since now for each event.
/// - averages - `&HashMap<String, i32>` - The average elapsed days between occurrences for each event.
///
/// ### Returns
/// - `(String, i32, i32)` - (event_name, days_since, average)
///
/// ### Example
/// ```
/// # use std::collections::HashMap;
/// # use since_when_lib::utils::sort_events;
/// let mut events = HashMap::new();
/// let times_0 = vec![4, 11, 22, 33, 44];
/// let times_1 = vec![1, 6, 11, 16, 21];
/// events.entry("foo".to_string()).or_insert(times_0);
/// events.entry("bar".to_string()).or_insert(times_1);
///
/// let mut averages = HashMap::new();
/// averages.entry("foo".to_string()).or_insert(22);
/// averages.entry("bar".to_string()).or_insert(11);
///
/// let mut expected = Vec::new();
/// expected.push(("bar".to_string(), 1, 11));
/// expected.push(("foo".to_string(), 4, 22));
///
/// assert_eq!(sort_events(&events, &averages), expected);
/// ```
pub fn sort_events(
    events: &HashMap<String, Vec<i32>>,
    averages: &HashMap<String, i32>,
) -> Vec<(String, i32, i32)> {
    let mut sorted_events = Vec::new();
    for event in events.iter() {
        sorted_events.push((
            event.0.clone(),
            event.1[0],
            *averages.get(&event.0.clone()).unwrap_or(&0),
        ));
    }
    sorted_events.sort_by(|a, b| a.1.cmp(&b.1));
    sorted_events
}

/// Get the event details sorted by days since.
///
/// ### Returns
/// - `Vec<(String, i32, i32)>` - A vector of tuples containing the event name, days since, and average elapsed days.
pub fn event_details() -> Vec<(String, i32, i32)> {
    // Open the data_base.
    let conn = setup_connection();
    // Get the events.
    let events = match get_events(&conn) {
        Ok(events) => events,
        Err(e) => {
            error!("Error: {}", e);
            vec![]
        }
    };
    // Calculate the days since each event.
    let days_since_now = get_days_since_now(&events);
    // Calculate the elapsed days between event occurrences.
    let elapsed = get_elapsed_days(&days_since_now);
    // Calculate the average elapsed days between occurrences.
    let averages = get_averages(&elapsed);
    // Sort the events by days since.
    sort_events(&days_since_now, &averages)
}

/// Make a new button.
///
/// ### Arguments
/// - message: `AppMessage` - The message to send when the button is pressed.
/// - label: `&str` - The label to display on the button.
/// - width: `u16` - The width of the button.
///
/// ### Returns
/// - `Button<'a, AppMessage, Renderer>` - The button.
pub fn new_button<'a>(
    message: AppMessage,
    label: &str,
    width: u16,
) -> Button<'a, AppMessage, Renderer> {
    let settings = Settings::new();
    button(
        text(label)
            .size(settings.text_size())
            .horizontal_alignment(Horizontal::Center),
    )
    .width(width)
    .style(theme::Button::Secondary)
    .on_press(message)
}

/// Creates a new row.
///
/// ### Returns
/// - `Row<'static, AppMessage, Renderer>`
pub fn make_new_row() -> Row<'static, AppMessage, Renderer> {
    let settings = Settings::new();
    Row::new()
        .spacing(settings.spacing())
        .align_items(Vertical::Top.into())
}
