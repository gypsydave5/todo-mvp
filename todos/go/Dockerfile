FROM golang:1.9-alpine as base
WORKDIR /usr/src
COPY . .
RUN CGO_ENABLED=0 go build -ldflags "-s -w" -o main
RUN ls
RUN pwd

FROM scratch
COPY --from=base /usr/src/main /go-http-microservice
COPY --from=base /usr/src/template.html /template.html
CMD ["/go-http-microservice"]
