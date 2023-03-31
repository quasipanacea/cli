use std::{env, fs};

use crate::util::{get_qp_dir, run_cmd};

pub fn install() {
	let qp_dir = get_qp_dir();
	println!("Quasipanacea directory: {}", qp_dir.to_str().unwrap());

	if qp_dir.exists() {
		fs::remove_dir_all(&qp_dir).unwrap();
	}
	fs::create_dir_all(&qp_dir).unwrap();
	env::set_current_dir(qp_dir).unwrap();

	// common
	fs::create_dir_all("common").unwrap();
	run_cmd(
		"curl",
		&[
			"-LsSo",
			"common/build.tar.gz",
			"https://github.com/quasipanacea/common/archive/refs/tags/nightly.tar.gz",
		],
	);
	run_cmd("tar", &["-C", "common", "-xf", "./common/build.tar.gz"]);

	// server
	fs::create_dir_all("server-deno").unwrap();
	run_cmd(
		"curl",
		&[
			"-LsSo",
			"server-deno/build.tar.gz",
			"https://github.com/quasipanacea/server-deno/releases/download/nightly/build.tar.gz",
		],
	);
	run_cmd(
		"tar",
		&["-C", "server-deno", "-xf", "./server-deno/build.tar.gz"],
	);

	// client
	fs::create_dir_all("client-web").unwrap();
	run_cmd(
		"curl",
		&[
			"-LsSo",
			"client-web/build.tar.gz",
			"https://github.com/quasipanacea/client-web/releases/download/nightly/build.tar.gz",
		],
	);
	run_cmd(
		"tar",
		&["-C", "client-web", "-xf", "client-web/build.tar.gz"],
	);
}
