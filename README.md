# witx-async-demo

This is a small demo of `witx-bindgen` and `async` functions. The [demo]
provides a form where a URL to a `.tar.gz` tarball is input, and the demo will,
from wasm, download, decompress, and extract the file. The file can then be
explored by clicking each of the files.

[Live demo][demo]

This is intended to show a number of features of `witx-bindgen` and interface
types in concert with one another, including:

* Resources - the response/headers objects from the host are resources that wasm
  doesn't directly have access to.
* `async` - the `fetch` method is exposed to wasm as an async method and the
  `Response` structure has an `async` method for further loading the body.
* Nontrivial wasm - the wasm has to parse both the `.gz` container and the
  `.tar` format of the downloaded tarballs, but readily available crates from
  crates.io make this easy in Rust.

[demo]: https://alexcrichton.github.io/witx-async-demo/
