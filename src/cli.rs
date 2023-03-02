use clap;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
	#[command(subcommand)]
	pub action: Action,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
	Install {},
	Run {},
}
