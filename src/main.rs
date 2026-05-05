use std::error::Error;
use clap::{Parser, Subcommand};

mod errors;
mod browser;
mod server;
mod auth;
mod calendar;

#[derive(Parser)]
#[command(name = "cli-scheduler")]
#[command(version = "0.2.0")]
#[command(about = "CLI tool untuk mengelola jadwal di Google Calendar", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Login dan simpan token Google Calendar
    Auth,

    /// Tambahkan event baru ke Google Calendar
    Add {
        /// Judul event
        #[arg(short, long)]
        title: String,

        /// Waktu mulai (format: 2026-05-10T09:00)
        #[arg(short, long)]
        start: String,

        /// Waktu selesai (format: 2026-05-10T10:00)
        #[arg(short, long)]
        end: String,

        /// Deskripsi event (opsional)
        #[arg(short, long)]
        desc: Option<String>,
    },

    /// Tampilkan 10 event mendatang dari Google Calendar
    List,

    /// Hapus event berdasarkan ID
    Delete {
        /// ID event yang akan dihapus
        id: String,
    },
}

fn main() {
    dotenvy::dotenv().ok();
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    match cli.command {
        Commands::Auth => {
            auth::start_auth_flow()?;
        }

        Commands::Add { title, start, end, desc } => {
            let client = calendar::get_client()?;
            let event = calendar::create_event(
                &client,
                &title,
                &start,
                &end,
                desc.as_deref(),
            )?;
            println!("Event berhasil dibuat!");
            println!("  Judul : {}", event.summary);
            println!("  Mulai : {}", event.start.date_time.unwrap_or_default());
            println!("  Selesai: {}", event.end.date_time.unwrap_or_default());
            if let Some(id) = event.id {
                println!("  ID    : {}", id);
            }
        }

        Commands::List => {
            let client = calendar::get_client()?;
            let events = calendar::list_events(&client)?;

            if events.is_empty() {
                println!("Tidak ada event mendatang.");
            } else {
                println!("{:<40} {:<25} {}", "Judul", "Mulai", "ID");
                println!("{}", "-".repeat(90));
                for event in events {
                    let start = event.start.date_time.unwrap_or_else(|| event.start.date.unwrap_or_default());
                    let id = event.id.unwrap_or_else(|| "-".to_string());
                    println!("{:<40} {:<25} {}", event.summary, start, id);
                }
            }
        }

        Commands::Delete { id } => {
            let client = calendar::get_client()?;
            calendar::delete_event(&client, &id)?;
            println!("Event '{}' berhasil dihapus.", id);
        }
    }

    Ok(())
}