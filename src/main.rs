use chrono::Duration;
use notify_rust::Notification;
use std::thread;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let period = Duration::minutes(15);
    let sleep_duration = period.to_std()?;

    println!("Water reminder has started.");
    println!("Reminder period set to every {:?}", sleep_duration);

    loop {
        thread::sleep(sleep_duration);

        if let Err(e) = Notification::new()
            .summary("Drink water!")
            .body("Dumbass.")
            .icon("drink-water-reminder")
            .show()
        {
            eprintln!("Push notification failed! Error: {}", e);
        } else {
            println!("Push notification sent!");
        }
    }
}
