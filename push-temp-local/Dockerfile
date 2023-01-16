FROM debian:bullseye-slim AS runtime
#FROM debian:bullseye AS runtime

RUN apt-get update -y \
    && apt-get install lld clang wget -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
ENV DB_USERNAME postgres
ENV DB_PASSWORD docker
ENV DB_PORT 5432
ENV DB_HOST docker.mhentgeslab.work
ENV DB_NAME shop
ENV DATABASE_URL postgres://postgres:docker@docker.mhentgeslab.work:5432/shop
ENV APP_PORT 8000
ENV APP_HOST 0.0.0.0
ENV APP_BASE_URL 0.0.0.0
ENV OTEL_EXPORTER_OTLP_ENDPOINT=http://192.168.1.35:4317
ENV OTEL_RESOURCE_ATTRIBUTES service.name=push-temp-local 
ENV OTEL_SERVICE_NAME push-temp-local

COPY ./target/release/push-temp-local /usr/local/bin/
EXPOSE 8000
ENTRYPOINT ["/usr/local/bin/push-temp-local"]  