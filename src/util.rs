use std::{path::PathBuf, process::Command};

use directories::BaseDirs;

pub fn get_qp_dir() -> PathBuf {
	let base_dirs = BaseDirs::new().unwrap();
	let qp_dir = PathBuf::from(base_dirs.home_dir()).join("qp-nightly");

	qp_dir
}

pub fn run(cmd: &str, args: &[&str]) {
	Command::new(cmd)
		.args(args)
		.spawn()
		.unwrap()
		.wait()
		.unwrap();
}
