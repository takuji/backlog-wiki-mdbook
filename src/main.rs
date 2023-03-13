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
    let api = wiki::Wiki::new(&args.apikey);
    let res = api.get_entries(&args.space, &args.project);
    match res {
        Ok(text) => println!("{:?}", text),
        Err(e) => println!("{:?}", e),
    }
}
