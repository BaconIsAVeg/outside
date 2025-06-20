BRANCH := "main"
NAME := "outside"
VER := shell('version-manager package ' + NAME + ' get')
ZIP_NAME := NAME + '-' + VER + '.tar.gz'

# Run the in-progress development process
dev: fmt fix clippy test # changelog
    @echo "{{BLACK + BG_BLUE}}Development checks complete.{{NORMAL}}"
    jj status

doc:
    cargo doc -q

changelog:
    git cliff -t {{VER}} -o CHANGELOG.md

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

# Clean up build artifacts
clean:
    @echo "{{BLACK + BG_GREEN}}Cleaning up build artifacts for {{NAME}}...{{NORMAL}}"
    rm -rf dist
    rm -f {{ZIP_NAME}}
    rm -f *.tar.zst
    rm -f *.tar.gz
    rm -rf pkg
    rm -rf src/target
    rm -rf src/{{NAME}}-{{VER}}
    rm -f src/{{NAME}}-{{VER}}.tar.gz

# Build the package
[doc('Run `just VER=<version> build` to update the version number')]
build: setver
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
    cp docs/LICENSE dist/
    cp README.md dist/
    cp target/release/{{NAME}} dist/
    tar -czf {{ZIP_NAME}} -C dist .

# Publish the packages
publish: test build publish-gitlab clean publish-crates
    @echo "{{BLACK + BG_BLUE}}Done publishing packages!{{NORMAL}}"

# Release the package to Gitlab
publish-gitlab: test build package
    @echo "{{BLACK + BG_GREEN}}Releasing package {{ZIP_NAME}} to Gitlab...{{NORMAL}}"
    glab release create \
        {{VER}} \
        --name "{{NAME}} {{VER}}" \
        --notes "Release of {{NAME}} version {{VER}}" \
        "./{{ZIP_NAME}}"

publish-crates: test build
    @echo "{{BLACK + BG_GREEN}}Releasing package {{ZIP_NAME}} to crates.io...{{NORMAL}}"
    cargo publish --allow-dirty

# Get the package version
getver:
    @echo "{{BLACK + BG_GREEN}}Fetching version for {{NAME}}...{{NORMAL}}"
    version-manager package {{NAME}} version

# Set the package version
setver:
    @echo "{{BLACK + BG_GREEN}}Setting version for {{NAME}} to {{VER}}...{{NORMAL}}"
    version-manager package {{NAME}} set {{VER}}
