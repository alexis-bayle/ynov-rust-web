FROM rust:1.67

WORKDIR /usr/src/tp
COPY . .

RUN cargo install --path .

RUN groupadd -r groupExo && useradd -r -g groupExo userExo
RUN chown -R userExo:groupExo /usr/src/tp

USER userExo

EXPOSE 8095

CMD ["cargo","run", "--release"]
