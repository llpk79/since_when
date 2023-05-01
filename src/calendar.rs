use crate::{app::AppMessage, settings::Settings, utils};
use chrono::Datelike;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, row, text, Column, Row};
use iced::{theme, Alignment, Command, Element, Renderer};
use num_traits::cast::FromPrimitive;

/// Creates a new row.
///
/// # Returns
/// - `Row<'static, AppMessage, Renderer>`
pub fn make_new_row() -> Row<'static, AppMessage, Renderer> {
    let settings = Settings::new();
    Row::new()
        .spacing(settings.spacing())
        .align_items(Vertical::Top.into())
}

/// The state of the Calendar.
#[derive(Debug, Clone, Copy)]
pub struct Calendar {
    month: u32,
    year: i32,
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
    /// # Arguments
    /// - message - `AppMessage`
    ///
    /// # Returns
    /// - `Command<AppMessage>`
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

    /// Create the Calendar view.
    ///
    /// The Calendar is a 7 x 6 grid of day buttons.
    /// Buttons for moving to the next and previous month.
    /// Button for adding an event.
    ///
    /// # Returns
    /// - `Element<'a, AppMessage>`
    pub fn view(self) -> Element<'a, AppMessage> {
        let settings = Settings::new();
        // Text to explain what to do.
        let instructions =
            text("Click a day to add or update an event.").size(settings.text_size());
        // Create a row for current month, prev and next month buttons.
        let instruction_row = row!(instructions)
            .spacing(settings.spacing())
            .align_items(Vertical::Top.into());
        let prev_button = button(text("<").size(settings.text_size()))
            .style(theme::Button::Secondary)
            .on_press(AppMessage::PreviousMonth);
        let next_button = button(text(">").size(settings.text_size()))
            .style(theme::Button::Secondary)
            .on_press(AppMessage::NextMonth);

        // Display the current month and year.
        let month = match chrono::Month::from_u32(self.month) {
            Some(month) => month,
            None => panic!("Invalid month"),
        };
        let text_month = text(format!("{:?} - {}", month, self.year)).size(settings.text_size());
        // Create a row with the prev and next month buttons and the current month and year.
        let nav_row = row![prev_button, text_month, next_button]
            .spacing(settings.spacing())
            .align_items(Vertical::Top.into());

        // Create a column to hold the Calendar, buttons, and instructions.
        let mut content = Column::new()
            .spacing(settings.spacing())
            .align_items(Alignment::Center);
        content = content.push(instruction_row);
        content = content.push(nav_row);

        // Draw the Calendar.
        let month_lengths = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        // Get the first weekday of the month to determine where to start the Calendar.
        let first_day = utils::get_date(self.year, self.month, 1);
        let weekday = first_day.weekday();
        let from_sun = weekday.num_days_from_sunday() as i32;
        let offset = from_sun - 1;
        let mut calendar_row = make_new_row();
        let mut day: u32;
        let mut print_day: String;
        for i in 0..42 {
            if (from_sun <= i) && (i < month_lengths[(self.month - 1) as usize] + offset + 1) {
                day = (i - offset) as u32;
                print_day = format!("{}", day)
            } else {
                day = 0;
                print_day = " ".to_string()
            };
            calendar_row = calendar_row.push(
                button(
                    text(print_day)
                        .size(settings.calendar_text_size())
                        .horizontal_alignment(Horizontal::Center),
                )
                .style(theme::Button::Secondary)
                .on_press(AppMessage::DayClicked(day, self.month, self.year))
                .width(settings.calendar_width()),
            );

            if (i + 1) % 7 == 0 {
                content = content.push(calendar_row);
                calendar_row = make_new_row();
            }
        }
        content = content.push(calendar_row);

        // Add a button to go to the Events window.
        let events_button = button(text("Events").size(settings.text_size()))
            .on_press(AppMessage::EventsWindow)
            .style(theme::Button::Secondary);
        let button_row = row![events_button];
        content = content.push(button_row);
        content.into()
    }
}
