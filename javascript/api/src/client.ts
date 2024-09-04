import axios, { AxiosInstance } from 'axios';

export class CollectedNotesClient {
  private client: AxiosInstance;

  constructor(baseURL: string, token: string) {
    this.client = axios.create({
      baseURL,
      headers: {
        Authorization: `Bearer ${token}`,
        Accept: 'application/json',
      },
    });
  }

  // Get list of sites
  async getSites() {
    return this.client.get('/sites');
  }

  // Create a new site
  async createSite(sitePath: string, name: string) {
    return this.client.post('/sites', { site_path: sitePath, name });
  }

  // Get a site by path
  async getSite(sitePath: string) {
    return this.client.get(`/sites/${sitePath}`);
  }

  // Update a site
  async updateSite(sitePath: string, data: { site_path: string; name: string; headline?: string; about?: string; domain?: string }) {
    return this.client.put(`/sites/${sitePath}`, data);
  }

  // Delete a site
  async deleteSite(sitePath: string) {
    return this.client.delete(`/sites/${sitePath}`);
  }

  // Get notes for a site
  async getNotesForSite(sitePath: string) {
    return this.client.get(`/sites/${sitePath}/notes`);
  }

  // Create a note for a site
  async createNoteForSite(sitePath: string, body: string, visibility: string) {
    return this.client.post(`/sites/${sitePath}/notes`, { body, visibility });
  }

  // Get a note by path
  async getNoteByPath(sitePath: string, notePath: string) {
    return this.client.get(`/sites/${sitePath}/notes/${notePath}`);
  }

  // Update a note
  async updateNoteByPath(sitePath: string, notePath: string, body: string, visibility: string) {
    return this.client.put(`/sites/${sitePath}/notes/${notePath}`, { body, visibility });
  }

  // Delete a note
  async deleteNoteByPath(sitePath: string, notePath: string) {
    return this.client.delete(`/sites/${sitePath}/notes/${notePath}`);
  }

  // Get links from a note
  async getLinksFromNote(sitePath: string, notePath: string) {
    return this.client.get(`/sites/${sitePath}/notes/${notePath}/links`);
  }

  // Get the body of a note as HTML
  async getNoteBodyAsHTML(sitePath: string, notePath: string) {
    return this.client.get(`/sites/${sitePath}/notes/${notePath}/body`);
  }

  // Get the note in Markdown format
  async getNoteAsMarkdown(sitePath: string, notePath: string) {
    return this.client.get(`/sites/${sitePath}/notes/${notePath}.md`, { headers: { Accept: 'text/plain' } });
  }

  // Get the note in plain text format
  async getNoteAsPlainText(sitePath: string, notePath: string) {
    return this.client.get(`/sites/${sitePath}/notes/${notePath}.txt`, { headers: { Accept: 'text/plain' } });
  }

  // Search notes within a site
  // Search notes within a site
  async searchNotes(sitePath: string, term: string, mode: 'exact' | 'semantic' = 'exact') {
    return this.client.get(`/sites/${sitePath}/notes/search`, { params: { term, mode: mode || 'exact' } });
  }
}
