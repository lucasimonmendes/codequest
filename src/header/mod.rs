use std::io::Write;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

pub fn print_header(title: &str, phrase: &str) {
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        SetForegroundColor(Color::Magenta),
        Print(title),
        Print('\n'),
        ResetColor
    )
    .unwrap();
    stdout.flush().unwrap();

    println!("{}", phrase);
}
