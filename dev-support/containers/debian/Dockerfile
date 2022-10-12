FROM rust:slim-bullseye AS builder

COPY dev-support /tmp/dev-support
RUN /tmp/dev-support/bin/install-deps

WORKDIR /build
COPY . /build

ARG DEBUG=0
ARG CARGO_ARGS=""
ARG BIN_NAME="template-rust"

ENV RUST_BACKTRACE=1
ENV PROTOC="/usr/bin/protoc"
ENV PROTOC_INCLUDE="/usr/include"

RUN set -eu; \
  ${PROTOC} --version; \
  if [ "${DEBUG}" = "1" ]; then \
    cargo build ${CARGO_ARGS}; \
    cp target/debug/${BIN_NAME} /usr/bin; \
  else \
    cargo build ${CARGO_ARGS} --release; \
    cp target/release/${BIN_NAME} /usr/bin; \
    INITIAL_SIZE="$(sh -c 'sz="$(du -sk /usr/bin/'${BIN_NAME}')" ; echo "${sz%%[[:space:]]*}"')"; \
    strip /usr/bin/${BIN_NAME}; \
    FINAL_SIZE="$(sh -c 'sz="$(du -sk /usr/bin/'${BIN_NAME}')" ; echo "${sz%%[[:space:]]*}"')"; \
    REMOVED_SIZE=$((INITIAL_SIZE - FINAL_SIZE)); \
    echo "Cleaning process removed ${REMOVED_SIZE}KB"; \
    echo "Dropped binary size from ${INITIAL_SIZE}KB to ${FINAL_SIZE}KB"; \
  fi; \
  file /usr/bin/${BIN_NAME}; \
  ldd /usr/bin/${BIN_NAME}; \
  /usr/bin/${BIN_NAME} --version; \
  ${BIN_NAME} --version;


FROM debian:bullseye-slim

RUN set -eu; \
  apt-get update; \
  apt-get install --no-install-recommends -y ca-certificates; \
  apt-get clean; \
  rm -rf /var/lib/apt/lists/*;

USER 8787:8787

ENV RUST_BACKTRACE=${DEBUG}

COPY --from=builder /usr/bin/${BIN_NAME} /usr/bin/

ENTRYPOINT [ "/usr/bin/template-rust", "--version" ]
