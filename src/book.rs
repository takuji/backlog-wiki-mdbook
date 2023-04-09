use std::{
    fs,
    io::{self},
    path::Path,
};

use mdbook::{Config, MDBook};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BookError {
    #[error(" '{0}' is already used")]
    DirectoryExists(String),

    #[error("io error {source}")]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("mdbook error {source}")]
    MdBookError {
        #[from]
        source: mdbook::errors::Error,
    },
}

pub fn create(dir: &str, title: &str) -> Result<(), BookError> {
    let dir_path = Path::new(dir);
    if dir_path.exists() {
        return Err(BookError::DirectoryExists(dir.to_string()));
    }
    init(dir, title)?;
    Ok(())
}

fn init(dir: &str, title: &str) -> Result<(), BookError> {
    fs::create_dir_all(dir)?;
    let mut cfg = Config::default();
    cfg.book.title = Some(title.to_string());
    // cfg.book.authors.push("Michael-F-Bryan".to_string());

    let mut builder = MDBook::init(&dir);
    builder.create_gitignore(true);
    builder.copy_theme(true);
    builder.with_config(cfg);
    builder.build()?;
    Ok(())
}

pub fn build(dir: &str) -> Result<(), BookError> {
    let book = MDBook::load(&dir)?;
    book.build()?;
    Ok(())
}
