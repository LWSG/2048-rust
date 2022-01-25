use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod game;

use game::Engine;

fn main() {
    let mut engine = Engine::init();
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(
        stdout,
        "{}{}",
        termion::cursor::Goto(1, 1),
        termion::clear::All
    )
        .unwrap();
    for i in &engine.game_data {
        for j in i {
            if *j != 0 {
                write!(stdout, "{}", *j).unwrap();
            }
            write!(stdout, "\t").unwrap();
        }
        write!(stdout, "\n\r").unwrap();
    }

    stdout.flush().unwrap();
    for c in stdin.keys() {
        //clearing the screen and going to top left corner
        write!(
            stdout,
            "{}{}",
            termion::cursor::Goto(1, 1),
            termion::clear::All
        )
            .unwrap();
        match c.unwrap() {
            Key::Up => {
                engine.go(game::Direction::Up);
            }
            Key::Down => {
                engine.go(game::Direction::Down);
            }
            Key::Left => {
                engine.go(game::Direction::Left);
            }
            Key::Right => {
                engine.go(game::Direction::Right);
            }
            Key::Ctrl('c') => {
                break;
            }
            _ => (),
        };
        //display
        //TODO Better display
        for i in &engine.game_data {
            for j in i {
                if *j != 0 {
                    write!(stdout, "{}", *j).unwrap();
                }
                write!(stdout, "\t").unwrap();
            }
            write!(stdout, "\n\r").unwrap();
        }
        //display score and quit
        if engine.is_failed() {
            write!(stdout, "\n\r{}", engine.score()).unwrap();
            break;
        }

        stdout.flush().unwrap();
    }
}
