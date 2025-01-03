## Install 

### docker

```bash
docker build -t open-gal-build . 
docker create --name extract-container open-gal-build
docker cp extract-container:/usr/src/app/build/ .
docker rm extract-container
```

### local

you need to have [rust]() installed and also 

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack minifier  
./build.sh
```