FROM tube-recorder/rust-builder:latest as builder

# copy the application code
COPY --chown=builder:builder . ./app

# build and install
RUN \
  (cd app; cargo install --path .)

FROM tube-recorder/rust-release:latest

# copy installed application
COPY \
  --from=builder \
  /usr/local/cargo/bin/server \
  /usr/local/bin/app

# default serice port
EXPOSE 50051

# default command
CMD ["app", "--stdout-log", "--log-file", "/home/release/app.log"]
