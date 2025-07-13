init:
	cargo deny init

fmt:
	cargo fmt

lint: fmt
	cargo check
	cargo clippy --all-targets --all-features -- -D warnings
	cargo deny check

build: lint
	cargo build

e2e: build
	@echo "Running end-to-end tests..."
	@./target/release/awsconnect store --name "test-aws" --secret "JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP" > /dev/null
	@./target/release/awsconnect generate --name "test-aws" > /dev/null
	@./target/release/awsconnect remove --name "test-aws" > /dev/null
	@echo "✅ All end-to-end tests passed!"
	