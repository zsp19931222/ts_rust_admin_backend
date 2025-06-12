FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

# 运行阶段（保持构建镜像一致性）
FROM debian:bookworm-slim
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/admin-backend /usr/local/bin/
COPY --from=builder /usr/src/app/.env /usr/src/app/
COPY --from=builder /usr/src/app/data /usr/src/app/data

RUN apt-get update && apt-get install -y libc6

# 设置环境变量
ENV PORT=10086

CMD ["admin-backend"]