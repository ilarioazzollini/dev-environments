ARG RUST_VERSION=1.91.0
FROM rust:${RUST_VERSION}

# Environment variables
ENV HOME=/root

# Set working directory
WORKDIR ${HOME}

# Add packages for formatting and linting
RUN rustup component add rustfmt
RUN rustup component add clippy
