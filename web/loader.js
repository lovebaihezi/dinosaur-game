export default function (wasm_url, init) {
    const spinner = document.getElementById("loading-spinner");

    // The `init` function is the default export from the wasm-bindgen module.
    // It takes the WASM URL as an argument and returns a promise.
    return init(wasm_url).then(() => {
        if (spinner) {
            spinner.style.display = 'none';
        }
    });
}
