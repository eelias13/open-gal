cd rust && wasm-pack build --target no-modules --no-typescript --release && cd ..
mkdir build
cp -r vs build/vs

minifier -o build/script.js web/script.js
minifier -o build/rust.js rust/pkg/rust.js

cp web/index.html build/index.html
cp rust/pkg/rust_bg.wasm build/rust_bg.wasm