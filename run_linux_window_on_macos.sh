#!/bin/bash

# Get IP address
export IP=$(ifconfig en0 | grep inet | awk '$1=="inet" {print $2}')

# Configure X11 permissions
xhost + $IP

# Run the container
docker run -it \
    -e DISPLAY=$IP:0 \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
    -v $(pwd):/usr/src/myapp \
    storm-linux cargo run
