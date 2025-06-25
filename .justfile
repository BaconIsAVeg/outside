BRANCH := "main"
NAME := "outside"
VER := shell('version-manager package ' + NAME + ' get')
ZIP_NAME := NAME + '-' + VER + '-' + 'x86_64.tgz'

# Run the in-progress development process
dev: fmt fix clippy test changes
    @echo "{{BLACK + BG_BLUE}}Development checks complete.{{NORMAL}}"
    jj status

doc:
    cargo doc -q

changes:
    git cliff -t {{VER}} -o Changes.md

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
build: setver changes
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

# Publish the packages
publish: test build publish-github publish-crates
    @echo "{{BLACK + BG_BLUE}}Done publishing packages!{{NORMAL}}"

# Release the package to Github
publish-github: test build package
    @echo "{{BLACK + BG_GREEN}}Releasing package {{ZIP_NAME}} to Github...{{NORMAL}}"
    # WIP

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
