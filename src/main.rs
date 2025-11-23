mod app;
mod core;
mod repo;
mod ui;

use clap::Parser;
use std::io;

use app::App;
use repo::Repo;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    from: String,
    #[arg(short, long)]
    into: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let repo = match Repo::new() {
        Ok(r) => r,
        Err(e) => panic!("Couldn't open repository: {}", e),
    };

    let mut terminal = ratatui::init();
    let app_result = App::new(repo, args.from, args.into)
        .run(&mut terminal);

    ratatui::restore();
    app_result
}
