FROM cr-cn-beijing.volces.com/cp-test-online-dont-delete/webhook:rust
WORKDIR /usr/src/webhook
COPY . .
EXPOSE 9000

RUN ./build.sh
ENTRYPOINT [ "./output/bootstrap.sh" ]