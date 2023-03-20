use std::{
    error,
    fs::{self, File},
    io::Write,
    path::Path,
};

use api::PageInfo;
use clap::Parser;

mod api;
mod book;

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

    let api = api::new(&args.space, &args.apikey);
    let project = api.get_project(&args.project)?;
    let pages = api.get_entries(&args.project)?;

    println!("{:?}", pages);

    // create a mdbook directory
    book::create(dir_path.to_str().unwrap(), &project.name)?;

    println!("{}", dir_path.to_str().unwrap());

    let src_dir = dir_path.join("src");
    // remove all .md files
    for entry in fs::read_dir(&src_dir)? {
        let file_path = entry?.path();
        if let Some(ext) = file_path.extension() {
            if ext == "md" {
                fs::remove_file(file_path)?
            }
        }
    }

    let summary_content = build_summary(&pages);
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

fn build_summary(pages: &Vec<PageInfo>) -> String {
    let mut content = String::new();
    content.push_str("# SUMMARY\n\n");
    for page in pages {
        content.push_str(format!("- [{}]({}.md)\n", page.name, page.id).as_str())
    }
    content
}
