FROM rust:slim-bullseye

RUN cargo install wasm-pack
RUN cargo install minifier

WORKDIR /usr/src/app

COPY . .

RUN cd rust && wasm-pack build --target no-modules --no-typescript --release && cd ..
RUN mkdir build
RUN cp -r vs build/vs

RUN minifier -o build/script.js web/script.js
RUN minifier -o build/rust.js rust/pkg/rust.js

RUN cp web/index.html build/index.html 
RUN cp rust/pkg/rust_bg.wasm build/rust_bg.wasm

CMD ["bash"]
