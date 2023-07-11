default: run-web

build-web:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --out-name bevy_app --out-dir web/build --target web target/wasm32-unknown-unknown/release/megatictactoe.wasm
    wasm-opt -O2 web/build/bevy_app_bg.wasm -o web/build/bevy_app_bg.wasm

run-web: build-web
    sfz -r ./web -p 5000