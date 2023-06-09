ARG test=false

# Start with a base alpine image
FROM rust:1.69-alpine AS build

# Set the working directory
WORKDIR /review

# Copy the project files
COPY src/ ./src/
COPY Cargo.toml .

RUN cargo build --release && \
    mkdir bin && \
    mv target/release/review bin/review

# Start with a base alpine image
FROM alpine:3.17.3 

# Install libstdc++
RUN apk update && \
    apk add --no-cache \
    libstdc++

# create a user and group
RUN addgroup -S shs && adduser -S shs -G shs
USER shs

# Copy the binary from the build stage
COPY --chown=shs:shs --from=build \
    ./review/bin/* \
    ./app/

# Set the entrypoint
ENTRYPOINT [ "./app/review" ]
