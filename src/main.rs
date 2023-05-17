use std::io::{stdout};
use std::thread;
use std::time::Duration;
use crossterm::event::{KeyEvent, Event, read, KeyEventKind};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, Show},
};
use render::Font;
use clap::Parser;

mod render;
mod animate;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    text: String,
    framerate: u64,
    delay: Option<u64>
}

fn main() {
    let cli: Cli = Cli::parse();
    run(&cli.text, cli.framerate, cli.delay); // example framerate 10 delay 500
}

fn run(text: &String, framerate: u64, delay: Option<u64>) {
    stdout().execute(Hide).unwrap();
    enable_raw_mode().unwrap();
    let chars: String = include_str!("font").replace("\r", "");
    let chars: Vec<&str> = chars.split("\n--NEXT-LETTER--\n").collect();
    let height = chars[0].split("\n").collect::<Vec<&str>>().len() as u32;
    let font: Font = Font::new(chars, height);
    let parts: Vec<(usize, String)> = text.split(", ").map(|string| render::frame(&font.render(string))).enumerate().collect();
    for (i, part) in &parts {
        animate::animate(part, Duration::from_millis(framerate));
        if i != &(parts.len() - 1) {
            match delay {
                Some(dur) => {
                    thread::sleep(Duration::from_millis(dur));
                }
                None => {
                    pause();
                }
            }
        }
    }
    pause();
    stdout().execute(Show).unwrap();
    disable_raw_mode().unwrap();
}

fn pause() {
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: _,
                modifiers: _,
                kind: KeyEventKind::Press,
                state: _,
            }) => return,
            _ => (),
        }
    }
}