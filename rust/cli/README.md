# Collected Notes CLI Tool (Rust)

The Collected Notes CLI Tool is a command-line interface for managing sites and notes using the Collected Notes API. It supports various operations such as listing, creating, updating, and deleting sites and notes, retrieving note content in different formats, and searching notes within a site.

## Prerequisites

- Rust installed on your machine (version 1.56 or higher).
- An API token for Collected Notes. Save this token in a file named `.collected-notes` in your home directory. The token should be stored in plain text format. Obtain your API token from the [Collected Notes API page](https://collectednotes.com/api).

### Setting Up the API Key

Ensure your API token is stored in `~/.collected-notes`:

```bash
echo "your-api-token" > ~/.collected-notes
```

Replace `your-api-token` with your actual API token.

## Building

1. **Clone the Repository**

   Clone this repository to your local machine:

   ```bash
   git clone https://github.com/Collected-Notes/tools
   cd tools/rust/cli
   ```

2. **Build the Tool**

   Compile the Rust code to create an executable:

   ```bash
   cargo build --release
   ```

   This command will generate an executable in the `target/release` directory.

## Command Line Tool Usage

Run the tool using the following commands:

### General Commands

- **Get all sites:**

  ```bash
  ./target/release/collected-notes-cli get-sites
  ```

- **Get a specific site by path:**

  ```bash
  ./target/release/collected-notes-cli get-site <site_path>
  ```

  Replace `<site_path>` with the path of the site you want to retrieve.

- **Create a new site:**

  ```bash
  ./target/release/collected-notes-cli create-site <site_path> <name>
  ```

  Replace `<site_path>` with the desired path and `<name>` with the name of the new site.

- **Update a site:**

  ```bash
  ./target/release/collected-notes-cli update-site <site_path> <name>
  ```

  Replace `<site_path>` and `<name>` with the updated information.

- **Delete a site:**

  ```bash
  ./target/release/collected-notes-cli delete-site <site_path>
  ```

  Replace `<site_path>` with the path of the site to delete.

### Note Commands

- **Get all notes for a site:**

  ```bash
  ./target/release/collected-notes-cli get-notes <site_path>
  ```

- **Create a new note for a site:**

  ```bash
  ./target/release/collected-notes-cli create-note <site_path> <body> <visibility>
  ```

  Replace `<site_path>`, `<body>`, and `<visibility>` with the site path, note body, and visibility status (`public` or `private`).

- **Get a specific note by path:**

  ```bash
  ./target/release/collected-notes-cli get-note <site_path> <note_path>
  ```

- **Update a note:**

  ```bash
  ./target/release/collected-notes-cli update-note <site_path> <note_path> <body> <visibility>
  ```

- **Delete a note:**

  ```bash
  ./target/release/collected-notes-cli delete-note <site_path> <note_path>
  ```

### Additional Commands

- **Get all links from a note:**

  ```bash
  ./target/release/collected-notes-cli get-links-from-note <site_path> <note_path>
  ```

- **Get the body of a note as HTML:**

  ```bash
  ./target/release/collected-notes-cli get-note-body-as-html <site_path> <note_path>
  ```

- **Get the note in Markdown format:**

  ```bash
  ./target/release/collected-notes-cli get-note-as-markdown <site_path> <note_path>
  ```

- **Get the note in plain text format:**

  ```bash
  ./target/release/collected-notes-cli get-note-as-plaintext <site_path> <note_path>
  ```

- **Search notes within a site:**

  ```bash
  ./target/release/collected-notes-cli search-notes <site_path> <term> [mode]
  ```

  Replace `<site_path>`, `<term>`, and optionally `[mode]` with the site path, search term, and search mode (`exact` or `semantic`).

## License

This project is licensed under the MIT License.