use serde::{Deserialize, Serialize};

pub struct Wiki {
    apikey: String,
}

impl Wiki {
    pub fn new(apikey: &String) -> Wiki {
        Wiki {
            apikey: apikey.clone(),
        }
    }

    pub fn get_entries(
        &self,
        space: &String,
        project: &String,
    ) -> Result<Vec<Page>, Box<dyn std::error::Error>> {
        //
        let url = format!(
            "https://{}/api/v2/wikis?apiKey={}&projectIdOrKey={}",
            space, self.apikey, project
        );
        let res = reqwest::blocking::get(url)?;
        let json: Vec<Page> = res.json()?;
        Ok(json)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    id: u64,
    name: String,
}
