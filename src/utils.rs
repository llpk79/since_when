use chrono::NaiveDate;
use iced::alignment::{Horizontal, Vertical};
use iced::theme::Button::Secondary;
use iced::widget::{button, text, Button, Row};
use log::error;
use std::collections::HashMap;

use crate::app::AppMessage;
use crate::database::{get_events, setup_connection};
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
        let date = get_date(event.year, event.month, event.day);
        let days = now.signed_duration_since(date).num_days() as i32;
        match days_since_now.get_mut(&event.name) {
            Some(days_vec) => {
                days_vec.push(days);
                days_vec.sort();
            }
            None => {
                days_since_now.insert(event.name.clone(), vec![days]);
            }
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
    return days_since
        .iter()
        .map(|(name, days)| {
            (
                name.to_owned(),
                days.windows(2).map(|w| w[1] - w[0]).collect(),
            )
        })
        .collect();
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
    return elapsed
        .iter()
        .map(|(name, days)| {
            (
                name.to_owned(),
                if !days.is_empty() {
                    days.iter().sum::<i32>() / days.len() as i32
                } else {
                    0
                },
            )
        })
        .collect();
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
    let mut sorted_events: Vec<(String, i32, i32)> = events
        .iter()
        .map(|(name, days)| (name.to_owned(), days[0], *averages.get(name).unwrap_or(&0)))
        .collect();
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
    let events = get_events(&conn).unwrap_or_else(|e| {
        error!("Error: {}", e);
        vec![]
    });
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
/// - `Button<AppMessage>` - The button.
pub fn new_button(message: AppMessage, label: text::Text, width: u16) -> Button<AppMessage> {
    let settings = Settings::new();
    button(
        label
            .size(settings.text_size())
            .horizontal_alignment(Horizontal::Center)
            .vertical_alignment(Vertical::Top),
    )
    .width(width)
    .height(40)
    .on_press(message)
    .style(Secondary)
}

/// Creates a new row.
///
/// ### Returns
/// - `Row<'static, AppMessage>`
pub fn make_new_row() -> Row<'static, AppMessage> {
    let settings = Settings::new();
    Row::new()
        .spacing(settings.spacing())
        .align_items(Vertical::Top.into())
}

/// Find the last day of a month.
///
/// ### Returns
/// - `i32`
pub fn last_day_of_month(year: i32, month: u32) -> i32 {
    get_date(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(get_date(year, month, 1))
    .num_days() as i32
}
