mod api;
mod data;
mod app;
mod ui;

use clap::{Parser, Subcommand};
use crate::data::quran;
use crate::app::App;
use ratatui::init;

#[derive(Parser)]
#[command(name = "quran")]
#[command(about = "A beautiful Quran TUI for developers", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Read a specific ayah or surah
    Read {
        /// Reference (e.g., 2:255 or "Al-Baqarah 255")
        reference: String,
    },
    /// Search the Quran
    Search {
        /// Query to search for
        query: String,
    },
    /// Get a random ayah
    Random,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Read { reference }) => {
            match data::reference::parse(&reference) {
                Ok(r) => {
                    if let Some(ayah_num) = r.ayah_num {
                        if let Some(ayah) = quran::get_ayah(r.surah_id, ayah_num) {
                            println!("[Surah {} Ayah {}] {}", r.surah_id, ayah_num, ayah.translation);
                        } else {
                            println!("Ayah {} not found in Surah {}", ayah_num, r.surah_id);
                        }
                    } else {
                        if let Some(surah) = quran::get_surah(r.surah_id) {
                            println!("Surah {}: {}", surah.id, surah.transliteration);
                            for ayah in &surah.verses {
                                println!("{}: {}", ayah.id, ayah.translation);
                            }
                        } else {
                            println!("Surah {} not found", r.surah_id);
                        }
                    }
                }
                Err(e) => println!("Error parsing reference: {}", e),
            }
        }
        Some(Commands::Search { query }) => {
            let results = quran::search(&query);
            if results.is_empty() {
                println!("No results found for '{}'", query);
            } else {
                for res in results {
                    println!("[{} {}:{}] {}", res.surah_name, res.surah_id, res.ayah_num, res.translation);
                }
            }
        }
        Some(Commands::Random) => {
            let ayah = quran::random_ayah();
            println!("[Surah {} Ayah {}] {}", ayah.surah_id, ayah.id, ayah.translation);
        }
        None => {
            let app = App::new();
            let terminal = init();
            app.run(terminal).await?;
            ratatui::restore();
        }
    }
    Ok(())
}
