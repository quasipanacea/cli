use std::{env, process::exit};

use crate::util::{run_cmd, run_cmd_silent};

pub fn build() {
	let current_dir = env::current_dir().unwrap();

	if current_dir.file_name().unwrap() == "cli" {
		env::set_current_dir(current_dir.parent().unwrap()).unwrap()
	}

	let current_dir = env::current_dir().unwrap();

	if current_dir.file_name().unwrap() != "repos" {
		eprintln!("Most be in directory: init/repos");
		exit(1);
	}

	if ! run_cmd_silent(
		"bash",
		&["-c", "cd ./common && git diff-index --quiet HEAD -- && git ls-files --exclude-standard --others"],
	) {
		eprintln!("Unstaged and untracked changes in 'common'. Exiting");
		exit(1);
	}
	if ! run_cmd_silent(
		"bash",
		&["-c", "cd ./client-web && git diff-index --quiet HEAD -- && git ls-files --exclude-standard --others"],
	) {
		eprintln!("Unstaged and untracked changes in 'client-web'. Exiting");
		exit(1);
	}
	if ! run_cmd_silent(
		"bash",
		&["-c", "cd ./server-deno && git diff-index --quiet HEAD -- && git ls-files --exclude-standard --others"],
	) {
		eprintln!("Unstaged and untracked changes in 'server-deno'. Exiting");
		exit(1);
	}

	run_cmd("bash", &["-c", "cd ./common && ./bake release-nightly"]);
	run_cmd("bash", &["-c", "cd ./client-web && ./bake release-nightly"]);
	run_cmd(
		"bash",
		&["-c", "cd ./server-deno && ./bake release-nightly"],
	);
}
