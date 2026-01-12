use chrono::{Duration, Local};
use notify_rust::Notification;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();
    let period = Duration::minutes(15); // arbitrary time between sips
    let sleep_duration = period.to_std()?;

    println!("{}", now.format("%I:%M %p"));
    println!("{:?}", sleep_duration);

    thread::sleep(sleep_duration);

    Notification::new()
        .summary("Drink water!")
        .body("Dumbass.")
        .icon("water-notify")
        .show()?;

    Ok(())
}
