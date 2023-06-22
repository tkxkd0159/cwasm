#!/bin/bash

display_usage() {
  printf "\nMissing %s parameter" "$1"
  printf "\nUsage: build-opti <contract_name>\n"
  exit 1
}

if [ -z "$1" ]; then
  display_usage "[contract_name]"
fi

name=$1

set -x

docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="devcontract_cache_${name},target=/code/contracts/${name}"/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.13.0 ./contracts/"${name}"
