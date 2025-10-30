FROM alpine:3.22.2

RUN apk add --no-cache curl dos2unix

COPY scripts/generate_tokens.sh /usr/local/bin/

RUN dos2unix /usr/local/bin/generate_tokens.sh && chmod +x /usr/local/bin/generate_tokens.sh

ENTRYPOINT ["/usr/local/bin/generate_tokens.sh"]
