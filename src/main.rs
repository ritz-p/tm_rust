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
    let one_millis = time::Duration::from_millis(10);
    let hundread_millis = time::Duration::from_millis(90);
    let mut res:Vec<String> = vec!["".to_string();10];
    let ti = character::ti();
    let nn = character::nn();
    let mo = character::mo();
    for i in 0..10{
        res[i] = format!("{} {} {} {} {} {} {} {}",ti[i],nn[i],ti[i],nn[i],mo[i],ti[i],mo[i],ti[i]);
    }
    let max = res[0].len();
    let pos = terminal_size().unwrap();
    let x = pos.0;
    let y = pos.1;
    for i in 0..x{
        for j in 1..=10{
            let _ = stdout.flush();
            thread::sleep(one_millis);
            write!(stdout,"{}",cursor::Goto(x - i,y/2 - 5 + j)).unwrap();
            if max < i as usize{
                write!(stdout,"{}",&res[(j as usize)-1][0..max]).unwrap();
            }else{
                write!(stdout,"{}",&res[(j as usize)-1][0..(i as usize)]).unwrap();
            }
        }
        thread::sleep(hundread_millis);
    }
    let _ = stdout.flush().unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}