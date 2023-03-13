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
    ) -> Result<String, Box<dyn std::error::Error>> {
        //
        let url = format!(
            "https://{}/api/v2/wikis?apiKey={}&projectIdOrKey={}",
            space, self.apikey, project
        );
        let res = reqwest::blocking::get(url)?;
        let text = res.text()?;
        Ok(text)
    }
}
