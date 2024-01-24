pub mod pomo {
    use indicatif::{self, ProgressBar, ProgressStyle};
    use soloud::*;
    use std::thread;
    use std::time::Duration;
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
        let mut current_state: Option<State> = None;
        loop {
            current_state = match current_state {
                Some(State::Work) => Some(State::Break),
                Some(State::Break) => Some(State::Work),
                None => Some(State::Work),
            };

            let sl = Soloud::default().unwrap();
            let mut speech = audio::Speech::default();
            let work_duration = work_time * 60;
            let break_duration = break_time * 60;
            // TODO fix speech synthesis
            match current_state {
                Some(State::Work) => {
                    println!("Starting work period!");
                    timer_aux(work_duration);
                    speech.set_text("Work period completed").unwrap();
                    sl.play(&speech);
                }
                Some(State::Break) => {
                    println!("Starting break period!");
                    timer_aux(break_duration);
                    speech.set_text("Break period completed").unwrap();
                    sl.play(&speech);
                }
                None => (),
            }
        }
    }

    pub fn timer_aux(duration: u64) {
        let progress = ProgressBar::new(duration);
        progress.set_style(
            ProgressStyle::with_template("[{wide_bar}] [{elapsed_precise}]")
                .unwrap()
                .progress_chars("##-"),
        );
        for _ in 0..duration {
            progress.inc(1);
            thread::sleep(Duration::from_secs(1));
        }
    }
}
