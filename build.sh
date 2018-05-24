#!/bin/bash
CMD=${1:-"dev"}

if [ $CMD = "prod" ]; then
  docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.26 cargo build --release
elif [ $CMD = "run" ]; then 
  docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.26 cargo run
else
  docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.26 cargo build
fi