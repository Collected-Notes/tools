# Collected Notes CLI Tools

This repository provides a set of command-line interface (CLI) tools for interacting with the Collected Notes API. These utilities allow users to efficiently manage sites and notes on the Collected Notes platform.

**With these tools, you can:**

- Retrieve and manage site information
- Create, read, update, and delete notes
- Perform various other operations supported by the Collected Notes API

Whether you're a developer looking to integrate Collected Notes into your workflow or a power user seeking more control over your content, these CLI tools offer a flexible and powerful way to interact with the platform.

## Project Structure

This repo consists of the following projects:

1. Go CLI (`/go/cli/`)
   - Main CLI implementation in Go
   - Handles API requests and command processing
   - See `/go/cli/README.md` for build instructions and usage details
   - Allows you to build a binary that can be used to interact with the API

2. JavaScript/Node.js CLI (`/javascript/cli`)
   - Alternative CLI implementation in JavaScript
   - Offers additional features and flexibility
   - Refer to `/javascript/api/README.md` for setup and usage information
   - Easy integration with existing JavaScript projects.

3. Rust CLI (`/rust/cli`)
   - High-performance CLI implementation in Rust
   - Provides efficient handling of API interactions
   - Check `/rust/cli/README.md` for compilation instructions and usage guide
   - Allows you to build a binary that can be used to interact with the API

Each component has its own README file with specific instructions for building, installing, and using the respective CLI tool. Please refer to the individual README files for detailed information on each implementation.
