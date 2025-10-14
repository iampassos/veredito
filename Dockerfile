FROM gcc:15.2.0

COPY ./judge.sh .
RUN chmod +x judge.sh

WORKDIR /submission

CMD ["/judge.sh"]
