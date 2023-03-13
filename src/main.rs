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
}

fn main() {
    let args = Args::parse();
    let api = wiki::Wiki::new(&args.space, &args.apikey);
    let res = api.get_entries(&args.project);
    match res {
        Ok(pages) => {
            println!("{:?}", pages);
            for page_info in pages {
                match api.get_page(page_info.id) {
                    Ok(page) => println!("{:?}", page),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Err(e) => println!("{:?}", e),
    }
}
