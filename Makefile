.PHONY: clippy
clippy:
	cargo clippy --all-features --tests -- -Dclippy::all -Dclippy::pedantic -D warnings

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: fmt-check
fmt-check:
	cargo fmt --all --check
