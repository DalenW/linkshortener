# THIS BREAKS WHEN COMPILING A CERTAIN CRATE
# otherwise it could save 2 GBs :(

#FROM frolvlad/alpine-glibc

#ENV RUSTUP_HOME=/usr/local/rustup \
#    CARGO_HOME=/usr/local/cargo \
#    PATH=/usr/local/cargo/bin:$PATH

#RUN apk update \
#	&& apk upgrade \
##	&& apk add bash curl

#RUN apk add --no-cache \
#        ca-certificates \
#        gcc

#RUN curl -s -L https://sh.rustup.rs | bash /dev/stdin -y

#RUN chmod -R a+w $RUSTUP_HOME $CARGO_HOME

FROM rust:latest

WORKDIR /app
COPY . .
RUN cargo install --path .

CMD ["linkshortener"]