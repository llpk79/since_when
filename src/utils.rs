use crate::events::EventOccurrence;
use chrono::NaiveDate;
use std::collections::HashMap;

/// Get the date from the day, month, and year.
///
/// # Arguments
/// - year: i32
/// - month: u32
/// - day: u32
///
/// # Returns
/// - NaiveDate
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
/// # Arguments
/// - events - &[EventOccurrence]
///
/// # Returns
/// - HashMap<String, Vec<i32>>
pub fn get_days_since_now(events: &[EventOccurrence]) -> HashMap<String, Vec<i32>> {
    let mut days_since_now: HashMap<String, Vec<i32>> = HashMap::new();
    for event in events.iter() {
        // Calculate the days between the events and the current date.
        let date = match NaiveDate::parse_from_str(&event.date, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                println!("Error parsing date");
                continue;
            }
        };
        let now = chrono::Local::now().naive_local().date();
        let days = now.signed_duration_since(date).num_days() as i32;
        if days_since_now.contains_key(&event.name) {
            let days_vec = match days_since_now.get_mut(&event.name) {
                Some(days_vec) => days_vec,
                None => {
                    println!("Error getting days vector");
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
/// - days_since - &HashMap<String, Vec<i32>>
///
/// ### Returns
/// - HashMap<String, Vec<i32>>
///
/// ### Example
/// ```
/// # use std::collections::HashMap;
/// let times = vec![1, 11, 22, 33, 44];
/// let mut days_since: HashMap<String, Vec<i32>> = HashMap::new();
/// days_since.insert("event".to_string(), times);
/// let expected_vec = vec![10, 11, 11, 11];
/// let expected = HashMap::new();
/// expected.insert("event".to_string(), expected_vec);
/// assert_eq!(get_elapsed_days(&days_since), expected);
/// ```
pub fn get_elapsed_days(days_since: &HashMap<String, Vec<i32>>) -> HashMap<String, Vec<i32>> {
    let mut elapsed: HashMap<String, Vec<i32>> = HashMap::new();
    let mut days_vec: Vec<i32> = Vec::new();
    for items in days_since.iter() {
        if items.1.len() > 1 {
            for item in 1..items.1.len() {
                let days = items.1[item] - items.1[item - 1];
                days_vec = match elapsed.get_mut(&items.0.clone()) {
                    Some(days_vec) => days_vec.clone(),
                    None => {
                        vec![days]
                    }
                };
                days_vec.push(days);
            }
            elapsed.entry(items.0.clone()).or_insert(days_vec.clone());
        }
    }
    elapsed
}

/// Get the average elapsed days between occurrences for each event.
///
/// ### Arguments
/// elapsed - HashMap<String, Vec<i32>>
///
/// ### Returns
/// HashMap<String, i32>
///
/// ### Example
/// ```
/// # use std::collections::HashMap;
/// let times = vec![1, 11, 22, 33, 44];
/// let map = HashMap::new();
/// map.entry("foo").or_insert(times);
/// assert_ep!(get_averages(map), 11);
/// ```
pub fn get_averages(elapsed: HashMap<String, Vec<i32>>) -> HashMap<String, i32> {
    let mut averages: HashMap<String, i32> = HashMap::new();
    for item in elapsed.iter() {
        let mut sum = 0;
        for num_days in item.1.iter() {
            sum += num_days;
        }
        let average = sum / item.1.len() as i32;
        averages.entry(item.0.to_string()).or_insert(average);
    }
    averages
}
