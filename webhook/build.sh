#!/usr/bin/env bash

mkdir -p output/bin
cp bootstrap.sh output/
chmod +x output/bootstrap.sh

cargo build --release

cp target/release/server output/bin/
