mod client;
use client::CollectedNotesClient;
use clap::{Parser, Subcommand};
use std::fs;
use std::error::Error;
use serde_json;

/// Simple CLI for interacting with CollectedNotes API
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Use development server (http://localhost:3000)
    #[arg(long)]
    dev: bool,

    /// Authentication token for the API
    #[arg(long)]
    token: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get all sites
    GetSites,
    
    /// Create a new site
    CreateSite {
        /// Site path
        site_path: String,
        /// Site name
        name: String,
    },
    
    /// Get a specific site
    GetSite {
        /// Site path
        site_path: String,
    },
    
    /// Update a specific site
    UpdateSite {
        /// Site path
        site_path: String,
        /// JSON data to update the site
        data: String,
    },
    
    /// Delete a specific site
    DeleteSite {
        /// Site path
        site_path: String,
    },
    
    /// Get all notes for a specific site
    GetNotesForSite {
        /// Site path
        site_path: String,
    },
    
    /// Create a note for a specific site
    CreateNoteForSite {
        /// Site path
        site_path: String,
        /// Note body
        body: String,
        /// Note visibility
        visibility: String,
    },
    
    /// Get a specific note by path
    GetNoteByPath {
        /// Site path
        site_path: String,
        /// Note path
        note_path: String,
    },
    
    /// Update a specific note by path
    UpdateNoteByPath {
        /// Site path
        site_path: String,
        /// Note path
        note_path: String,
        /// Note body
        body: String,
        /// Note visibility
        visibility: String,
    },
    
    /// Delete a specific note by path
    DeleteNoteByPath {
        /// Site path
        site_path: String,
        /// Note path
        note_path: String,
    },
    
    /// Get links from a specific note
    GetLinksFromNote {
        /// Site path
        site_path: String,
        /// Note path
        note_path: String,
    },
    
    /// Get the note body as HTML
    GetNoteBodyAsHtml {
        /// Site path
        site_path: String,
        /// Note path
        note_path: String,
    },
    
    /// Get the note as Markdown
    GetNoteAsMarkdown {
        /// Site path
        site_path: String,
        /// Note path
        note_path: String,
    },
    
    /// Get the note as plain text
    GetNoteAsPlainText {
        /// Site path
        site_path: String,
        /// Note path
        note_path: String,
    },
    
    /// Search notes
    SearchNotes {
        /// Site path
        site_path: String,
        /// Search term
        term: String,
        /// Search mode (default: "exact")
        #[arg(long, default_value = "exact")]
        mode: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // Determine the base URL based on the `--dev` flag
    let base_url = if cli.dev {
        "http://localhost:3000"
    } else {
        "https://api.collectednotes.com"
    };

    let token = cli.token.unwrap_or_else(|| {
        fs::read_to_string(dirs::home_dir().unwrap().join(".collected-notes"))
            .expect("Token not provided and could not read from ~/.collected-notes")
            .trim()
            .to_string()
    });

    let client = CollectedNotesClient::new(base_url, &token);

    match cli.command {
        Commands::GetSites => handle_response(client.get_sites().await).await,
        Commands::CreateSite { site_path, name } => handle_response(client.create_site(&site_path, &name).await).await,
        Commands::GetSite { site_path } => handle_response(client.get_site(&site_path).await).await,
        Commands::UpdateSite { site_path, data } => {
            let parsed_data: serde_json::Value = serde_json::from_str(&data)?;
            handle_response(client.update_site(&site_path, &parsed_data).await).await
        }
        Commands::DeleteSite { site_path } => handle_response(client.delete_site(&site_path).await).await,
        Commands::GetNotesForSite { site_path } => handle_response(client.get_notes_for_site(&site_path).await).await,
        Commands::CreateNoteForSite { site_path, body, visibility } => {
            handle_response(client.create_note_for_site(&site_path, &body, &visibility).await).await
        }
        Commands::GetNoteByPath { site_path, note_path } => handle_response(client.get_note_by_path(&site_path, &note_path).await).await,
        Commands::UpdateNoteByPath { site_path, note_path, body, visibility } => {
            handle_response(client.update_note_by_path(&site_path, &note_path, &body, &visibility).await).await
        }
        Commands::DeleteNoteByPath { site_path, note_path } => handle_response(client.delete_note_by_path(&site_path, &note_path).await).await,
        Commands::GetLinksFromNote { site_path, note_path } => handle_response(client.get_links_from_note(&site_path, &note_path).await).await,
        Commands::GetNoteBodyAsHtml { site_path, note_path } => handle_response(client.get_note_body_as_html(&site_path, &note_path).await).await,
        Commands::GetNoteAsMarkdown { site_path, note_path } => handle_response(client.get_note_as_markdown(&site_path, &note_path).await).await,
        Commands::GetNoteAsPlainText { site_path, note_path } => handle_response(client.get_note_as_plain_text(&site_path, &note_path).await).await,
        Commands::SearchNotes { site_path, term, mode } => handle_response(client.search_notes(&site_path, &term, &mode).await).await,
    }

    Ok(())
}

/// Helper function to handle responses and print them in JSON format
async fn handle_response<T: serde::Serialize>(result: Result<T, Box<dyn Error>>) {
    match result {
        Ok(data) => {
            println!("{}", serde_json::to_string_pretty(&data).unwrap());
        }
        Err(e) => {
            let error_message = serde_json::json!({
                "error": e.to_string()
            });
            println!("{}", serde_json::to_string_pretty(&error_message).unwrap());
        }
    }
}
