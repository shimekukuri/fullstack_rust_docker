FROM node:20 AS frontbuildvue
WORKDIR /app
COPY ./frontend .
RUN npm install
RUN npm run build

FROM messense/rust-musl-cross:x86_64-musl AS rustbuilder
ENV SQLX_OFFLINE=true
WORKDIR /app
COPY /backend/ .
RUN ls -la ./
COPY --from=frontbuildvue /app/dist/ /app/src/dist
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM scratch
COPY --from=rustbuilder /app/target/x86_64-unknown-linux-musl/release/htmx /app
COPY /backend/configuration /configuration
ENTRYPOINT [ "/app" ]
EXPOSE 4000
