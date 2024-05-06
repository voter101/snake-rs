use crossterm::{
    queue,
    terminal::{Clear, ClearType},
};
use std::io::Stdout;

pub fn full_clear(stdout: &mut Stdout) -> std::io::Result<()> {
    queue!(stdout, Clear(ClearType::All),)?;
    Ok(())
}
