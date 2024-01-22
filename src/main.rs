use crossterm::event::{read, KeyCode, KeyEvent};
use crossterm::queue;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, Event},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;
use std::io::Result;
use std::io::Write;
use std::time::Duration;

mod game;

fn main() -> Result<()> {
    let mut game = game::Universe::new(40, 40);
    let mut stdout = stdout();

    enable_raw_mode()?;
    game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
    execute!(
        stdout,
        EnterAlternateScreen,
        SetForegroundColor(Color::White),
        Hide
    )?;

    loop {
        if poll(Duration::from_millis(500))? {
            if let Event::Key(KeyEvent { code, .. }) = read()? {
                if code == KeyCode::Esc {
                    break;
                }
            }
        } else {
            queue!(stdout, Clear(ClearType::All))?;
            let mut i = 0;
            while let Ok(line) = game.get_row_as_string(i) {
                queue!(stdout, MoveTo(0, i as u16), Print(line))?;
                i += 1;
            }

            queue!(
                stdout,
                MoveTo(0, (i + 1) as u16),
                Print("Press Esc to exit...")
            )?;
            stdout.flush()?;
            game.tick();
        }
    }
    execute!(stdout, ResetColor, Show, LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
