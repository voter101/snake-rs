use crate::consts;

use crossterm::{
    cursor::Hide,
    execute,
    style::{SetBackgroundColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};

pub fn hook_into_terminal(stdout: &mut std::io::Stdout) -> std::io::Result<()> {
    enable_raw_mode()?;
    execute!(
        stdout,
        EnterAlternateScreen,
        SetBackgroundColor(consts::BACKGROUND_COLOR),
        SetForegroundColor(consts::BACKGROUND_TEXT_COLOR),
        Hide
    )
}

pub fn unmount_from_terminal() -> std::io::Result<()> {
    disable_raw_mode()
}
