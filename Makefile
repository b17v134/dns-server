.PHONY: clippy
clippy:
	cargo clippy --all-features --tests -- -Dclippy::all -Dclippy::pedantic -D warnings

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: fmt-check
fmt-check:
	cargo fmt --all --check

.PHONY: integration-tests-docker integration-tests-bind-start integration-tests-bind-stop

integration-tests-docker:
	sudo podman build -f $(PWD)/docker/Dockerfile -t bind

integration-tests-bind-start:
	sudo podman network create --subnet 10.23.0.0/24 bind
	sudo podman run \
		-d \
		--network bind \
		--ip 10.23.0.7 \
		-v $(PWD)/testconfig/bind/named.conf.local:/etc/bind/named.conf.local:Z \
		-v $(PWD)/testconfig/bind/named.conf.options:/etc/bind/named.conf.options:Z \
		-v $(PWD)/testconfig/bind/example.test-bind:/etc/bind/example.test-bind:Z \
		--name bind \
		bind

integration-tests-bind-stop:
	sudo podman rm -f bind || true
	sudo podman network rm bind

.PHONY: integration-tests
integration-tests:
	cargo build --release
	$(MAKE) integration-tests-bind-start
	ruby integration-tests/tests.rb -v
	$(MAKE) integration-tests-bind-stop

.PHONY: docker
docker:
	podman build -f hack/Dockerfile.alpine -t rust-dev-alpine