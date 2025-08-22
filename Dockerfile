# Stage 1: Build
FROM rust:1.85 AS backend_builder

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

# Build the release version
RUN cargo build --target x86_64-unknown-linux-gnu --release --all-features

# Stage 2: Build
FROM rust:1.85 AS frontend_builder

# Install cross-compilation dependencies
RUN apt-get update && \
    rustup target add x86_64-unknown-linux-gnu

# Set the working directory
WORKDIR /usr/frontend/src/app

# Create .cargo/config.toml for cross-compilation
RUN mkdir -p .cargo
RUN echo '[target.x86_64-unknown-linux-gnu]' > .cargo/config.toml

# Copy the source code into the container
COPY ./frontend .

# Build the release version
RUN cargo build --target x86_64-unknown-linux-gnu --release --all-features

# Stage 3: Runtime
FROM debian:bullseye

# Install minimal dependencies for a static binary
RUN apt-get update && \ 
    apt-get install ca-certificates

# Create a non-root user
RUN useradd appuser

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=backend_builder /usr/backend/src/app/target/x86_64-unknown-linux-gnu/release/backend ./backend
COPY --from=frontend_builder /usr/frontend/src/app/target/x86_64-unknown-linux-gnu/release/frontend ./frontend
#COPY --from=backend_builder $HOME/.cargo/bin/sqlx-cli sqlx-cli

# Copy static assets
COPY ./backend/assets ./assets/


# Set the ownership and permissions
RUN chown appuser:appuser ./backend && \
    chmod 755 ./backend
# Set the ownership and permissions
RUN chown appuser:appuser ./frontend && \
    chmod 755 ./frontend
# Switch to the non-root user
USER appuser

# Expose the port
EXPOSE 8080

# Start the application
CMD ["./backend", "./frontend"]
