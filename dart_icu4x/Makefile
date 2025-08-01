
help: ## This help dialog.
	@IFS=$$'\n' ; \
	help_lines=(`fgrep -h "##" $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//'`); \
	for help_line in $${help_lines[@]}; do \
			IFS=$$'#' ; \
			help_split=($$help_line) ; \
			help_command=`echo $${help_split[0]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
			help_info=`echo $${help_split[2]} | sed -e 's/^ *//' -e 's/ *$$//'` ; \
			printf "%-30s %s\n" $$help_command $$help_info ; \
	done

fresh: ## Runs `clean`, `codegen-build`, and `generate-intl` for a fresh setup.
	make clean 

clean: ## Cleans Flutter project.
	flutter clean
	flutter pub get
	cargo build --manifest-path rust/Cargo.toml 
	flutter_rust_bridge_codegen generate

cargo-build: ## Runs `cargo build --manifest-path rust/Cargo.toml`.
	cargo build --manifest-path rust/Cargo.toml 

generate: ## Runs `flutter_rust_bridge_codegen generate`.
	flutter_rust_bridge_codegen generate

analyze: ## Runs `flutter analyze`.
	flutter analyze

format: ## Formats dart files.
	dart format .

test: ## Runs unit tests.
	flutter test
	