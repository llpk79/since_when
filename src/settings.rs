// App settings.
pub struct Settings {
    text_size: u16,
    add_button_size: u16,
    spacing: u16,
    calendar_text_size: u16,
    calendar_width: u16,
    padding: u16,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            text_size: 40,
            add_button_size: 225,
            spacing: 20,
            calendar_text_size: 40,
            calendar_width: 60,
            padding: 5,
        }
    }
    pub fn text_size(&self) -> u16 {
        self.text_size
    }
    pub fn add_button_size(&self) -> u16 {
        self.add_button_size
    }
    pub fn spacing(&self) -> u16 {
        self.spacing
    }
    pub fn calendar_text_size(&self) -> u16 {
        self.calendar_text_size
    }
    pub fn calendar_width(&self) -> u16 {
        self.calendar_width
    }
    pub fn padding(&self) -> u16 {
        self.padding
    }
}