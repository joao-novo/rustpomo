pub mod pomo {
    use figlet_rs;
    use std::time::{Duration, Instant};
    pub enum State {
        Work,
        Break,
    }

    /// Starts a timer with the desired work time and break time.
    ///
    /// # Arguments
    /// * `work_time` - A 64-bit unsigned integer that holds the duration of the work period in minutes
    /// * `break_time` - A 64-bit unsigned integer that holds the duration of the break period in minutes
    pub fn timer(work_time: u64, break_time: u64) -> fn(Duration) {
        //TODO add timer functionality
        let mut current_state: Option<State> = None;
        loop {
            current_state = match current_state {
                Some(State::Work) => Some(State::Break),
                Some(State::Break) => Some(State::Work),
                None => Some(State::Work),
            };

            let work_duration = Duration::from_secs(work_time * 60);
            let break_duration = Duration::from_secs(break_time * 60);
            match current_state {
                Some(State::Work) => timer_aux(work_duration),
                Some(State::Break) => timer_aux(break_duration),
                None => (),
            }
        }
    }

    pub fn timer_aux(duration: Duration) {
        let start = Instant::now();
        let font = figlet_rs::FIGfont::standard().unwrap();
        let mins = duration.as_secs() / 60;
        let secs = duration.as_secs() % 60;
        let figure = font.convert(&format!("{}:{}", mins, secs)[..]);
        println!("{}", figure.unwrap());
        std::thread::sleep(duration);
    }
}
