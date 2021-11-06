ARG TARGET="x86_64-unknown-linux-musl"
ARG APP_NAME="bidi_detector"

FROM docker.io/rust:alpine as builder
ARG TARGET

RUN apk add --no-cache musl-dev
WORKDIR /app/src/
RUN echo "// dummy" > /app/src/lib.rs
COPY Cargo.toml /app/
COPY Cargo.lock /app/
WORKDIR /app
RUN cargo fetch --target ${TARGET}
RUN rm -f src/lib.rs
COPY src /app/src
RUN cargo build --target ${TARGET} --release

FROM scratch as prod
ARG TARGET
ARG APP_NAME
COPY --from=builder app/target/${TARGET}/release/${APP_NAME} /app/${APP_NAME}
WORKDIR /data

ENTRYPOINT [ "/app/bidi_detector" ]