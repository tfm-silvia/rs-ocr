# rs-ocr

[![npm version](https://img.shields.io/npm/v/rs-ocr.svg)](https://www.npmjs.com/package/rs-ocr)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple, fast, and lightweight Node.js library for extracting text from PDF files, powered by Rust and WebAssembly.

This crate is primarily a WebAssembly wrapper around the excellent [`pdf_extract`](https://crates.io/crates/pdf-extract) crate, making its powerful PDF text extraction capabilities available in a Node.js environment.

## ✨ Features

* **Fast:** Leverages Rust's performance for quick text extraction.
* **Lightweight:** No heavy dependencies, ideal for serverless functions and backend services.
* **Simple API:** A single function to get the job done.
* **Server-Side Ready:** Built specifically for the `nodejs` target.

## 📦 Installation

Install the package from the npm registry using your favorite package manager:

```bash
npm install rs-ocr
```bash
yarn add rs-ocr
```bash
pnpm add rs-ocr
```

## 🚀 Usage

The library exposes a single function, `ocr`, which takes a `Buffer` or `Uint8Array` of the PDF file's contents and returns the extracted text as a string.

### Basic Node.js Example

Here is a simple example of reading a PDF file from the filesystem and extracting its text.

```javascript
// main.js
import { ocr } from 'rs-ocr';
import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

// Get the directory name of the current module
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

try {
  // Read the PDF file into a buffer
  const pdfBuffer = readFileSync(join(__dirname, 'path-to-your-document.pdf'));

  // Extract the text
  const extractedText = ocr(pdfBuffer);

  console.log('--- Extracted Text ---');
  console.log(extractedText);
  console.log('----------------------');

} catch (error) {
  console.error('Error processing PDF:', error);
}
```

### SvelteKit Form Action Example

Here is a real-world example of handling a PDF file upload in a SvelteKit endpoint.

```javascript
// src/routes/your-route/+server.js
import { ocr } from 'rs-ocr';
import { json } from '@sveltejs/kit';

/** @type {import('./$types').RequestHandler} */
export const POST = async ({ request }) => {
    try {
        const formData = await request.formData();
        const uploadedFile = formData.get('pdfFile');

        // Basic validation
        if (!(uploadedFile instanceof File)) {
            return json({ error: 'No file uploaded or invalid form data.' }, { status: 400 });
        }

        // Optional: File size validation
        const MAX_FILE_SIZE_BYTES = 100 * 1024 * 1024; // 100MB
        if (uploadedFile.size > MAX_FILE_SIZE_BYTES) {
            return json(
                { error: `File size exceeds the limit of ${MAX_FILE_SIZE_BYTES / (1024 * 1024)}MB.` },
                { status: 400 }
            );
        }

        // Convert the file to an ArrayBuffer, then to a Buffer for the ocr function
        const fileBuffer = Buffer.from(await uploadedFile.arrayBuffer());

        // Perform the text extraction
        const extractedText = ocr(fileBuffer);

        return json({ text: extractedText }, { status: 200 });

    } catch (error) {
        console.error('Failed to process PDF upload:', error);
        return json({ error: 'An internal server error occurred.' }, { status: 500 });
    }
};
```

## 🛠️ Building from Source

If you wish to build the package yourself, you will need to have the Rust toolchain and `wasm-pack` installed.

1.  **Install Rust:**
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
    ```

2.  **Install wasm-pack:**
    ```bash
    curl [https://rustwasm.github.io/wasm-pack/installer/init.sh](https://rustwasm.github.io/wasm-pack/installer/init.sh) -sSf | sh
    ```

3.  **Build the Package:**

    This project requires a specific `RUSTFLAGS` configuration because the `getrandom` crate (a dependency of `pdf_extract`) needs to be told which backend to use in a non-browser WASM environment.

    Run the following command from the root of the project directory:

    ```bash
    RUSTFLAGS='--cfg getrandom_backend="wasm_js"' wasm-pack build --target nodejs
    ```

    This will compile the Rust code to WebAssembly, generate the necessary JavaScript bindings, and place everything in a `pkg` directory, ready to be published to npm.

## 🙏 Acknowledgements

This library would not be possible without the fantastic work done by the developers of the **[`pdf_extract`](https://crates.io/crates/pdf-extract)** crate. This `rs-ocr` package is simply a lightweight wrapper to make that crate's functionality accessible to the Node.js ecosystem via WebAssembly. All credit for the core PDF processing logic goes to them.

## 📜 License

This project is licensed under the MIT License. See the `LICENSE` file for details.
