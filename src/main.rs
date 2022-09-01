use clap::Parser;

fn main() {
    let args = rust_pomodoro::InterfacePomodoro::parse();
    args.handle();
}
// % cargo run -- --work-time 10 --break-time 10 -- --tasks a b c
