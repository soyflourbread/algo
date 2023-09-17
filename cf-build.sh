#!/bin/bash
mkdir bin
curl -sSL https://github.com/rust-lang/mdBook/releases/download/v0.4.34/mdbook-v0.4.34-x86_64-unknown-linux-gnu.tar.gz | tar -xz --directory=bin
bin/mdbook build
