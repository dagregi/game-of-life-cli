use clap::Parser;
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
use game::Universe;
use std::fs;
use std::io::stdout;
use std::io::Result;
use std::io::Write;
use std::time::Duration;

mod game;

#[derive(Debug, Parser)]
/// Conway's game of life written in rust
struct Args {
    /// The delay between ticks (in milliseconds).
    #[arg(short, long, default_value = "500")]
    delay: u64,
    #[arg(short, long)]
    /// The file path to read the intial state of the game
    input: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut stdout = stdout();

    let delay = args.delay;
    let mut game = if let Some(path) = args.input {
        read_from_file(&path)?
    } else {
        let mut default_game = Universe::new(5, 5);
        default_game.set_cells(&[(2, 1), (2, 2), (2, 3)]);
        default_game
    };
    enable_raw_mode()?;
    execute!(
        stdout,
        EnterAlternateScreen,
        SetForegroundColor(Color::White),
        Hide
    )?;

    loop {
        if poll(Duration::from_millis(delay))? {
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

fn read_from_file(path: &str) -> Result<Universe> {
    let contents = fs::read_to_string(path)?;

    let lines: Vec<&str> = contents.lines().collect();
    let dimensions: Vec<u32> = lines
        .first()
        .ok_or("File is empty")
        .unwrap()
        .split('x')
        .map(|s| s.parse().unwrap())
        .collect();
    let rows = dimensions.first().copied().unwrap_or(0);
    let cols = dimensions.last().copied().unwrap_or(0);

    let matrix: Vec<Vec<u8>> = lines
        .iter()
        .skip(1)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let cells: Vec<(u32, u32)> = matrix
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, col)| {
                if col == &1 {
                    Some((j as u32, i as u32))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut game = Universe::new(cols, rows);
    game.set_cells(&cells);
    Ok(game)
}
