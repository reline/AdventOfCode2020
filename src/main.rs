mod dayone;
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
}

fn main() {
    let args = Advent::from_args();
    match args.day {
        Day::DayOne { path } => {
            let expense_report = io::file_to_int_array(path);
            let result = dayone::partone(expense_report);
            println!("{}", result);
        }
    }
}
