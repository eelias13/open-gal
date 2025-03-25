cd rust 
wasm-pack build --target no-modules --no-typescript --release  
cd ..

cp rust/pkg/rust_bg.wasm web/rust_bg.wasm
cp rust/pkg/rust.js web/wasm_loader.js