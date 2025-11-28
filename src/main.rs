mod app;
mod repo;
mod state;
mod ui;

use clap::Parser;
use std::env;
use std::fs::File;
use std::io;
use tracing_subscriber::{filter::EnvFilter, fmt::{self, writer::BoxMakeWriter}, layer::SubscriberExt, util::SubscriberInitExt};

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
    if let Ok(log_level) = env::var("RUST_LOG") && log_level == "debug" {
        init_logging();
    }

    let args = Args::parse();

    let repo = match Repo::new() {
        Ok(r) => r,
        Err(e) => panic!("Couldn't open repository: {}", e),
    };

    let mut terminal = ratatui::init();
    let mut app = match App::new(repo, args.from, args.into) {
        Ok(a) => a,
        Err(e) => panic!("{e}"),
    };

    let result = app.run(&mut terminal);

    ratatui::restore();

    result
}

fn init_logging() {
    let file = File::create("debug.log").expect("could not create log file");

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(
            fmt::layer()
                .with_writer(BoxMakeWriter::new(file))
                .with_ansi(false)
        )
        .init();
}
