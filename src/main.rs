mod app;
mod core;
mod ui;

use clap::Parser;
use std::io;

use app::App;

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
    println!("From: {}, Into: {}", args.from, args.into);

    let mut terminal = ratatui::init();
    let app_result = App::new(args.from, args.into).run(&mut terminal);
    ratatui::restore();
    app_result
}
