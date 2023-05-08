use std::fs;
use std::io::stdout;
use std::io::Write;
use std::process::Command;
use std::thread;
use std::time::Duration;
use crossterm::{
    ExecutableCommand,
    cursor::{Hide, Show}
};

const CHARSET: &str = "!\"#$%&'()*+,-_./\\|0123456789:;<=>?@abcdefghijklmnopqrstuvwxyz[£]↑←─♠╮╰╯╲╱●♥╭╳○♣♦┼│π◥▌▄▔▁▏▒▕◤├▗└┐▂┌┴┬┤▎▍▃▖▝┘▘▚─";

fn main() {
    let to_animate = fs::read_to_string("text_to_animate.txt")
        .expect("Should have been able to read the file");
    stdout().execute(Hide).unwrap();
    let parts: Vec<(usize, &str)> = to_animate.split("\r\n--NEXT--\r\n").enumerate().collect();
    for (i, part) in &parts {
        animate(&part, Duration::from_millis(10));
        if i != &(parts.len() - 1) {
            thread::sleep(Duration::from_millis(500));
        }
    }
    stdout().execute(Show).unwrap();
}

fn find(string: &str, char: char) -> Option<usize> {
    string.chars().position(|c| c == char)
}

fn animate(image: &str, speed: Duration) {
    let mut frames = 0;
    let mut done = false;
    while !done {
        let mut frame = String::new();
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        for c in image.chars() {
            let char_lower = c.to_ascii_lowercase();
            if CHARSET.contains(char_lower) {
                let index: i32 = find(CHARSET, char_lower).unwrap() as i32;
                let dist = x + y;
                let display_index = i32::min(index, frames - dist);
                if display_index >= 0 {
                    let index = display_index as usize;
                    frame.push(if c.is_lowercase() { CHARSET.chars().nth(index).unwrap() } else {
                        match CHARSET.chars().nth(index) {
                            Some(str) => {str.to_ascii_uppercase()}
                            None => {panic!("): {} > {}, {}", (frames - dist), CHARSET.find(char_lower).unwrap(), char_lower)}
                        }
                    });
                } else {
                    frame.push(' ');
                }
            } else {
                frame.push(c);
                if c == '\n' {
                    y += 1;
                    x = -1;
                }
            }
            x += 1;
        }
        clear_terminal();
        print!("{}", frame);
        stdout().flush().unwrap();
        thread::sleep(speed);
        frames += 1;
        if frame == image {
            done = true;
        }
    }
}

fn clear_terminal() {
    Command::new("cmd").arg("/C").arg("cls").status().unwrap();
}