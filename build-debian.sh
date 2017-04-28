#!/bin/bash
# Build a binary for Debian
set -e

echo "Building acra-collector for Debian 8..."
docker build . -t acra-collector-debian
id=$(docker create acra-collector-debian)
mkdir -p target/release-debian
docker cp $id:/source/target/release/acra-collector target/release-debian/acra-collector
docker rm -v $id
strip target/release-debian/acra-collector
echo "Done. Output is at target/release-debian/"
