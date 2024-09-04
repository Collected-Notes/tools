# Collected Notes CLI Tool

This command-line tool is built in Go to interact with the Site Management API. It allows you to manage sites and notes, perform CRUD operations, search notes, and retrieve content in different formats.

## Features

- List all sites
- Create, update, and delete sites
- List, create, update, and delete notes for a site
- Get note content in HTML, Markdown, or plain text format
- Search notes within a site
- Retrieve all links from a note

## Prerequisites

- Go (Golang) installed on your machine (version 1.16 or higher).
- An API token saved in a file named `.collected-notes` in your home directory. The token should be stored in a plain text format.

### Environment Setup

1. **Install Go**

   To install Go, follow the instructions for your platform from the official [Go installation guide](https://golang.org/doc/install).

2. **Set up API Token**

   Ensure your API token is stored in `~/.collected-notes`:

   ```bash
   echo "your-api-token" > ~/.collected-notes
   ```

   Replace `your-api-token` with your actual API token.

## Installation

1. **Clone the Repository**

   Clone this repository to your local machine:

   ```bash
   git clone https://github.com/your-username/collected-notes-cli-go.git
   cd collected-notes-cli-go
   ```

2. **Build the Tool**

   Compile the Go code to create an executable:

   ```bash
   go build -o collected-notes-cli
   ```

   This command will generate an executable named `collected-notes-cli` in your current directory.

## Usage

Run the tool using the following commands:

### General Commands

- **Get all sites:**

  ```bash
  ./collected-notes-cli get-sites
  ```

- **Get a specific site by path:**

  ```bash
  ./collected-notes-cli get-site <site_path>
  ```

  Replace `<site_path>` with the path of the site you want to retrieve.

- **Create a new site:**

  ```bash
  ./collected-notes-cli create-site <site_path> <name>
  ```

  Replace `<site_path>` with the desired path and `<name>` with the name of the new site.

- **Update a site:**

  ```bash
  ./collected-notes-cli update-site <site_path> <name> [headline] [about] [domain]
  ```

  Replace `<site_path>`, `<name>`, and optionally `[headline]`, `[about]`, `[domain]` with the updated information.

- **Delete a site:**

  ```bash
  ./collected-notes-cli delete-site <site_path>
  ```

  Replace `<site_path>` with the path of the site to delete.

### Note Commands

- **Get all notes for a site:**

  ```bash
  ./collected-notes-cli get-notes <site_path>
  ```

- **Create a new note for a site:**

  ```bash
  ./collected-notes-cli create-note <site_path> <body> <visibility>
  ```

  Replace `<site_path>`, `<body>`, and `<visibility>` with the site path, note body, and visibility status (`public` or `private`).

- **Get a specific note by path:**

  ```bash
  ./collected-notes-cli get-note <site_path> <note_path>
  ```

- **Update a note:**

  ```bash
  ./collected-notes-cli update-note <site_path> <note_path> <body> <visibility>
  ```

- **Delete a note:**

  ```bash
  ./collected-notes-cli delete-note <site_path> <note_path>
  ```

### Additional Commands

- **Get all links from a note:**

  ```bash
  ./collected-notes-cli get-links-from-note <site_path> <note_path>
  ```

- **Get the body of a note as HTML:**

  ```bash
  ./collected-notes-cli get-note-body-as-html <site_path> <note_path>
  ```

- **Get the note in Markdown format:**

  ```bash
  ./collected-notes-cli get-note-as-markdown <site_path> <note_path>
  ```

- **Get the note in plain text format:**

  ```bash
  ./collected-notes-cli get-note-as-plaintext <site_path> <note_path>
  ```

- **Search notes within a site:**

  ```bash
  ./collected-notes-cli search-notes <site_path> <term> [mode]
  ```

  Replace `<site_path>`, `<term>`, and optionally `[mode]` with the site path, search term, and search mode (`exact` or `semantic`).

## Error Handling

If you encounter errors, ensure that:

- Your API token is correctly stored in `~/.collected-notes`.
- Your command syntax is correct.

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes.

## License

This project is licensed under the MIT License.