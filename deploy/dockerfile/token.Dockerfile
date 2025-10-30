FROM alpine:3.22.2

RUN apk add --no-cache bash curl dos2unix jq

COPY scripts/generate_tokens.sh /usr/local/bin/
COPY scripts/quality_gate_setup.sh /usr/local/bin/

RUN dos2unix /usr/local/bin/generate_tokens.sh /usr/local/bin/quality_gate_setup.sh \
    && chmod +x /usr/local/bin/generate_tokens.sh /usr/local/bin/quality_gate_setup.sh

ENTRYPOINT ["/bin/sh","-c","/usr/local/bin/generate_tokens.sh && /usr/local/bin/quality_gate_setup.sh"]
