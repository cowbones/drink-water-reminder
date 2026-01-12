use chrono::{Duration, Local};
use notify_rust::Notification;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = Local::now();
    let period = Duration::minutes(15); // arbitrary time between sips
    let sleep_duration = period.to_std()?;

    println!("Reminder period set to every {:?}", sleep_duration);

    println!("Waiting...");
    thread::sleep(sleep_duration);

    if let Err(e) = Notification::new()
        .summary("Drink water!")
        .body("Dumbass.")
        .icon("water-notify")
        .show()
    {
        println!("Push notification at {} failed! Error: {}", now.format("%I:%M %p"), e);
    } else {
        println!("Notification sent at {}", now.format("%I:%M %p"));
    }

    Ok(())
}
