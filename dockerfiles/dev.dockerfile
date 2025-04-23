FROM rust:1.86.0

# Install dioxus-cli
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall dioxus-cli

# Install node & npm
RUN apt-get update && apt-get install -y npm

WORKDIR /app

COPY . .
RUN rm -rf dockerfiles

EXPOSE 8080

# ENTRYPOINT [ "dx", "serve" ]
ENTRYPOINT [ "tail", "-f", "/dev/null" ]
