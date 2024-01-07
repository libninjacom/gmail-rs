
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
