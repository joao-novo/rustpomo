use clap::Parser;
use rustpomo::pomo;

#[derive(Parser, Debug)]
#[clap(author = "João Novo", version, about)]
/// Simple CLI Pomodoro Timer
struct Args {
    /// Work period duration (in minutes)
    #[arg(short, long, default_value_t = 25)]
    work_time: u64,

    /// Break period duration (in minutes)
    #[arg(short, long, default_value_t = 5)]
    break_time: u64,
}

fn main() {
    let args = Args::parse();
    pomo::timer(args.work_time, args.break_time);
}
