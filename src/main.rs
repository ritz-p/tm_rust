pub mod character;

use termion::*;
use std::io::{Write, stdout, stdin};
use termion::raw::IntoRawMode;
use std::{thread,time};

fn main() {
    let _stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
    let _ = stdout.flush();

    let hundread_millis = time::Duration::from_millis(100);
    let mut res:Vec<String> = vec!["".to_string();10];
    let ti = character::ti();
    let nn = character::nn();
    let mo = character::mo();
    for i in 0..10{
        res[i] = format!("{} {} {} {} {} {} {} {}",ti[i],nn[i],ti[i],nn[i],mo[i],ti[i],mo[i],ti[i]);
    }
    let len = res[0].len();
    for i in 0..len{
        for j in 1..=10{
            let _ = stdout.flush();
            write!(stdout,"{}",cursor::Goto(0,j)).unwrap();
            write!(stdout,"{}",&res[(j as usize)-1][0..=(i as usize)]).unwrap();
        }
        thread::sleep(hundread_millis);
    }
    let _ = stdout.flush().unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}