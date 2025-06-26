BRANCH := "main"
NAME := "outside"
VER := shell('tomlq -r .package.version Cargo.toml')
ZIP_NAME := NAME + '-' + VER + '-' + 'x86_64.tgz'
RUNID := `gh run list --json databaseId,status,conclusion | jq 'map(select(.status=="completed" and .conclusion=="success")) | .[0:1] | .[].databaseId'`

# Run the in-progress development process
dev: fmt fix clippy test
    @echo "{{BLACK + BG_BLUE}}Development checks complete.{{NORMAL}}"
    jj status

doc:
    cargo doc -q

changes:
    git cliff -l -o CHANGELOG.md

test:
    @echo "{{BLACK + BG_GREEN}}Running tests for {{NAME}} version {{VER}}...{{NORMAL}}"
    cargo test -q

fmt:
    cargo fmt -q

fix:
    cargo fix -q --allow-dirty --allow-staged

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

# Generate the code coverage report
coverage:
    cargo tarpaulin --skip-clean --all-targets --include-tests --out Html --output-dir dev/

# Build the package
[doc('Run `just VER=<version> build` to update the version number')]
build:
    @echo "{{BLACK + BG_GREEN}}Building {{NAME}} version {{VER}}...{{NORMAL}}"
    cargo update
    cargo -q build --release

# Test, build, and install the package
install: test build
    @echo "{{BLACK + BG_GREEN}}Installing {{NAME}} version {{VER}}...{{NORMAL}}"
    cargo -q install --path .

# Package the build artifacts
package: test build
    @echo "{{BLACK + BG_GREEN}}Packaging {{NAME}} version {{VER}}...{{NORMAL}}"
    mkdir -p dist
    # cp LICENSE dist/
    # cp README.md dist/
    cp target/release/{{NAME}} dist/
    tar -czf {{ZIP_NAME}} -C dist .

publish-github:
    @echo "{{BLACK + BG_GREEN}}Fetching latest changes from {{BRANCH}}...{{NORMAL}}"
    @echo "Latest successful run ID: {{RUNID}}"
    mkdir -p dist
    gh run download {{RUNID}} --dir temp-{{NAME}}/
    find temp-{{NAME}}/ -type f -regex '.*[gz,zip]' -exec mv {} dist/ \;
    rename -v 'outside' 'outside-{{VER}}' dist/*
    rm -rf temp-{{NAME}}
    @echo "{{BLACK + BG_GREEN}}Creating Github release...{{NORMAL}}"
    gh release create {{VER}} dist/* --title "Release {{VER}}" -F CHANGELOG.md
    rm -rf dist

# Publish the packages
publish: test build package publish-crates
    @echo "{{BLACK + BG_BLUE}}Done publishing packages!{{NORMAL}}"

publish-crates: test build
    @echo "{{BLACK + BG_GREEN}}Releasing package {{ZIP_NAME}} to crates.io...{{NORMAL}}"
    git checkout main
    cargo release patch --sign -x
