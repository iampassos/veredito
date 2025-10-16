FROM alpine:latest

RUN apk add --no-cache gcc musl-dev coreutils util-linux

ARG USER_ID=1000
ARG GROUP_ID=1000

RUN addgroup -g ${GROUP_ID} -S sandbox && \
    adduser -u ${USER_ID} -S -G sandbox executor
USER executor

WORKDIR /submission

CMD ["/bin/sh"]
