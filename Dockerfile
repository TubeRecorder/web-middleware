FROM my-build/rust-builder:latest as builder

COPY . .

RUN \
  cargo install --path .

FROM my-build/rust-release:latest

COPY \
  --from=builder \
  /usr/local/cargo/bin/server \
  /usr/local/bin/app

EXPOSE 50051

CMD ["app", "--stdout-log", "--log-file", "/var/log/app/app.log", "--download-host", "download"]
