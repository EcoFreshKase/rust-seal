#!/bin/bash

# Delete rust-seal.config.json
rm -f rust-seal.config.json

# Delete keys directory and everything in it
rm -rf keys

# Delete everything in test directory except testfile.txt
find test -mindepth 1 ! -name 'testfile.txt' -delete
