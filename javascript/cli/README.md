# Collected Notes CLI Tool (JavaScript)

The Collected Notes CLI JavaScript Tool is a command-line interface for managing sites and notes using the Collected Notes API. It supports various operations such as listing, creating, updating, and deleting sites and notes, retrieving note content in different formats, and searching notes within a site.

## Prerequisites

- **Node.js** (version 14 or higher).
- **npm** (Node Package Manager).
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
   cd tools/js/cli
   ```

2. **Install Dependencies**

   Use `npm` to install the required dependencies:

   ```bash
   npm install
   ```

3. **Compile the TypeScript Code**

   Compile the TypeScript code to JavaScript:

   ```bash
   npx tsc
   ```

   This command will generate compiled JavaScript files in the `dist` directory.

## Command Line Tool Usage

Run the tool using the following commands:

### General Commands

- **Get all sites:**

  ```bash
  node dist/index.js get-sites
  ```

- **Get a specific site by path:**

  ```bash
  node dist/index.js get-site <site_path>
  ```

  Replace `<site_path>` with the path of the site you want to retrieve.

- **Create a new site:**

  ```bash
  node dist/index.js create-site <site_path> <name>
  ```

  Replace `<site_path>` with the desired path and `<name>` with the name of the new site.

- **Update a site:**

  ```bash
  node dist/index.js update-site <site_path> '{"site_path":"new_path","name":"new_name","headline":"new_headline","about":"new_about","domain":"new_domain"}'
  ```

  Replace `<site_path>` and the JSON object with the updated information.

- **Delete a site:**

  ```bash
  node dist/index.js delete-site <site_path>
  ```

  Replace `<site_path>` with the path of the site to delete.

### Note Commands

- **Get all notes for a site:**

  ```bash
  node dist/index.js get-notes <site_path>
  ```

- **Create a new note for a site:**

  ```bash
  node dist/index.js create-note <site_path> <body> <visibility>
  ```

  Replace `<site_path>`, `<body>`, and `<visibility>` with the site path, note body, and visibility status (`public` or `private`).

- **Get a specific note by path:**

  ```bash
  node dist/index.js get-note <site_path> <note_path>
  ```

- **Update a note:**

  ```bash
  node dist/index.js update-note <site_path> <note_path> <body> <visibility>
  ```

- **Delete a note:**

  ```bash
  node dist/index.js delete-note <site_path> <note_path>
  ```

### Additional Commands

- **Get all links from a note:**

  ```bash
  node dist/index.js get-links-from-note <site_path> <note_path>
  ```

- **Get the body of a note as HTML:**

  ```bash
  node dist/index.js get-note-body-as-html <site_path> <note_path>
  ```

- **Get the note in Markdown format:**

  ```bash
  node dist/index.js get-note-as-markdown <site_path> <note_path>
  ```

- **Get the note in plain text format:**

  ```bash
  node dist/index.js get-note-as-plaintext <site_path> <note_path>
  ```

- **Search notes within a site:**

  ```bash
  node dist/index.js search-notes <site_path> <term> [mode]
  ```

  Replace `<site_path>`, `<term>`, and optionally `[mode]` with the site path, search term, and search mode (`exact` or `semantic`).

## License

This project is licensed under the

 MIT License.
