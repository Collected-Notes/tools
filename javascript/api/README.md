## Collected Notes CLI JavaScript Tool

This command-line tool is built in TypeScript to interact with the Site Management API. It allows you to manage sites and notes, perform CRUD operations, search notes, and retrieve content in different formats.

## Features

- List all sites
- Create, update, and delete sites
- List, create, update, and delete notes for a site
- Get note content in HTML, Markdown, or plain text format
- Search notes within a site
- Retrieve all links from a note

## Prerequisites

- **Node.js** (version 14 or higher).
- **npm** (Node Package Manager).
- An API token saved in a file named `.collected-notes` in your home directory. The token should be stored in plain text format.

### Environment Setup

1. **Install Node.js and npm**

   To install Node.js, follow the instructions for your platform from the official [Node.js installation guide](https://nodejs.org/).

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
   git clone https://github.com/your-username/collected-notes-cli-js.git
   cd collected-notes-cli-js
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

## Usage

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

### Development Mode

To run the tool in development mode using `localhost:3000` instead of the production server:

```bash
node dist/index.js --dev get-sites
```

## Error Handling

If you encounter errors, ensure that:

- Your API token is correctly stored in `~/.collected-notes`.
- Your command syntax is correct.

## Contributing

Contributions are welcome! Please fork the repository and create a pull request with your changes.

## License

This project is licensed under the MIT License.

### Summary of Changes

- **Changed the language** to TypeScript instead of Go.
- **Updated prerequisites** to reflect the requirements for Node.js, npm, and TypeScript.
- **Replaced Go commands** with appropriate `node` commands to run the TypeScript tool.
- **Included instructions for development mode** using the `--dev` flag.
- **Adjusted build instructions** for installing dependencies and compiling TypeScript code.