use chrono::{Duration, Local};
use notify_rust::Notification;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let period = Duration::minutes(15);
    let sleep_duration = period.to_std()?;

    println!("Reminder period set to every {:?}", sleep_duration);

    loop {
        thread::sleep(sleep_duration);

        let now = Local::now();

        if let Err(e) = Notification::new()
            .summary("Drink water!")
            .body("Dumbass.")
            .icon("drink-water-reminder")
            .show()
        {
            eprintln!(
                "Push notification at {} failed! Error: {}", now.format("%H:%M:%S"), e
            );
        } else {
            println!("Notification sent at {}", now.format("%H:%M:%S"));
        }
    }
}
