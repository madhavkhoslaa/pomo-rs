use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, TextView, LinearLayout, SelectView};
use cursive::traits::*;

fn main() {
    let mut siv = cursive::default();
    siv.add_layer(Dialog::around(TextView::new("Pomodoro"))
        .button("Quit", |s| s.quit())
        .button("Start", start_pomodoro));
    siv.run();
}

fn build_root<'a>(siv: &'a mut Cursive, pomo: &str) -> &'a mut Cursive{
    siv.pop_layer();
    let select = SelectView::<String>::new()
        .on_submit(on_submit)
        .with_name("select");

    let buttons = LinearLayout::vertical()
        .child(Button::new("Add Task", add_task))
        .child(Button::new("Delete Task", delete_task))
        .child(Button::new("Start", start_timer))
        .child(Button::new("Pause", pause_timer))
        .child(Button::new("Reset timer",reset_timer))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));
    siv.add_layer(Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons)).title(pomo));
    siv
}

fn start_pomodoro<'a>(siv: &'a mut Cursive) -> (){
    let siv_start = build_root(siv,  "00:00");
    //siv_start
}

fn add_task(siv: &mut Cursive){}

fn on_submit(siv: &mut Cursive, name: &str){}

fn delete_task(siv: &mut Cursive){
}

fn reset_timer(siv: &mut Cursive){
    let start_timer_siv = build_root(siv, "Reset Pressed");
}
fn start_timer(siv: &mut Cursive){
    let start_timer_siv = build_root(siv, "Start Pressed");
}

fn pause_timer(siv: &mut Cursive){
    let start_timer_siv = build_root(siv, "Pause Pressed");
}
