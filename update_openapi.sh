#!/bin/bash

docker build -t crust:latest generator

docker run --rm -v "${PWD}:/out" -v "${PWD}/nethsm-api.yaml:/nethsm-api.yaml" -v "${PWD}/generator_conf.yaml:/conf.yaml" crust:latest generate -i=/nethsm-api.yaml -o out -g crust -c /conf.yaml
cargo fmt
