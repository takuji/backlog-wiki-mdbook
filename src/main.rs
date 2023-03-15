use std::{
    error,
    fs::{self, File},
    io::Write,
    path::Path,
};

use clap::Parser;

mod wiki;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    apikey: String,

    #[arg(short, long)]
    space: String,

    #[arg(short, long)]
    project: String,

    #[arg(short, long)]
    dir: String,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();
    let dir_path = Path::new(args.dir.as_str());
    fs::create_dir_all(&dir_path)?;
    println!("{}", dir_path.to_str().unwrap());
    let api = wiki::Wiki::new(&args.space, &args.apikey);
    let pages = api.get_entries(&args.project)?;
    println!("{:?}", pages);
    for page_info in pages {
        let page = api.get_page(page_info.id)?;
        println!("{:?}", page);
        let file_path = dir_path.join(page.id.to_string());
        let mut file = File::create(file_path)?;
        file.write_all(page.content.as_bytes())?;
    }
    Ok(())
}
