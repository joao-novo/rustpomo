pub mod pomo {
    pub enum State {
        Work,
        Break,
    }

    /// Starts a timer in the desired state, work time and break time.
    ///
    /// # Arguments
    /// * `work_time` - A 64-bit unsigned integer that holds the duration of the work period in minutes
    /// * `break_time` - A 64-bit unsigned integer that holds the duration of the break period in minutes
    /// * `current_state` - A variant of the enum State that holds the current state of the timer (Work or Break)
    pub fn timer(work_time: u64, break_time: u64, current_state: State) {
        //TODO add timer functionality

        match current_state {
            State::Work => timer(work_time, break_time, State::Break),
            State::Break => timer(work_time, break_time, State::Work),
        }
    }
}
