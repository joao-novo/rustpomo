pub mod pomo {
    use indicatif::{self, ProgressBar, ProgressStyle}; // Progress bar used for the timer
    use std::{thread, time::Duration};
    use tts; // Speech synthesis

    /// Enum that holds each of the possible states
    ///
    /// # Variants
    /// * `Work` - Represents the state in which the timer is in the work period
    /// * `Break` - Represents the state in which the timer is in the break period
    enum State {
        Work,
        Break,
    }

    /// Starts a timer with the desired work time and break time.
    ///
    /// # Arguments
    /// * `work_time` - A 64-bit unsigned integer that holds the duration of the work period in minutes
    /// * `break_time` - A 64-bit unsigned integer that holds the duration of the break period in minutes
    pub fn timer(work_time: u64, break_time: u64) -> fn(Duration) {
        println!("Welcome to Rustpomo. Your timer will begin in the work period with a duration of {work_time} minutes, followed by a break period of {break_time} minutes. It will keep looping until you close the terminal or kill the process.");
        thread::sleep(Duration::from_secs(3));
        let mut current_state: Option<State> = None;
        loop {
            // Handles state-switching
            current_state = match current_state {
                Some(State::Work) => Some(State::Break),
                Some(State::Break) => Some(State::Work),
                None => Some(State::Work),
            };

            let work_duration = work_time * 60;
            let break_duration = break_time * 60;
            let mut speech = tts::Tts::new(tts::Backends::SpeechDispatcher).unwrap();
            // Handles timer and speech synthesis
            match current_state {
                Some(State::Work) => {
                    println!("Starting work period!");
                    timer_aux(work_duration);
                    speech.speak("Work period over", false).unwrap();
                }
                Some(State::Break) => {
                    println!("Starting break period!");
                    timer_aux(break_duration);
                    speech.speak("Break period over", false).unwrap();
                }
                None => (),
            }
        }
    }
    /// Auxiliary function used for the timer itself
    ///
    /// # Arguments
    /// * `duration` - An integer that represents the duration of the timer
    fn timer_aux(duration: u64) {
        let progress = ProgressBar::new(duration);
        progress.set_style(
            ProgressStyle::with_template("[{wide_bar}] [{elapsed_precise}]") // Creates a progress bar with the elapsed time to its right
                .unwrap()
                .progress_chars("##-"),
        );
        for _ in 0..duration {
            progress.inc(1);
            thread::sleep(Duration::from_secs(1));
        }
    }
}
