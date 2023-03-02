# shellcheck shell=bash

task.release() {
	cargo release --push-remote me -x "$@"
}
