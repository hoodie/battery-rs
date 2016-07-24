#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(feature = "dev", allow(blacklisted_name))]

use std::io::Read;
use std::io::Result;
use std::fs::File;
use std::path::{Path,PathBuf};

#[macro_use] extern crate clap;
use clap::{App, Arg};

extern crate termion;
use termion::color;
use termion::color::Fg;


const SPARKS:&'static str="_▁▁▂▃▄▄▅▆▇██";

fn list_batteries() -> Result<Vec<PathBuf>> {
    let path = "/sys/class/power_supply/";
    //let path = "./power_supply/";
    std::fs::read_dir(path).map(|entries|entries
                                .filter_map(|entry| entry.ok())
                                .map(|entry| entry.path().join("capacity"))
                                .filter(|path| path.exists())
                                .collect::<Vec<PathBuf>>()
                               )
}

fn read_capacity(battery_path:&Path) -> Result<String>{
    let mut file = try!(File::open(battery_path));
    let mut capacity = String::new();
    file.read_to_string(&mut capacity).unwrap();
    Ok(capacity)
}



use std::fmt::Display;
fn format_cap<T:Display>(content:T, capacity:i32) -> String{
    match capacity{
        0...5    => format!("{1}{0}", content, Fg(color::Red)),
        5...10   => format!("{1}{0}", content, Fg(color::Red)),
        10...20  |
        20...30  => format!("{1}{0}", content, Fg(color::LightRed)),
        30...40  => format!("{1}{0}", content, Fg(color::Yellow)),
        40...55  => format!("{1}{0}", content, Fg(color::LightYellow)),
        45...65  => format!("{1}{0}", content, Fg(color::Green)),
        65...100 => format!("{1}{0}", content, Fg(color::LightGreen)),
        _        => format!("{1}{0}", content, Fg(color::Black))
    }
}

fn test(){
    for (i,c) in SPARKS.chars().enumerate() {
        let bar = format_cap(c, (i as i32)*10);
        print!("{bar}", bar = &bar);
    }
}

fn test_colors() {
    for cap in 1..20{
        let capacity = cap * 5;
        let bar = SPARKS.chars().nth(10).unwrap_or('x');
        let bar = format_cap(bar, capacity);
        print!("{bar}", bar = &bar);
    }
}

fn write_capacity_simple(battery_path:&Path) -> String {
    let capacity:i32 = read_capacity(battery_path).unwrap_or("-1".to_owned())
        .trim()
        .parse()
        .unwrap_or(-1);

    format!("{}", capacity)
}

fn write_capacity(battery_path:&Path, percent:bool) -> String {
    let capacity:i32 = read_capacity(battery_path).unwrap_or("-1".to_owned())
        .trim()
        .parse().unwrap_or(-1);

    let bar = SPARKS.chars().nth(capacity as usize / 10).unwrap_or('x');
    let bar = format_cap(bar, capacity);
    if percent{
        format!("{bar}{style} {capacity}%",
                capacity = capacity,
                style = termion::style::Bold,
                bar = &bar)
    } else {
        format!("{bar}{style} {capacity}",
                capacity = capacity,
                style = termion::style::Bold,
                bar = &bar)
    }
}

fn for_each_battery<F>(func:F) where F: Fn(&Path) -> String{
    if let Ok(paths) = list_batteries(){
        let buf = paths.iter()
                       .map(|pbuf|func(pbuf))
                       .collect::<String>();
        println!("{}", buf);
    } else {
        println!("no battery found");
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

        .arg( Arg::with_name("list")
              .help("lists batteries")
              .short("l").long("list"))

        .get_matches();

    if matches.is_present("test") {
        test();
    }

    else if matches.is_present("test colors") {
        test_colors();
    }

    else if matches.is_present("simple") {
        for_each_battery(|battery_path| write_capacity_simple(battery_path));
    }

    else if matches.is_present("list") {
    }

    else
    {
        for_each_battery(|battery_path|
                         write_capacity(battery_path, !matches.is_present("nopercent"))
                        );
    }
}
