use chrono::Datelike;
use iced::alignment::{Horizontal, Vertical};
use iced::theme::Button::Secondary;
use iced::widget::{button, row, text, Column, Row};
use iced::{Alignment, Command, Element, Renderer};
use num_traits::cast::FromPrimitive;
use std::collections::HashMap;

use crate::{
    app::AppMessage,
    database::events_by_year_month,
    settings::Settings,
    utils::{get_date, last_day_of_month, make_new_row, new_button},
};

/// The state of the Calendar.
#[derive(Debug, Clone, Copy)]
pub struct Calendar {
    month: u32,
    year: i32,
}

/// Default Calendar implementation.
impl Default for Calendar {
    fn default() -> Self {
        Calendar::new()
    }
}

/// Calendar window implementation.
impl<'a> Calendar {
    pub fn new() -> Calendar {
        // Get the current date for starting month.
        let now = chrono::Utc::now();
        let month = now.month();
        let year = now.year();
        Self { month, year }
    }

    /// Updates the Calendar State via messages.
    ///
    /// ### Arguments
    /// - message - `AppMessage` - The message to process.
    ///
    /// ### Returns
    /// - `Command<AppMessage>` - The command to execute.
    pub fn update(&mut self, message: AppMessage) -> Command<AppMessage> {
        match message {
            AppMessage::PreviousMonth => {
                // If the current month is January, set the month to December and decrement the year.
                if self.month == 1 {
                    self.month = 12;
                    self.year -= 1;
                }
                // Otherwise, decrement the month.
                else {
                    self.month -= 1;
                }
            }
            AppMessage::NextMonth => {
                // If the current month is December, set the month to January and increment the year.
                if self.month == 12 {
                    self.month = 1;
                    self.year += 1;
                }
                // Otherwise, increment the month.
                else {
                    self.month += 1;
                }
            }
            _ => {}
        }
        Command::none()
    }

    /// Instructions for the Calendar window.
    ///
    /// ### Returns
    /// - `Row<'a, AppMessage, Renderer>` - The instructions row.
    fn instruction_row(self) -> Row<'a, AppMessage, Renderer> {
        // Text to explain what to do.
        let settings = Settings::new();
        let instructions =
            text("Click a day to add or update an event.").size(settings.text_size());
        // Create a row for current month, prev and next month buttons.
        let instruction_row = row![instructions]
            .spacing(settings.spacing())
            .align_items(Vertical::Top.into());
        instruction_row
    }

    /// Creates a row with the current month and year, prev and next month buttons.
    ///
    /// ### Returns
    /// - `Row<'a, AppMessage, Renderer>` - The navigation row.
    fn nav_row(self) -> Row<'a, AppMessage, Renderer> {
        let settings = Settings::new();
        let prev_button = new_button(AppMessage::PreviousMonth, text("<"), settings.text_size());
        // Display the current month and year.
        let month = match chrono::Month::from_u32(self.month) {
            Some(month) => month,
            None => panic!("Invalid month"),
        };
        let text_month = text(format!("{:?} - {}", month, self.year))
            .size(settings.text_size())
            .horizontal_alignment(Horizontal::Center)
            .width(160);
        let next_button = new_button(AppMessage::NextMonth, text(">"), settings.text_size());
        // Return a row with the prev and next month buttons and the current month and year.
        row![prev_button, text_month, next_button]
            .spacing(settings.spacing())
            .align_items(Vertical::Center.into())
    }

    /// Creates the Calendar view.
    ///
    /// ### Returns
    /// - `Column<'a, AppMessage, Renderer>` - The Calendar view.
    fn calendar(self) -> Column<'a, AppMessage, Renderer> {
        let settings = Settings::new();
        // Create a column to hold the Calendar.
        let mut calendar = Column::new()
            .spacing(settings.spacing())
            .align_items(Alignment::Center);
        let mut calendar_row = make_new_row();
        // Get the weekday of the first day of the month to determine where to start the Calendar.
        let first_day = get_date(self.year, self.month, 1);
        let last_day = last_day_of_month(self.year, self.month);
        let weekday = first_day.weekday();
        let from_sun = weekday.num_days_from_sunday() as i32;
        // Get the offset to start the Calendar.
        let offset = from_sun - 1;
        // Variables to hold the current day and the day to display.
        let mut day: u32;
        let mut print_day: String;
        let current_events = match events_by_year_month(self.year, self.month) {
            Ok(current_events) => current_events,
            Err(_) => HashMap::new(),
        };
        // Iterate through the 6x7 calendar grid.
        for i in 0..42 {
            // If the current day is between the first day of the month and the last day of the month, display the day.
            if (from_sun <= i) && (i < (last_day + from_sun)) {
                day = (i - offset) as u32;
                let day_of_week = get_date(self.year, self.month, day).weekday();

                print_day = format!("{}    {}", day, day_of_week)
            // Otherwise, display a blank space.
            } else {
                day = 0;
                print_day = " ".to_string()
            };
            if current_events.contains_key(&day) {
                if let Some(event_vec) = current_events.get(&day) {
                    for event in event_vec {
                        print_day = print_day + "\n" + event;
                    }
                }
            };
            calendar_row = calendar_row.push(
                button(
                    text(print_day)
                        .vertical_alignment(Vertical::Top)
                        .horizontal_alignment(Horizontal::Left)
                        .size(15),
                )
                .on_press(AppMessage::DayClicked(day, self.month, self.year))
                .style(Secondary)
                .width(settings.calendar_width())
                .height(settings.calendar_width()),
            );
            // If the current day is a Saturday, push the current row and start a new week.
            if (i + 1) % 7 == 0 {
                calendar = calendar.push(calendar_row);
                calendar_row = make_new_row();
            }
        }
        calendar = calendar.push(calendar_row);
        calendar
    }

    /// Create the Calendar view.
    ///
    /// The Calendar is a 7 x 6 grid of day buttons.
    ///
    /// ### Returns
    /// - `Element<'a, AppMessage>` - The Calendar page.
    pub fn view(self) -> Element<'a, AppMessage> {
        let settings = Settings::new();
        // Create a column to hold the calendar, nav buttons, and instructions.
        let content = Column::new()
            .push(self.instruction_row())
            .push(self.nav_row())
            .push(self.calendar())
            .push(new_button(
                AppMessage::EventsWindow,
                text("Events"),
                settings.add_button_size(),
            ))
            .spacing(settings.spacing())
            .align_items(Alignment::Center);
        content.into()
    }
}
