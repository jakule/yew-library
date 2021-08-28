# build environment
FROM rust:1.54-bullseye as builder
WORKDIR /app
COPY . .
RUN rustup target add wasm32-unknown-unknown

ENV TRUNK_VERSION 0.13.1

RUN cd /usr/local/cargo/bin/ && wget -qO- https://github.com/thedodd/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-
RUN trunk build --release

# production environment
FROM nginx:1.21-alpine
COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 8000
CMD ["nginx", "-g", "daemon off;"]