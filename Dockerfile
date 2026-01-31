# Dockerfile
# Satisfies RULE 2 - maintain Dockerfile

FROM rust:slim-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./

RUN mkdir -p src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release || true

COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ARG APP_NAME=plainpad
ARG APP_VERSION=0.1.0
ARG APP_MAINTAINER="Gilles Biagomba <gilles.infosec@gmail.com>"

LABEL maintainer="${APP_MAINTAINER}"
LABEL version="${APP_VERSION}"
LABEL org.opencontainers.image.title="${APP_NAME}"

RUN useradd -m -s /usr/sbin/nologin ${APP_NAME}
WORKDIR /home/${APP_NAME}
USER ${APP_NAME}

COPY --from=builder /app/target/release/${APP_NAME} /usr/local/bin/${APP_NAME}

ENTRYPOINT ["plainpad"]
CMD []
