use clap::Parser;

/// Simple CLI Pomodoro Timer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Work period duration (in minutes)
    #[arg(short, long, default_value_t = 25)]
    work: u32,

    /// Break period duration (in minutes)
    #[arg(short, long, default_value_t = 5)]
    r#break: u32,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
