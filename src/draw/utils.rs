use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::Stdout;

pub fn full_clear(stdout: &mut Stdout) -> std::io::Result<()> {
    execute!(stdout, Clear(ClearType::All), Clear(ClearType::Purge))?;
    Ok(())
}
