FROM rust:latest

# Install X11 dependencies
RUN apt-get update && apt-get install -y \
    libx11-dev \
    libxcb1-dev \
    libx11-xcb-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/myapp
COPY . .

RUN cargo build
