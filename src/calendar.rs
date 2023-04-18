use crate::AppMessage;
use chrono::Datelike;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, row, text, Column, Row};
use iced::{theme, Element};
use iced::{Alignment, Command, Renderer};
use num_traits::cast::FromPrimitive;

const TEXT_SIZE: u16 = 50;
const SPACING: u16 = 10;
const CAL_TEXT_SIZE: u16 = 50;
const CAL_WIDTH: u16 = CAL_TEXT_SIZE + 20;

// Creates a new row.
pub fn make_new_row() -> Row<'static, AppMessage, Renderer> {
    Row::new().spacing(15).align_items(Vertical::Top.into())
}

// The state of the Calendar.
#[derive(Debug, Clone, Copy)]
pub struct Calendar {
    month: u32,
    year: i32,
}

impl<'a> Calendar {
    pub fn new() -> Calendar {
        // Get the current date.
        let now = chrono::Utc::now();
        // Get the current month.
        let month = now.month();
        // Get the current year.
        let year = now.year();
        // Create a new Calendar.
        Self { month, year }
    }
    // Updates the Calendar State via messages.
    pub fn update(&mut self, message: AppMessage) -> Command<AppMessage> {
        match message {
            // The user clicked the "Previous Month" button.
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
            // The user clicked the "Next Month" button.
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
    pub fn view(self) -> Element<'a, AppMessage> {
        // Text to explain what to do.
        let instructions = text("Click a day to add an event.").size(TEXT_SIZE);
        // Create a prev and next month buttons.
        let instruction_row = row!(instructions)
            .spacing(SPACING)
            .align_items(Vertical::Top.into());
        let prev_button = button(text("<").size(TEXT_SIZE))
            .style(theme::Button::Primary)
            .on_press(AppMessage::PreviousMonth);
        let next_button = button(text(">").size(TEXT_SIZE))
            .style(theme::Button::Primary)
            .on_press(AppMessage::NextMonth);
        // Display the current month and year.
        let month = chrono::Month::from_u32(self.month).unwrap();
        let text_month = text(format!("{:?} - {}", month, self.year)).size(TEXT_SIZE);
        // Create a row with the prev and next month buttons and the current month and year.
        let nav_row = row![prev_button, text_month, next_button]
            .spacing(SPACING)
            .align_items(Vertical::Top.into());
        let month_lengths = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        // Get the first weekday of the month to determine where to start the Calendar.
        let first_day = chrono::NaiveDate::from_ymd_opt(self.year, self.month, 1)
            .unwrap()
            .weekday();
        let from_sun = first_day.num_days_from_sunday() as i32;
        let offset = from_sun - 1;
        let mut content = Column::new()
            .spacing(SPACING)
            .align_items(Alignment::Center);
        content = content.push(instruction_row);
        content = content.push(nav_row);
        let mut calendar_row = make_new_row();
        // Draws the Calendar.
        for i in 0..42 {
            if (from_sun <= i) && (i < month_lengths[(self.month - 1) as usize] + offset + 1) {
                let day = (i - offset) as u32;
                calendar_row = calendar_row.push(
                    button(
                        text(format!("{}", day))
                            .size(CAL_TEXT_SIZE)
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .style(theme::Button::Primary)
                    .on_press(AppMessage::DayClicked(day, self.month, self.year))
                    .width(CAL_WIDTH),
                );
            } else {
                calendar_row = calendar_row.push(
                    button(
                        text(" ")
                            .size(CAL_TEXT_SIZE)
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .style(theme::Button::Primary)
                    .on_press(AppMessage::DayClicked(0, self.month, self.year))
                    .width(CAL_WIDTH),
                );
            }
            if (i + 1) % 7 == 0 {
                content = content.push(calendar_row);
                calendar_row = make_new_row();
            }
        }
        content = content.push(calendar_row);
        let events_button = button(text("Events").size(TEXT_SIZE))
            .on_press(AppMessage::EventsWindow)
            .style(theme::Button::Primary);
        let button_row = row![events_button];
        content = content.push(button_row);
        content.into()
    }
}