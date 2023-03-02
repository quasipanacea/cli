use std::{
	path::PathBuf,
	process::{exit, Command, Stdio},
};

use directories::BaseDirs;

pub fn get_qp_dir() -> PathBuf {
	let base_dirs = BaseDirs::new().unwrap();
	let qp_dir = PathBuf::from(base_dirs.home_dir()).join("qp-nightly");

	qp_dir
}

pub fn assert_command(cmd: &str, args: &[&str]) {
	let exists = match Command::new(cmd)
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()
	{
		Ok(_) => true,
		Err(err) => {
			if let NotFound = err.kind() {
				false
			} else {
				panic!("{}", err);
			}
		}
	};

	if !exists {
		eprintln!("{}", String::from("Command not found: ") + cmd);
		exit(1);
	}
}

pub fn run(cmd: &str, args: &[&str]) {
	Command::new(cmd)
		.args(args)
		.spawn()
		.unwrap()
		.wait()
		.unwrap();
}
