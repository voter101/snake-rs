use crossterm::style::Color;

pub const FPS_LIMIT: u64 = 300;

pub const BACKGROUND_COLOR: Color = Color::AnsiValue(238);
pub const BACKGROUND_TEXT_COLOR: Color = Color::White;

pub const BOARD_FIELD_BACKGROUND_COLOR: Color = Color::AnsiValue(52);
pub const BOARD_FIELD_TEXT_COLOR: Color = Color::White;

pub const BOARD_BORDER_COLOR: Color = Color::AnsiValue(232);

pub const FPS_COUNTER_BACKGROUND_COLOR: Color = Color::AnsiValue(27);
pub const FPS_COUNTER_TEXT_COLOR: Color = Color::Black;
