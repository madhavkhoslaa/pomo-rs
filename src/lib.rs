struct InterfacePomodoro {
    pub work_time_minutes: u8,
    pub break_time_minutes: u8,
    pub tasks: Vec<String>
}

pub mod PomodoroCRUD {
    impl crate::InterfacePomodoro {
    pub fn new(work_time_minutes: u8, break_time_minutes: u8, tasks: Vec<String>) -> crate::InterfacePomodoro {
        crate::InterfacePomodoro {
            work_time_minutes,
            break_time_minutes,
            tasks,
        }
    }

    pub fn update_break_time(&mut self, break_time_minutes: u8){
        self.break_time_minutes = break_time_minutes;
    }
    pub fn update_work_time(&mut self, work_time_minutes: u8){
        self.work_time_minutes = work_time_minutes;
    }
}
}

mod block_distraction {
    // Blocks distracting apps using /etc/hosts
    pub fn block(){}
    pub fn unblock(){}
}
mod alerts { 
    pub fn alert(){}
    pub fn progress_bar(){}
    pub fn minimal(){}
    pub fn motivate(){}
}

mod tui {
    pub fn time_input_screen(){}
    pub fn goals_input_screen(){}
    pub fn focus_screen(){}
    pub fn update_settings_screen(){}
}
