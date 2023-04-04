use std::{
    error,
    fs::{self, File},
    io::Write,
    path::Path,
    process,
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

    if project.text_formatting_rule != "markdown" {
        eprintln!(
            "project {}'s text format rule is not 'markdown' ({}).",
            project.project_key, project.text_formatting_rule
        );
        process::exit(1);
    }
    let pages = api.get_entries(&args.project)?;

    // create a mdbook directory
    book::create(dir_path.to_str().unwrap(), &project.name)?;

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
        println!("- {}", page.name);
        let file_path = src_dir.join(format!("{}.md", page.id));
        let mut file = File::create(file_path)?;
        file.write_all(page.content.as_bytes())?;
        let attachments = api.get_attachments(page_info.id)?;
        for attachment in attachments {
            println!("  - {:?}", attachment);
        }
    }
    Ok(())
}

struct Node {
    id: Option<u32>,
    name: String,
    children: Vec<Node>,
}

fn build_summary(pages: &Vec<PageInfo>) -> String {
    let mut content = String::new();
    content.push_str("# SUMMARY\n\n");
    let mut tree = Vec::<Node>::new();
    for page in pages {
        let components: Vec<&str> = page.name.split("/").collect();
        build_tree(page, &mut tree, &components);
    }
    for node in &tree {
        let s = render_node(node, 0);
        content.push_str(&s);
    }
    content
}

fn render_node(node: &Node, level: usize) -> String {
    let mut content = String::new();
    if let Some(id) = node.id {
        let s = format!("{}- [{}]({}.md)\n", "  ".repeat(level), node.name, id);
        content.push_str(&s);
    }
    for child in &node.children {
        let s = render_node(&child, level + 1);
        content.push_str(&s);
    }
    content
}

fn build_tree(page: &PageInfo, tree: &mut Vec<Node>, components: &[&str]) {
    if components.is_empty() {
        return;
    }
    let is_leaf = components.len() == 1;
    let el = components[0];
    let node_opt = tree.iter_mut().find(|e| e.name == el);
    if is_leaf {
        match node_opt {
            Some(node) => node.id = Some(page.id),
            None => tree.push(Node {
                id: Some(page.id),
                name: page.name.to_string(),
                children: Vec::new(),
            }),
        }
    } else {
        match node_opt {
            Some(node) => build_tree(page, &mut node.children, &components[1..]),
            None => {
                let mut nodes = Vec::<Node>::new();
                build_tree(page, &mut nodes, &components[1..]);
                let node = Node {
                    id: Some(page.id),
                    name: page.name.to_string(),
                    children: nodes,
                };
                tree.push(node);
            }
        };
    }
}
