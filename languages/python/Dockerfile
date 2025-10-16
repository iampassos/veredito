FROM alpine:latest

RUN apk add --no-cache python3 musl-dev coreutils util-linux bash

ARG USER_ID=1000
ARG GROUP_ID=1000

RUN addgroup -g ${GROUP_ID} -S sandbox && \
    adduser -u ${USER_ID} -S -G sandbox executor
USER executor

WORKDIR /submission

CMD ["/bin/bash"]
