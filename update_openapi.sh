#!/bin/bash

docker build -t crust:latest generator

docker run --rm -u $UID -v "${PWD}:/out" -v "${PWD}/generator_conf.yaml:/conf.yaml" crust:latest generate -i=https://nethsmdemo.nitrokey.com/api_docs/nethsm-api.yaml -o out -g crust -c /conf.yaml
cargo fmt