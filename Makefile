.PHONY: fmt
fmt:
	find -type f -name "*.rs" -not -path "*target*" -exec rustfmt --edition 2018 {} \;

.PHONY: build-docker
build-docker:
	cp -r ~/.ssh ssh
	DOCKER_BUILDKIT=1 docker \
		build \
		-t template-cli:latest \
		--ssh \
		--squash .
	docker image save template-cli:latest -o template_cli.tar
	shred ssh/id_rsa
	shred ssh/id_rsa.pub
	rm -rf ssh
	pigz -f -9 template_cli.tar

.PHONY: build-cli
build-cli:
	./scripts/build_cli.sh

.PHONY: build-cli-debug
build-cli-debug:
	(cargo build ; cp target/debug/cli template-cli)
