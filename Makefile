.PHONY: clippy
clippy:
	cargo clippy --all-features --tests -- -Dclippy::all -Dclippy::pedantic -D warnings