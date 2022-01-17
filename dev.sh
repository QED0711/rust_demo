#!/bin/bash

docker build -t rust_dev:latest .;

docker run --rm -ti \
-p 8000:8000 \
-v "$(pwd)"/:/demo/ \
rust_dev:latest \
bash