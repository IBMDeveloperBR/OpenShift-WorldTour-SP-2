# Use custom musl builder image
#FROM us.icr.io/vnderlev-namespace/rust-musl-builder:latest as BUILDER
FROM vmpereiraf/rust-musl-builder:latest as BUILDER

# --- Add our source code.
ADD --chown=rust:rust . ./

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `rust-webapp`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=BUILDER \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/rust-webapp \
    /usr/local/bin/
    
# Expose port
EXPOSE 8080

# Run Actix-Web
CMD /usr/local/bin/rust-webapp
