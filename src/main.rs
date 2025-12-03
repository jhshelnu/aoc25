use clap::Parser;

mod day1;
mod day2;
mod day3;

#[derive(Parser)]
struct Args {
    /// Which day's problem to solve
    #[arg(long, value_parser = parse_day)]
    day: u8,
}

fn parse_day(day: &str) -> Result<u8, String> {
    let day: u8 = day
        .parse()
        .map_err(|_| format!("Expected numeric value in range [1, 12], got '{}'", day))?;

    if !(1..=12).contains(&day) {
        return Err(format!(
            "Expected numeric value in range [1, 12], got '{}'",
            day
        ));
    }

    Ok(day)
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        _ => panic!("day {} is not yet implemented", args.day),
    }
}
