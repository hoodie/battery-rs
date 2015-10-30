use std::fs::File;
use std::io::Read;
use std::io::Result;

extern crate colors;
use colors::{Styles,Style};

const SPARKS:&'static str="_▁▁▂▃▄▄▅▆▇██";

fn read_capacity() -> Result<String>{
    let mut file = try!(File::open("/sys/class/power_supply/BAT0/capacity"));
    let mut capacity = String::new();
    file.read_to_string(&mut capacity).unwrap();
    Ok(capacity)
}

fn cap_color(capacity:i32) -> (Styles,Styles) {
    match capacity{
        0...5    => (Styles::Black,  Styles::Dim),
        5...10   => (Styles::Black,  Styles::Bold),
        10...20  => (Styles::Red,    Styles::Dim),
        20...30  => (Styles::Red,    Styles::Bold),
        30...40  => (Styles::Yellow, Styles::Dim),
        40...55  => (Styles::Yellow, Styles::Bold),
        45...65  => (Styles::Green,  Styles::Bold),
        65...100 => (Styles::Green,  Styles::Dim),
        _ => (Styles::Black, Styles::Inverse)
    }
}

fn _test(){
    for cap in 1..20{
        let capacity = cap * 5;
        let bar = SPARKS.chars().nth(capacity as usize / 10).unwrap_or('x');
        let bar = bar.to_string()
            .style(cap_color(capacity).0)
            .style(cap_color(capacity).1);
        print!("{bar}{capacity}%", capacity = capacity, bar = &bar);
    }
}

fn main(){
    let capacity:i32 = read_capacity().unwrap_or("-1".to_owned())
                                      .trim()
                                      .parse().unwrap_or(-1);
        let bar = SPARKS.chars().nth(capacity as usize / 10).unwrap_or('x');
        let bar = bar.to_string()
            .style(cap_color(capacity).0)
            .style(cap_color(capacity).1);
        println!("{bar}{capacity}%", capacity = capacity, bar = &bar);


}
