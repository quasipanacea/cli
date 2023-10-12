use std::{os::unix::process::CommandExt, process::Command};

use crate::util::{assert_command, get_qp_dir};

pub fn run() {
	let qp_dir = get_qp_dir();
	if !qp_dir.exists() {
		eprintln!("Please run subcommand 'install' first");
	}
	assert_command("deno", &["--version"]);

	let deno_file = qp_dir.join("server/output/build/bundle.js");
	let common_dir = qp_dir.join("common/common-nightly");
	let public_dir = qp_dir.join("client-web/output/dist");

	Command::new("deno")
		.args(["run", "--allow-all", deno_file.to_str().unwrap()])
		.env("QP_COMMON", common_dir)
		.env("QP_PUBLIC", public_dir)
		.exec();
}
