use enigo::{Coordinate, Enigo, Mouse, Settings};
use std::env;
use std::process;
use std::{thread, time::Duration};

fn move_mouse(interval: f64) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Mouse mover started with {} second interval. Press Ctrl+C to exit.",
        interval
    );

    // For enigo 0.6.1, you need to provide Settings
    let settings = Settings::default();
    let mut enigo = Enigo::new(&settings)?;

    ctrlc::set_handler(move || {
        println!("\nScript terminated by user.");
        process::exit(0);
    })?;

    loop {
        // Move mouse right by 50 pixels (relative movement)
        enigo.move_mouse(50, 0, Coordinate::Rel)?;
        thread::sleep(Duration::from_millis(100));

        // Move mouse left by 50 pixels (back to original position)
        enigo.move_mouse(-50, 0, Coordinate::Rel)?;

        // Sleep for the specified interval
        thread::sleep(Duration::from_secs_f64(interval));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <interval_in_seconds>", args[0]);
        eprintln!("Example: {} 30.0", args[0]);
        process::exit(1);
    }

    let interval: f64 = match args[1].parse() {
        Ok(val) if val > 0.0 => val,
        _ => {
            eprintln!("Error: Interval must be a positive number");
            process::exit(1);
        }
    };

    if let Err(e) = move_mouse(interval) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
