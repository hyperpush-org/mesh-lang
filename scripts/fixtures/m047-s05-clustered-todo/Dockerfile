FROM ubuntu:24.04
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY output /usr/local/bin/todo-starter
ENV PORT=8080
ENV TODO_DB_PATH=/var/lib/todo/todo.sqlite3
ENV TODO_RATE_LIMIT_WINDOW_SECONDS=60
ENV TODO_RATE_LIMIT_MAX_REQUESTS=5
ENV MESH_CLUSTER_PORT=4370
VOLUME ["/var/lib/todo"]
EXPOSE 8080 4370
ENTRYPOINT ["/usr/local/bin/todo-starter"]
