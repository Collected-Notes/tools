import * as fs from 'fs';
import * as path from 'path';
import { CollectedNotesClient } from './client';

// Determine base URL based on the --dev flag
const isDev = process.argv.includes('--dev');
const host = isDev ? 'http://localhost:3000' : 'https://api.collectednotes.com';

// Remove the --dev argument if it exists so it doesn't interfere with command parsing
const args = process.argv.filter(arg => arg !== '--dev');

// Read the API token from the file
const getApiToken = (): string | null => {
  try {
    const tokenPath = path.resolve(process.env.HOME || '', '.collected-notes');
    const token = fs.readFileSync(tokenPath, 'utf8').trim();
    return token || null;
  } catch (err) {
    console.error('Error reading API token from ~/.collected-notes:', err);
    return null;
  }
};

// Main function to handle CLI commands
const main = async () => {
  const token = getApiToken();
  if (!token) {
    console.error('API token not found. Please ensure the token is stored in ~/.collected-notes');
    return;
  }

  const client = new CollectedNotesClient(host, token);
  const [, , command, ...commandArgs] = args;

  try {
    switch (command) {
      case 'get-sites':
        const sites = await client.getSites();
        console.log(JSON.stringify(sites.data, null, 2));
        break;
      case 'get-site':
        if (commandArgs.length < 1) {
          console.error('Usage: get-site <site_path>');
          return;
        }
        const site = await client.getSite(commandArgs[0]);
        console.log(JSON.stringify(site.data, null, 2));
        break;
      case 'create-site':
        if (commandArgs.length < 2) {
          console.error('Usage: create-site <site_path> <name>');
          return;
        }
        const newSite = await client.createSite(commandArgs[0], commandArgs[1]);
        console.log(JSON.stringify(newSite.data, null, 2));
        break;
      case 'search-notes':
        if (commandArgs.length < 2) {
          console.error('Usage: search-notes <site_path> <term> [mode]');
          return;
        }
        const sitePath = commandArgs[0];
        const term = commandArgs[1];
        const mode = commandArgs[2] || 'exact'; // Default to 'exact' mode if not provided
        const searchResults = await client.searchNotes(sitePath, term, mode as 'exact' | 'semantic');
        console.log(JSON.stringify(searchResults.data, null, 2));
        break;
      // Add more cases for different commands
      default:
        console.error(`Unknown command: ${command}`);
        console.log('Available commands: get-sites, get-site, create-site, search-notes');
    }
  } catch (error: any) { // Cast 'error' to 'any' to access its properties
    if (error.response) {
      console.error('Error:', error.response.data);
    } else {
      console.error('Error:', error.message);
    }
  }
};

main();