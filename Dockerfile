FROM tube-recorder/rust-builder:latest as builder

COPY . .

RUN \
  cargo install --path .

FROM tube-recorder/rust-release:latest

COPY \
  --from=builder \
  /usr/local/cargo/bin/server \
  /usr/local/bin/app

EXPOSE 50051

CMD ["app", "--stdout-log", "--log-file", "/var/log/app/app.log"]
