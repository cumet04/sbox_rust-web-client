#!/bin/bash

cargo build --release
mv target/release/sbox_rust-web-client lambda/deploy/bootstrap
cd lambda
lambroll deploy --src deploy
