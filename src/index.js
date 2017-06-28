const wasm = require('./main.rs');

wasm.initialize().then(module => {
    // Create a Javascript wrapper around our Rust function
    const add = module.cwrap('add', 'number', ['number', 'number']);

    console.log('Calling Rust functions from Javascript!');
    console.log(add(1, 2));
});