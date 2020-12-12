mod dayfive;
mod dayfour;
mod dayone;
mod daysix;
mod daythree;
mod daytwo;
mod io;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Advent {
    // The advent day corresponding to the challenge to run
    #[structopt(subcommand)]
    day: Day,
}

#[derive(StructOpt)]
enum Day {
    DayOne {
        // The path to the file to read
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    DayTwo {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    DayThree {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    DayFour {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    DayFive {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    DaySix {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
}

fn main() {
    let args = Advent::from_args();
    match args.day {
        Day::DayOne { path } => {
            let expense_report = io::file_to_int_array(path);
            println!("Part One: {}", dayone::partone(&expense_report));
            println!("Part Two: {}", dayone::parttwo(&expense_report));
        }
        Day::DayTwo { path } => {
            let passwords = io::file_to_lines(path);
            println!("Part One: {}", daytwo::partone(&passwords));
            println!("Part Two: {}", daytwo::parttwo(&passwords));
        }
        Day::DayThree { path } => {
            let map = io::file_to_lines(path);
            println!("Part One: {}", daythree::partone(&map));
            println!("Part Two: {}", daythree::parttwo(&map));
        }
        Day::DayFour { path } => {
            let passports = io::file_to_lines(path);
            println!("Part One: {}", dayfour::partone(&passports));
            println!("Part Two: {}", dayfour::parttwo(&passports));
        }
        Day::DayFive { path } => {
            let seats = io::file_to_lines(path);
            println!("Part One: {}", dayfive::partone(&seats));
            println!("Part Two: {}", dayfive::parttwo(&seats));
        }
        Day::DaySix { path } => {
            let answers = io::file_to_lines(path);
            println!("Part One: {}", daysix::partone(&answers));
            println!("Part Two: {}", daysix::parttwo(&answers));
        }
    }
}
