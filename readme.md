# Shared Spaces App Spike/POC

This is a POC I created back in 2020. It's an app that allows users to share files P2P in-browser via WebRTC.

See [Past Rust Experiments: Positivelys & Shared Spaces](https://logankeenan.com/posts/past-rust-experiments-positivelys-and-shared-spaces/) for more details

## Experiments

1. Figure out how to make an app with Rust and WASM
2. Use "HTTP Requests" across boundaries
    * Send data across the FFI between JavaScript and WASM. This allowed for familiar server-side pattern between the
      browser and the WASM app. The WASM app returns HTML responses and the browser JS code updates the DOM
    * Send "HTTP Requests" across the WebRTC connection. Again, this allowed for a familiar server-side pattern. Each 
      WebRTC client asks like independent server where they can make HTTP requests to each other. 
3. A basic BitTorrent implementation for file sharing where files are split into parts, ordered, hashed,
   and sent across the WebRTC connection to be reconstructed

## Locally Dev

`cargo watch -s 'wasm-pack build --target web --scope logankeenan' -i pkg` 
