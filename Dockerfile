FROM rust:latest
RUN curl -fsSL https://deb.nodesource.com/setup_19.x | bash - && apt-get install -y nodejs

WORKDIR /workbench
COPY . .

RUN cargo build --release

FROM ubuntu:latest
WORKDIR /app
COPY --from=0 /workbench/target/release/library-server /app/
COPY --from=0 /workbench/web /app/web

ARG PORT=8080
ENV PORT ${PORT}
EXPOSE ${PORT}

CMD ["/app/library-server"]