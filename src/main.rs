use std::fs::File;
use std::io::Read;
use std::io::Result;

#[macro_use] extern crate clap;
use clap::{App, Arg};

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
        20...30  => (Styles::Red,    Styles::Dim),
        30...40  => (Styles::Red,    Styles::Bold),
        40...55  => (Styles::Yellow, Styles::Bold),
        45...65  => (Styles::Green,  Styles::Bold),
        65...100 => (Styles::Green,  Styles::Dim),
        _ => (Styles::Black, Styles::Inverse)
    }
}

fn test(){
    for (i,c) in SPARKS.chars().enumerate(){
        let (color, style) = cap_color((i as i32)*10);
        let bar = c.to_string()
            .style(color)
            .style(style);
        print!("{bar}", bar = &bar);
    }
}

fn test_colors(){
    for cap in 1..20{
        let capacity = cap * 5;
        let bar = SPARKS.chars().nth(10).unwrap_or('x');
        let (color, style) = cap_color(capacity);
        let bar = bar.to_string()
            .style(color)
            .style(style);
        print!("{bar}", bar = &bar);
    }
}

fn print_capacity_simple( ){
    let capacity:i32 = read_capacity().unwrap_or("-1".to_owned())
                                      .trim()
                                      .parse().unwrap_or(-1);
        println!("{}", capacity );
}

fn print_capacity( percent:bool ){
    let capacity:i32 = read_capacity().unwrap_or("-1".to_owned())
        .trim()
        .parse().unwrap_or(-1);
    let bar = SPARKS.chars().nth(capacity as usize / 10).unwrap_or('x');
    let bar = bar.to_string()
        .style(cap_color(capacity).0)
        .style(cap_color(capacity).1);
    if percent{
        println!("{bar}{capacity}%", capacity = capacity, bar = &bar);
    } else {
        println!("{bar}{capacity}", capacity = capacity, bar = &bar);
    }
}

fn main(){
    let matches = App::new("battery")
        .version(&crate_version!()[..])
        .author("Hendrik Sollich <hendrik@hoodie.de>")
        .about("better acpi terminal tool except ramble yamble")
        //.arg_required_else_help(true)

        .arg( Arg::with_name("test")
              .help("tests")
              .short("t").long("test"))

        .arg( Arg::with_name("simple")
              .help("simple display")
              .short("s").long("simple"))

        .arg( Arg::with_name("test colors")
              .help("tests colors")
              .short("c").long("colors"))

        .arg( Arg::with_name("nopercent")
              .help("don't show percent sign ( zsh compatibility ) ")
              .short("n").long("nopercent"))

        .get_matches();

    if matches.is_present("test") {
        test();
    } else if matches.is_present("test colors") {
        test_colors();
    } else if matches.is_present("simple") {
        print_capacity_simple();
    } else {
        print_capacity(!matches.is_present("nopercent"));
    }
}
