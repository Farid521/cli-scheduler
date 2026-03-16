use std::{env};
use std::error::Error;

mod connection;
mod server;
pub struct SchedulerOptions {
    time_start: i16,
    time_end: i16,
    name: String,
    description: Option<String>,
}

impl SchedulerOptions {
    fn create_schedule(args: &[String]) -> Result<Self, Box<dyn Error>> {
        if args.len() < 4 {
            return Err("Not enough arguments".into());
        }

        let time_start: i16 = args[2].parse()?;
        let time_end = args[3].parse()?;
        let name = args[4].clone();

        Ok(Self { time_start, time_end, name, description: None})
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let commands_args: Vec<String> = env::args().collect();

    if commands_args.len() < 2 {
        return Err("no command option have been passed".into());
    }

    let command_option = &commands_args[1];

    if command_option == "sch" {
        let schedule = SchedulerOptions::create_schedule(&commands_args)?;
        println!("{} scheduled at: {} - {}", schedule.name, schedule.time_start, schedule.time_end);
    }
    else if command_option == "test_auth" {
        server::create_server()?;
        connection::login_redirect();
        
        // Keep the main thread alive to prevent server from dying
        println!("Server is running. Press Ctrl+C to stop.");
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }
    else if  command_option == "test_server"{
        server::create_server()?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Application error: {}", e);
    }
}