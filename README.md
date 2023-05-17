<div align="center">
    <img src="assets/logo.svg" alt="curss logo" height="125">
</div>

Curss is a minimal RSS feed aggregator with notifications.

## Status

Curss is currently a work-in-progress.

## Packages

Development of curss is separated into several packages.

Package | Description
------- | -----------
[curss](curss) | The main server binary. Responsible for command line argument parsing, configuration, and starting the server.
[curss_server](curss_server) | The server library.
[curss_client](curss_client) | The web client.

## Building

### Dependencies

Curss is a full stack rust app, so you will need to [install rust](https://www.rust-lang.org/tools/install).

To build the web client you will need the `wasm32-unknown-unknown` target. If you installed rust with rustup you can use:

```shell
rustup target add wasm32-unknown-unknown
```

For the web client, you will also need to install [Node.js](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm) (for using [tailwindcss](https://tailwindcss.com/) via `npx`) and the [dioxus CLI](https://github.com/DioxusLabs/cli).

### Server

The server binary is built like a normal rust project:

```shell
cargo build -p curss --release
```

To use the server binary, the client must be built, and the output directory supplied via the `--client-dir` option (default is `dist`).

### Client

To build the web client:
- Switch to the `curss_client` directory
- Use tailwind to generate the CSS
- Build the output directory with dioxus

```shell
cd curss_client
npx tailwindcss -i input.css -o public/tailwind.css
dioxus build --release
```

## Development

After installing the [dependencies](#dependencies), start the server:

```shell
cargo run -p curss -- --client-dir curss_client/dist/
```

In another terminal, in the `curss_client` directory, start the tailwind watcher:

```shell
npx tailwindcss -i input.css -o public/tailwind.css --watch
```

Again in another terminal, and again in the `curss_client` directory, start the dioxus dev server:

```shell
dioxus serve
```

Optionally, you can use [`mprocs`](https://github.com/pvolok/mprocs) from the main project directory to start the above commands all at once in a single terminal.

## License

Curss is licensed under the [GNU General Public License v3.0](https://www.gnu.org/licenses/gpl-3.0.en.html) or later, see the [license](LICENSE) file for details.
