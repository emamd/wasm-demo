Playing around with Rust and webassembly.

Using the node/webpack/rust-wasm-loader method specifed [here](https://medium.com/@ianjsikes/get-started-with-rust-webassembly-and-webpack-58d28e219635) somehow gives you an js file of 1.4 MB.

The rest of the stuff in /site is served through SimpleHTTPServer, the code is generated with a Makefile, and is much smaller. Details [here](http://asquera.de/blog/2017-04-10/the-path-to-rust-on-the-web/).

Because it's combining two different tutorials at once, it's a little messy.