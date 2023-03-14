use clap::Parser;

mod cli;
mod subcommands;
mod util;

use cli::{Action, Args};

fn main() {
	let cli = Args::parse();

	match cli.action {
		Action::Install {} => {
			subcommands::install();
		}
		Action::Run {} => {
			subcommands::run();
		}
		Action::BuildRelease {} => {
			subcommands::build();
		}
	}
}
