use clap::Parser;
use std::{thread, time};
use notify_rust::*;
use symlink;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct InterfacePomodoro {
    /// This argument takes in the work_time
    #[clap(short, long)]
    pub work_time: u8,

    /// This argument takes in the break_time
    #[clap(short, long)]
    pub break_time: u8,

    /// This argument takes in a list of comma seperated tasks
    #[clap()]
    pub block_distraction: bool,

    /// This argument takes in a list of comma seperated tasks
    pub tasks: Vec<String>,

    #[clap(short)]
    pub cycle: u8,
}

//pub struct Handler {
//    handle: u8,
//    next: Option<Handler>,
//}

impl InterfacePomodoro {
    pub fn handle(&self) {
        let work_millis = time::Duration::from_secs((&self.work_time*60).into());
        let break_millis = time::Duration::from_secs((&self.break_time*60).into());
        let mut current:  u8 = 0;
        loop {

            if self.cycle != 0 && current > 0 {
                println!("cycle  {} of {} started", current,  self.cycle);
            }
            println!("{:?}", self);
            // sends the notification
            println!("has started");
            handlers::alerts::start(&self);
            // blocks list
            handlers::hosts::block(&self);
            // displayes the list of tasks
            //  function(show tasks)

            thread::sleep(work_millis/2);
            println!("mid");
            // thread sleep for half time
            // thread.sleep()
            handlers::alerts::during(&self);
            thread::sleep(work_millis/2);
            println!("end");

            // thread sleep for next half time
            // thread.sleep(second half)
            println!("break starts");
            handlers::hosts::block(&self);
            thread::sleep(break_millis);

            // lifecycle ends 
            handlers::alerts::end(&self);
            // display that break has started
            // tell if end o cycles left
            current = current + 1;
            if current == self.cycle {
                break;
            }
        }

    }
}

//cargo run -- --work-time 10 --break-time 5 -c 0 -- --block-distractions a b c
//InterfacePomodoro { work_time: 10, break_time: 5, block_distraction: true, tasks: ["a", "b", "c"], cycle: 0 }

// Pomodoro Lifetime
// Start
// During
// End
//
// At Start:
// 1. Initial Notification is given on the desktop
// 2. The /etc/hosts is overriden by the symlink
// 3. Either triggered by the command or the program itself after the break
//
// During:
// 1. Mid time the work time a notification is given to stay on track
//
// End:
// 1. SymLink is broken
// 2. notification is given to the user

// Create time based chain of responsibility for lifecycle of a pomodoro
// Ask for pomodoro cycles, default is 1.
// Start =>  Thread.sleep(work_time/2) + During Notification + thead.sleep/2 -> End(if more cycles
// then loop)
mod handlers {
    pub mod alerts {
        pub fn start(args: &crate::InterfacePomodoro) -> Result<(), Box<dyn std::error::Error>> {
            // This gives a persistent notification till the time yo are working
            // To remind you to not waste time
            let body = format!(
                "You need to focus on your work for next {} minutes.",
                args.work_time
            );
            crate::Notification::new()
                .summary("Your Pomodoro started")
                .body(&body)
                .hint(crate::Hint::Resident(true))
                .timeout(crate::Timeout::Never)
                .show()?;
            Ok(())
        }

        pub fn during(args: &crate::InterfacePomodoro) -> Result<(), Box<dyn std::error::Error>> {
            let body = format!(
                "You need to focus on your work for next {} minutes.",
                args.work_time / 2
            );
            crate::Notification::new()
                .summary("You are half way through ! Keep on the good work")
                .body(&body)
                .hint(crate::Hint::Resident(true))
                .timeout(crate::Timeout::Never)
                .show()?;
            Ok(())
        }
        pub fn end(args: &crate::InterfacePomodoro) -> Result<(), Box<dyn std::error::Error>> {
            let body = format!("Your timer for {} minutes, is complete", args.work_time);
            crate::Notification::new()
                .summary("You have completed the pomodoro.")
                .body(&body)
                .hint(crate::Hint::Resident(true))
                .timeout(crate::Timeout::Never)
                .show()?;
            Ok(())
        }
    }
    pub mod hosts {
        pub fn block(args: &crate::InterfacePomodoro) -> Result<(), Box<dyn std::error::Error>> {
            if args.block_distraction {
                let _result = crate::symlink::symlink_file("/etc/hosts", "");
            }
            Ok(())
        }
        pub fn unblock(args: &crate::InterfacePomodoro) -> Result<(), Box<dyn std::error::Error>> {
            let _result = crate::symlink::remove_symlink_dir("/etc/hosts");
            Ok(())
        }
    }
}

pub mod logging {

    #[derive(Debug)]
    pub struct Log<'a> {
        text: &'a str,
        event: &'a str,
        time: usize,
    }
    // This is done to log your pomodoro sessions and the tasks that you did dring them
    pub fn log(args: Log) {
        println!("logging is to be implemented for {:?}", args);
    }
}
