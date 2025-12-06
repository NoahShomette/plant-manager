# Stage 1: Build
FROM rust:1.88 AS backend_builder

VOLUME ["/app/assets", "/app/config"]

# Install cross-compilation dependencies
RUN apt-get update && \
    rustup target add x86_64-unknown-linux-gnu

#RUN cargo install sqlx-cli

# Set the working directory
WORKDIR /usr/backend/src/app

# Create .cargo/config.toml for cross-compilation
RUN mkdir -p .cargo
RUN echo '[target.x86_64-unknown-linux-gnu]' > .cargo/config.toml

# Copy the source code into the container
COPY ./backend .
COPY ./shared ../shared

# Build the release version
RUN cargo build --target x86_64-unknown-linux-gnu --release --all-features

# Stage 3: Runtime
FROM debian:bookworm

# Install minimal dependencies for a static binary
RUN apt-get update && \
    apt-get install -y ca-certificates

# Create a non-root user
RUN useradd appuser

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=backend_builder /usr/backend/src/app/target/x86_64-unknown-linux-gnu/release/backend ./backend
#COPY --from=backend_builder $HOME/.cargo/bin/sqlx-cli sqlx-cli

# Copy static assets
COPY ./backend/assets ./assets/

# Set the ownership and permissions
RUN chown appuser:appuser ./backend && \
    chmod 755 ./backend

# Switch to the non-root user
USER appuser

# Expose the port
EXPOSE 8080

# Start the application
CMD ["./backend"]
