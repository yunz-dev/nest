use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

fn main() {
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
        .progress_chars("#>-"));

    for i in 0..100 {
        pb.inc(1);
        pb.set_message(format!("Step {}", i + 1));
        std::thread::sleep(Duration::from_millis(50));
    }

    pb.finish_with_message("Done!");
}
