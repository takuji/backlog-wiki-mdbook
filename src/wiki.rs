use serde::{Deserialize, Serialize};

pub struct Wiki {
    apikey: String,
    space: String,
}

impl Wiki {
    pub fn new(space: &String, apikey: &String) -> Wiki {
        Wiki {
            space: space.clone(),
            apikey: apikey.clone(),
        }
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

    pub fn get_page(&self, id: u64) -> Result<Page, Box<dyn std::error::Error>> {
        let url = format!(
            "https://{}/api/v2/wikis/{}?apiKey={}",
            self.space, id, self.apikey
        );
        let res = reqwest::blocking::get(url)?;
        let json: Page = res.json()?;
        Ok(json)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageInfo {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: u64,
    pub project_id: u64,
    pub name: String,
    pub content: String,
}
