
run:
    cargo run

test:
    cargo test

build:
    cargo build

install:
    cargo install --path .

transform:
    test -e openapi.yaml || req https://tryapisproxy.com/spec/googlemail | jq -Y > openapi.yaml
    cd transform && checkexec ../transform.yaml ../openapi.yaml src/main.rs -- cargo run -- ../openapi.yaml ../transform.yaml

generate:
    test -e transform.yaml || just transform
    libninja gen -v -lrust --version 0.12.0 gmail transform.yaml

# Bump version. level=major,minor,patch
version level:
    git diff-index --exit-code HEAD > /dev/null || ! echo You have untracked changes. Commit your changes before bumping the version.
    cargo set-version --bump {{level}}
    cargo update # This bumps Cargo.lock
    VERSION=$(rg  "version = \"([0-9.]+)\"" -or '$1' Cargo.toml | head -n1) && \
        git commit -am "Bump version {{level}} to $VERSION" && \
        git tag v$VERSION && \
        git push origin v$VERSION
    git push

publish:
    cargo publish

patch: test
    just version patch
    just publish
