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

pub fn run_cmd(cmd: &str, args: &[&str]) -> bool {
	let status = Command::new(cmd)
		.args(args)
		.spawn()
		.unwrap()
		.wait()
		.unwrap();

	match status.code() {
		Some(code) => {
			if code == 0 {
				true
			} else {
				false
			}
		}
		None => false,
	}
}

pub fn run_cmd_silent(cmd: &str, args: &[&str]) -> bool {
	let status = Command::new(cmd)
		.args(args)
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn()
		.unwrap()
		.wait()
		.unwrap();

	match status.code() {
		Some(code) => {
			if code == 0 {
				true
			} else {
				false
			}
		}
		None => false,
	}
}

pub fn assert_command(cmd: &str, args: &[&str]) {
	let a = Command::new(cmd)
		.args(args)
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.spawn();

	match a {
		Err(_) => {
			eprintln!("Failed to run command: {cmd}");
			exit(1);
		}
		Ok(mut b) => match &b.wait() {
			Err(_) => {
				eprintln!("Failed to run command: {cmd}");
				exit(1);
			}
			Ok(_) => {}
		},
	}
}
