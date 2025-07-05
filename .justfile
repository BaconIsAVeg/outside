BRANCH := "main"
NAME := "outside"

dev: fmt fix clippy test
    @echo "{{BLACK + BG_BLUE}}Development checks complete.{{NORMAL}}"
    jj status

doc:
    cargo doc -q

demo:
    cd demo && vhs demo.tape

test:
    cargo test -q

fmt:
    cargo fmt -q

fix:
    cargo fix -q --allow-dirty --allow-staged

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

coverage:
    cargo tarpaulin --skip-clean --all-targets --include-tests --out Html --output-dir dev/

build:
    cargo update
    cargo -q build --release

install: test build
    cargo -q install --path .

publish: test build
    @echo "{{BLACK + BG_GREEN}}Releasing package to crates.io and github...{{NORMAL}}"
    git checkout {{BRANCH}}

    continue := shell('read "NOP?Publish release to creates.io? CTRL-C to cancel."')
    cargo release patch --sign --no-push -x

    read "NOP?Publish release to Github? CTRL-C to cancel."
    goreleaser release --clean
