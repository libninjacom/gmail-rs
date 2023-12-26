
run:
    cargo run

test:
    cargo test

build:
    cargo build

install:
    cargo install --path .

generate:
    test -e openapi.yaml || req https://tryapisproxy.com/spec/googlemail | jq -Y > openapi.yaml
    cd transform && checkexec ../transform.yaml ../openapi.yaml src/main.rs -- cargo run -- ../openapi.yaml ../transform.yaml
    libninja gen -v -lrust gmail transform.yaml