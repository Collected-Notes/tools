use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Site {
    pub id: i32,
    pub name: String,
    pub site_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub visibility: String,
}

pub struct CollectedNotesClient {
    client: Client,
    base_url: String,
    token: String,
}

impl CollectedNotesClient {
    pub fn new(base_url: &str, token: &str) -> Self {
        CollectedNotesClient {
            client: Client::new(),
            base_url: base_url.to_string(),
            token: token.to_string(),
        }
    }

    pub async fn get_sites(&self) -> Result<Vec<Site>, Box<dyn Error>> {
        let url = format!("{}/sites", self.base_url);
        let response = self.client.get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?;
        
        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            if body.trim().is_empty() {
                Ok(vec![])
            } else {
                Ok(serde_json::from_str(&body)?)
            }
        } else {
            Err(format!("HTTP error: {}. Body: {}", status, body).into())
        }
    }

    pub async fn create_site(&self, site_path: &str, name: &str) -> Result<Site, Box<dyn Error>> {
        let url = format!("{}/sites", self.base_url);
        let response = self.client.post(&url)
            .bearer_auth(&self.token)
            .json(&serde_json::json!({ "site_path": site_path, "name": name }))
            .send()
            .await?;
        
        let status = response.status();
        let body = response.text().await?;

        if status.is_success() {
            if body.trim().is_empty() {
                Err("Empty response body".into())
            } else {
                Ok(serde_json::from_str(&body)?)
            }
        } else {
            Err(format!("HTTP error: {}. Body: {}", status, body).into())
        }
    }

    pub async fn get_site(&self, site_path: &str) -> Result<Site, Box<dyn Error>> {
        let url = format!("{}/sites/{}", self.base_url, site_path);
        let response = self.client.get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<Site>()
            .await?;
        Ok(response)
    }

    pub async fn update_site(&self, site_path: &str, data: &serde_json::Value) -> Result<Site, Box<dyn Error>> {
        let url = format!("{}/sites/{}", self.base_url, site_path);
        let response = self.client.put(&url)
            .bearer_auth(&self.token)
            .json(data)
            .send()
            .await?
            .json::<Site>()
            .await?;
        Ok(response)
    }

    pub async fn delete_site(&self, site_path: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/sites/{}", self.base_url, site_path);
        self.client.delete(&url)
            .bearer_auth(&self.token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_notes_for_site(&self, site_path: &str) -> Result<Vec<Note>, Box<dyn Error>> {
        let url = format!("{}/sites/{}/notes", self.base_url, site_path);
        let response = self.client.get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<Vec<Note>>()
            .await?;
        Ok(response)
    }

    pub async fn create_note_for_site(&self, site_path: &str, body: &str, visibility: &str) -> Result<Note, Box<dyn Error>> {
        let url = format!("{}/sites/{}/notes", self.base_url, site_path);
        let response = self.client.post(&url)
            .bearer_auth(&self.token)
            .json(&serde_json::json!({ "body": body, "visibility": visibility }))
            .send()
            .await?
            .json::<Note>()
            .await?;
        Ok(response)
    }

    pub async fn get_note_by_path(&self, site_path: &str, note_path: &str) -> Result<Note, Box<dyn Error>> {
        let url = format!("{}/sites/{}/notes/{}", self.base_url, site_path, note_path);
        let response = self.client.get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<Note>()
            .await?;
        Ok(response)
    }

    pub async fn update_note_by_path(&self, site_path: &str, note_path: &str, body: &str, visibility: &str) -> Result<Note, Box<dyn Error>> {
        let url = format!("{}/sites/{}/notes/{}", self.base_url, site_path, note_path);
        let response = self.client.put(&url)
            .bearer_auth(&self.token)
            .json(&serde_json::json!({ "body": body, "visibility": visibility }))
            .send()
            .await?
            .json::<Note>()
            .await?;
        Ok(response)
    }

    pub async fn delete_note_by_path(&self, site_path: &str, note_path: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/sites/{}/notes/{}", self.base_url, site_path, note_path);
        self.client.delete(&url)
            .bearer_auth(&self.token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_links_from_note(&self, site_path: &str, note_path: &str) -> Result<Vec<String>, Box<dyn Error>> {
        let url = format!("{}/sites/{}/notes/{}/links", self.base_url, site_path, note_path);
        let response = self.client.get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<Vec<String>>()
            .await?;
        Ok(response)
    }

    pub async fn get_note_body_as_html(&self, site_path: &str, note_path: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/sites/{}/notes/{}/body", self.base_url, site_path, note_path);
        let response = self.client.get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub async fn get_note_as_markdown(&self, site_path: &str, note_path: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/sites/{}/notes/{}.md", self.base_url, site_path, note_path);
        let response = self.client.get(&url)
            .bearer_auth(&self.token)
            .header("Accept", "text/plain")
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub async fn get_note_as_plain_text(&self, site_path: &str, note_path: &str) -> Result<String, Box<dyn Error>> {
        let url = format!("{}/sites/{}/notes/{}.txt", self.base_url, site_path, note_path);
        let response = self.client.get(&url)
            .bearer_auth(&self.token)
            .header("Accept", "text/plain")
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }

    pub async fn search_notes(&self, site_path: &str, term: &str, mode: &str) -> Result<Vec<Note>, Box<dyn Error>> {
        let url = format!("{}/sites/{}/notes/search", self.base_url, site_path);
        let response = self.client.get(&url)
            .bearer_auth(&self.token)
            .query(&[("term", term), ("mode", mode)])
            .send()
            .await?
            .json::<Vec<Note>>()
            .await?;
        Ok(response)
    }
}