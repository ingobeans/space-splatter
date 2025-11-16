# SPLATTERd

<img width="900" alt="Screenshot 2025-11-16 005355" src="https://github.com/user-attachments/assets/d2713a46-562a-421f-a480-cacfcebc1f59" />

Splatter ALIENS. Reach the ESCAPE POD. But most importantly, SURVIVE.

(a short bullet hell game where you kill aliens aboard your space station!)

Written in Rust! Everything done entirely by myself aside from font. 

## Building

You will need rust installed.

To run standalone, do `cargo run`.

To build for web, using `basic-http-server` for serving, do 
```sh
cargo build --release --target wasm32-unknown-unknown && cp target/wasm32-unknown-unknown/release/splatterd.wasm web/ && basic-http-server web/
```
