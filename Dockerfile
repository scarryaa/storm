FROM rust:latest

# Install X11 and Vulkan dependencies
RUN apt-get update && apt-get install -y \
    libx11-dev \
    libxcb1-dev \
    libx11-xcb-dev \
    vulkan-tools \
    vulkan-validationlayers-dev \
    spirv-tools \
    libvulkan-dev \
    mesa-vulkan-drivers \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/myapp
COPY . .

# Set Vulkan environment variables
ENV VULKAN_SDK=/usr
ENV PATH="$VULKAN_SDK/bin:$PATH"
ENV LD_LIBRARY_PATH="/usr/lib"
ENV VK_LAYER_PATH="/usr/share/vulkan/explicit_layer.d"

RUN cargo build

