use dialoguer::{Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

fn main() {
    let selection = Select::new()
        .item("python3")
        .item("rust")
        .item("golang")
        .item("scala")
        .item("haskell")
        .default(0)
        .interact()
        .unwrap();

    println!("You selected: {}", selection);

    let input: String = Input::new()
        .with_prompt("Enter Project Name")
        .interact_text()
        .unwrap();

    bar_test();
    println!("Project: {} has been successfully initialised!", input);
}

fn bar_test() {

    let bar = ProgressBar::new(1000);
    bar.set_message("downloading files...");
    for _ in 0..1000 {
        bar.inc(1);
        std::thread::sleep(Duration::from_millis(5));
}
bar.finish();

let bar = ProgressBar::new_spinner();
    bar.set_message("verifying");
bar.enable_steady_tick(Duration::from_millis(100));

    for _ in 0..1000 {
        std::thread::sleep(Duration::from_millis(5));
// ... do some work
    //
}
bar.finish();
}
