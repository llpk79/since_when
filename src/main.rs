/*
Written in rust, using iced for the gui, and rusqlite for the data_base.
This app is to track the time since an Event has happened, expressed in days.
For example, the time since you last changed your oil, or the time since you last had a haircut.
The app is designed to be simple, and easy to use.
The app has three windows;
     - A window displaying a Calendar.
         - The Calendar is displayed in a grid of 7 columns and 6 rows.
         - Each cell in the grid is a button labeled with the day of the month.
         - The month and year are displayed above the Calendar.
         - Arrow buttons allow the user to navigate between months.
     - Clicking a Calendar date opens a page for entering new events.
         - A text box allows the user to enter a new event.
         - A button labeled "Add" allows the user to add the event to the data_base.
         - Clicking the "Update" button adds an Occurrence of the event to the data_base.
         - Clicking the "Delete" button removes the event from the data_base.
     - The main page showing all events tracked by the app and elapsed time since the event.
         - A list of events tracked for the day is displayed.
         - Elapsed time in days since the last event and average time between events are
         displayed to the left of the event name.
         - A button labeled Calendar is displayed at the bottom of the page.
*/

use iced::{Application, Settings};
use since_when_lib::app::SinceWhen;

/// The main function.
pub fn main() -> iced::Result {
    SinceWhen::run(Settings::default())
}
