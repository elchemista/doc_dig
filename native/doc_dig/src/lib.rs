//! DocDig – Rust NIFs using `extractous`
//!
//! Compile with `mix rustler.crate build` (or through the usual `mix compile` pipeline).
//! Each NIF returns `{:ok, {text, metadata_string}} | {:error, reason}` on the Elixir side.
//! The four exported NIFs are:
//!
//!   * `extract_file/1`      –  given a local path, returns the extracted text.
//!   * `extract_url/1`       –  fetches & extracts from a remote URL.
//!   * `extract_bytes/1`     –  extracts from a raw binary (e.g. result of `File.read!/1`).
//!   * `extract_file_ocr/2`  –  like `extract_file/1` but forces OCR; second arg is an
//!                              optional Tesseract language code (defaults to "eng").
//!
//! Metadata is returned as a debug‑formatted string for the moment. If you need richer
//! metadata on the Elixir side, replace `meta_to_string/1` with a `serde_json` conversion
//! and decode JSON in Elixir.

use extractous::{Extractor, PdfOcrStrategy, PdfParserConfig, TesseractOcrConfig};
use rustler::Binary;
use std::io::{BufReader, Read};

#[rustler::nif(schedule = "DirtyCpu")]
fn extract_file(path: String) -> Result<(String, String), String> {
    let extractor = Extractor::new();
    extractor
        .extract_file_to_string(&path)
        .map(|(text, meta)| (text, meta_to_string(meta)))
        .map_err(|e| e.to_string())
}

#[rustler::nif(schedule = "DirtyCpu")]
fn extract_url(url: String) -> Result<(String, String), String> {
    let extractor = Extractor::new();
    match extractor.extract_url(&url) {
        Ok((stream, meta)) => Ok((read_stream(stream)?, meta_to_string(meta))),
        Err(e) => Err(e.to_string()),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn extract_bytes(bytes: Binary) -> Result<(String, String), String> {
    let extractor = Extractor::new();
    match extractor.extract_bytes(bytes.as_slice()) {
        Ok((stream, meta)) => Ok((read_stream(stream)?, meta_to_string(meta))),
        Err(e) => Err(e.to_string()),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn extract_file_ocr(path: String, lang: Option<String>) -> Result<(String, String), String> {
    let lang = lang.unwrap_or_else(|| "eng".to_owned());

    let extractor = Extractor::new()
        .set_ocr_config(TesseractOcrConfig::new().set_language(&lang))
        .set_pdf_config(PdfParserConfig::new().set_ocr_strategy(PdfOcrStrategy::OCR_ONLY));

    extractor
        .extract_file_to_string(&path)
        .map(|(text, meta)| (text, meta_to_string(meta)))
        .map_err(|e| e.to_string())
}

fn read_stream<R: Read>(stream: R) -> Result<String, String> {
    let mut reader = BufReader::new(stream);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    String::from_utf8(buf).map_err(|e| e.to_string())
}

fn meta_to_string<M: std::fmt::Debug>(meta: M) -> String {
    format!("{:?}", meta)
}

rustler::init!("Elixir.DocDig");
