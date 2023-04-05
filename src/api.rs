use std::{fs::File, io::Write, path::Path};

use serde::{Deserialize, Serialize};

pub struct BacklogApi {
    apikey: String,
    space: String,
}

pub fn new(space: &String, apikey: &String) -> BacklogApi {
    BacklogApi {
        space: space.clone(),
        apikey: apikey.clone(),
    }
}

impl BacklogApi {
    pub fn get_project(&self, key: &str) -> Result<Project, Box<dyn std::error::Error>> {
        let url = format!(
            "https://{}/api/v2/projects/{}?apiKey={}",
            self.space, key, self.apikey,
        );
        let res = reqwest::blocking::get(url)?;
        let json: Project = res.json()?;
        Ok(json)
    }

    pub fn get_entries(
        &self,
        project: &String,
    ) -> Result<Vec<PageInfo>, Box<dyn std::error::Error>> {
        //
        let url = format!(
            "https://{}/api/v2/wikis?apiKey={}&projectIdOrKey={}",
            self.space, self.apikey, project
        );
        let res = reqwest::blocking::get(url)?;
        let json: Vec<PageInfo> = res.json()?;
        Ok(json)
    }

    pub fn get_page(&self, id: u32) -> Result<Page, Box<dyn std::error::Error>> {
        let url = format!(
            "https://{}/api/v2/wikis/{}?apiKey={}",
            self.space, id, self.apikey
        );
        let res = reqwest::blocking::get(url)?;
        let json: Page = res.json()?;
        Ok(json)
    }

    /// Get attachments of a page
    pub fn get_attachments(&self, id: u32) -> Result<Vec<Attachment>, Box<dyn std::error::Error>> {
        // call https://<space>.backlog.com/api/v2/wikis/<id>/attachments
        let url = format!(
            "https://{}/api/v2/wikis/{}/attachments?apiKey={}",
            self.space, id, self.apikey
        );
        let res = reqwest::blocking::get(url)?;
        let json: Vec<Attachment> = res.json()?;
        Ok(json)
    }

    /// Download all the attachments of a page
    /// # Arguments
    /// * `page` - a page
    /// * `dir` - a directory to save attachments
    /// # Returns
    /// * `Vec<String>` - a list of file names
    /// # Errors
    /// * `std::io::Error` - if failed to create a file
    /// * `reqwest::Error` - if failed to download a file
    /// * `serde_json::Error` - if failed to parse a response
    pub fn download_all_attachments(
        &self,
        page: &Page,
        dir: &str,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut file_names = Vec::new();
        for attachment in &page.attachments {
            let file_name = self.download_attachment(page.id, attachment, dir)?;
            file_names.push(file_name);
        }
        Ok(file_names)
    }

    fn download_attachment(
        &self,
        page_id: u32,
        attachment: &Attachment,
        dir: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // call https://<space>.backlog.com/api/v2/wikis/<id>/attachments/<id>
        // and save the file to <dir>/<name>
        let url = format!(
            "https://{}/api/v2/wikis/{}/attachments/{}?apiKey={}",
            self.space, page_id, attachment.id, self.apikey
        );
        let res = reqwest::blocking::get(url)?;
        let file_path = Path::new(dir).join(&attachment.name);
        let mut file = File::create(&file_path)?;
        file.write_all(&res.bytes()?)?;
        Ok(file_path.to_string_lossy().to_string())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: u32,
    pub project_key: String,
    pub name: String,
    pub text_formatting_rule: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageInfo {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: u32,
    pub project_id: u64,
    pub name: String,
    pub content: String,
    pub attachments: Vec<Attachment>,
    pub shared_files: Vec<SharedFile>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Attachment {
    pub id: u32,
    pub name: String,
    pub size: usize,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SharedFile {
    id: u32,
    project_id: u32,
    #[serde(rename = "type")]
    file_type: String,
    dir: String,
    name: String,
    size: usize,
}
