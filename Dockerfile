FROM gcc:15.2.0

COPY ./sandbox.sh .
RUN chmod +x sandbox.sh

RUN useradd -m sandboxuser
USER sandboxuser

WORKDIR /submission

CMD ["/sandbox.sh"]
