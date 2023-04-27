/*
Written in rust, using iced for the gui, and rusqlite for the data_base.
This app is to track the time since an event has happened, expressed in days.
For example, the time since you last changed your oil, or the time since you last had a haircut.
The app is designed to be simple, and easy to use.
The app has three windows:
     - The main page showing all events tracked by the app and elapsed time since the event.
         - A list of events tracked for the day is displayed.
         - Elapsed time in days since the last event and average time between events are
         displayed to the right of the event name.
         - A button labeled "Add/Update Event" is displayed at the bottom of the page,
         it takes you to the calendar page.
     - A window displaying a Calendar.
         - The Calendar is displayed in a grid of 7 columns and 6 rows.
         - Each cell in the grid is a button labeled with the day of the month.
         - The month and year are displayed above the Calendar.
         - Arrow buttons allow the user to navigate between months.
     - Clicking a Calendar date opens a page for entering new events.
         - The date selected is displayed at the top of the page.
         - A text box allows the user to enter a new event title.
         - A button labeled "Add Event" allows the user to add the event to the data_base.
         - Clicking the "Update Event" button adds an occurrence of the event to the data_base.
         - Clicking the "Delete Event" button removes the event from the data_base.
         - Buttons for returning to the main page and the calendar page are displayed at the
         bottom of the page.
 */
#![windows_subsystem = "windows"]  // Prevents windows from opening a terminal window.

use iced::{Application, Settings};
use since_when_lib::app::SinceWhen;
use env_logger::Env;
extern crate log;

/// The main function.
pub fn main() -> iced::Result {
    // Initialize the logger.
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "error")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    // Run the app.
    SinceWhen::run(Settings::default())
}
