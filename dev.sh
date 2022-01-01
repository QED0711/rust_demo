#!/bin/bash

docker build -t rust_dev:latest .;

docker run --rm -ti \
-p 5000:5000 \
-v "$(pwd)"/:/demo/ \
rust_dev:latest \
bash