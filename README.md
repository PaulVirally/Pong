# Pong in WebAssembly with WebGL2
This is a version of the classic Pong game written in Rust with a WebAssembly target that utilizes WebGL2 to render the graphics.

# Compiling and Running
To compile to a WASM target, install [wasm-pack](https://github.com/rustwasm/wasm-pack) then run the following from the root directory of this repo:
```sh
wasm-pack build --target web
```
To deploy this small app, you can run any old http server (below is an example using python3):
```sh
python3 -m http.server
```
Then you can head over to your server and enjoy Pong in your browser!
