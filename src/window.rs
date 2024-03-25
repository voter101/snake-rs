use crossterm::terminal::window_size;

pub type WindowDimensions = (u16, u16);

pub fn window_dimensions() -> WindowDimensions {
    let terminal_size = window_size().unwrap();

    (terminal_size.rows, terminal_size.columns)
}
