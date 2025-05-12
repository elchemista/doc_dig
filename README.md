# DocDig

**DocDig** is an Elixir wrapper around the Rust-based [`extractous`](https://github.com/yobix-ai/extractous) library, exposing high-performance document and web-page text extraction via Rustler NIFs.

## Features

* Extract text from **local files**: PDF, DOCX, HTML, Markdown, etc.
* Fetch and extract from **URLs**.
* Extract from in-memory **binaries**.
* Perform **OCR** on image-only PDFs or images via Tesseract (customizable language).
* Optional **bang (`!`)** variants that raise on errors for concise workflows.
* Precompiled NIFs with [`rustler_precompiled`](https://github.com/philss/rustler_precompiled) support for zero‑toolchain installs.

## Installation

Add to your `mix.exs`:

```elixir
def deps do
  [
    {:doc_dig, github: "elchemista/doc_dig", branch: "master"}
  ]
end
```

Then fetch and compile:

```bash
mix deps.get
mix compile
```

## Usage Examples

```elixir
# Extract from a local Markdown file:
{:ok, {text, metadata}} = DocDig.extract_file("README.md")
IO.puts(text)
IO.inspect(metadata)

# Raise on failure:
{text, _meta} = DocDig.extract_file!("README.md")

# Extract from a URL:
{:ok, {html_text, _}} = DocDig.extract_url("https://example.com")

# Extract from in-memory binary (e.g. download via HTTPoison):
{:ok, file_bytes} = HTTPoison.get("https://example.com/sample.docx")
{:ok, {doc_text, _}} = DocDig.extract_bytes(file_bytes)

# Force OCR on a scanned PDF (German language):
{:ok, {ocr_text, _}} = DocDig.extract_file_ocr("invoice_scanned.pdf", "deu")
```

## Contributing

1. Fork the repo
2. Create a feature branch: `git checkout -b feature/my-addition`
3. Run tests: `mix test`
4. Submit a pull request

## Credits

* **extractous** by Yobix AI and contributors
* **Rustler** by the Rustler team
* **Tesseract OCR** for OCR support
* **Elixir** and **Erlang/OTP** community

## License

Apache-2.0 © [License](LICENSE)
