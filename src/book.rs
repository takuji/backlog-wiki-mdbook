use std::fs;

use mdbook::{Config, MDBook};

pub struct Book {
    dir: String,
}

impl Book {
    pub fn new(dir: &str) -> Book {
        Book {
            dir: String::from(dir),
        }
    }

    pub fn init(&self) -> Result<(), Box<dyn std::error::Error>> {
        fs::create_dir_all(&self.dir)?;
        let mut cfg = Config::default();
        cfg.book.title = Some("My Book".to_string());
        cfg.book.authors.push("Michael-F-Bryan".to_string());

        let mut builder = MDBook::init(&self.dir);
        builder.create_gitignore(true);
        builder.copy_theme(true);
        builder.with_config(cfg);
        builder.build()?;
        Ok(())
    }
}
