use std::{error, fs::File, io::Write, path::Path};

use clap::Parser;

mod book;
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
    let book_dir = book::Book::new(dir_path.to_str().unwrap());
    book_dir.init()?;

    println!("{}", dir_path.to_str().unwrap());

    let api = wiki::Wiki::new(&args.space, &args.apikey);
    let pages = api.get_entries(&args.project)?;

    println!("{:?}", pages);

    let src_dir = dir_path.join("src");

    let mut summary_content = String::new();
    summary_content.push_str("# SUMMARY\n\n");
    for page_info in &pages {
        let v = format!("- [{}]({}.md)\n", page_info.name, page_info.id);
        summary_content.push_str(v.as_str());
    }
    let mut summary_file = File::create(src_dir.join("SUMMARY.md").as_path())?;
    summary_file.write_all(summary_content.as_bytes())?;

    for page_info in pages {
        let page = api.get_page(page_info.id)?;
        println!("{:?}", page);
        let file_path = src_dir.join(format!("{}.md", page.id));
        let mut file = File::create(file_path)?;
        file.write_all(page.content.as_bytes())?;
    }
    Ok(())
}
