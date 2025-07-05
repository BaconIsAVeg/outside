BRANCH := "main"
NAME := "outside"
VER := shell('tomlq -r .package.version Cargo.toml')
ZIP_NAME := NAME + '-' + VER + '-' + 'x86_64.tgz'
TEMPFILE := shell('mktemp -t cliff.XXXXXX')
RUNID := `gh run list --json databaseId,status,conclusion | jq 'map(select(.status=="completed" and .conclusion=="success")) | .[0:1] | .[].databaseId'`

dev: fmt fix clippy test
    @echo "{{BLACK + BG_BLUE}}Development checks complete.{{NORMAL}}"
    jj status

doc:
    cargo doc -q

demo:
    cd demo && vhs demo.tape

changes:
    git cliff -o CHANGELOG.md

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
    @echo "{{BLACK + BG_GREEN}}Installing {{NAME}} version {{VER}}...{{NORMAL}}"
    cargo -q install --path .

publish: test build
    @echo "{{BLACK + BG_GREEN}}Releasing package {{ZIP_NAME}} to crates.io and github...{{NORMAL}}"
    git checkout main

    read "NOP?Publish release to creates.io? CTRL-C to cancel."
    cargo release patch --sign --no-push -x

    read "NOP?Publish release to Github? CTRL-C to cancel."
    goreleaser release --clean
