#!/bin/bash

set -e


svd2rust -i svd/ASR6601.svd

rm -rf src

form -i lib.rs -o src/ && rm lib.rs

cargo fmt
