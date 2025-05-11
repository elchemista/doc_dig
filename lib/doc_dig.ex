defmodule DocDig do
  @moduledoc """
  Elixir wrapper around the Rust‑based [`extractous`](https://github.com/DianaSuvorova/extractous)
  library (see `native/doc_dig/`).  It lets you pull readable text from almost
  any document or web page directly in Elixir.

  Every API returns either:

    * `{:ok, {text, metadata_string}}` – success (text may be empty for images
      without OCR, etc.)
    * `{:error, reason}`               – a human‑readable error message

  Each function also has a **bang (!) variant** that raises on failure to keep
  happy‑path code tidy:

  ```elixir
  {text, _meta} = DocDig.extract_file!("README.md")
  ```
  """

  use Rustler, otp_app: :doc_dig, crate: "doc_dig"

  @doc """
  Extract text from a **local file** (PDF, DOCX, HTML, etc.).

  ```elixir
  iex> {:ok, {text, meta}} = DocDig.extract_file("README.md")
  iex> String.contains?(text, "DocDig")
  true
  ```
  """
  @spec extract_file(Path.t()) :: {:ok, {binary(), binary()}} | {:error, binary()}
  def extract_file(_path), do: nif_not_loaded()

  @doc """
  Extract text from a **URL**.  Follows redirects; downloads the resource to a
  temp file under the hood.

  ```elixir
  iex> {:ok, {text, _meta}} = DocDig.extract_url("https://example.com")
  iex> String.starts_with?(text, "Example Domain")
  true
  ```
  """
  @spec extract_url(String.t()) :: {:ok, {binary(), binary()}} | {:error, binary()}
  def extract_url(_url), do: nif_not_loaded()

  @doc """
  Extract from an in‑memory **binary**, e.g. the result of `File.read!/1`.

  Useful when the document is already in memory or was fetched through another
  HTTP client.

  ```elixir
  iex> bytes = File.read!("test/support/sample.docx")
  iex> {:ok, {text, _}} = DocDig.extract_bytes(bytes)
  iex> text =~ "Quarterly Report"
  true
  ```
  """
  @spec extract_bytes(binary()) :: {:ok, {binary(), binary()}} | {:error, binary()}
  def extract_bytes(_bytes), do: nif_not_loaded()

  @doc """
  Force **OCR** on a PDF or image‑only document.  Second argument is the
  [Tesseract‑lang code](https://github.com/tesseract-ocr/tesseract/wiki/Data-Files)
  (default: `"eng"`).

  ```elixir
  iex> DocDig.extract_file_ocr!("scanned_invoice.pdf", "deu")
  {text, meta}
  ```
  """
  @spec extract_file_ocr(Path.t(), String.t()) :: {:ok, {binary(), binary()}} | {:error, binary()}
  def extract_file_ocr(_path, _lang \\ "eng"), do: nif_not_loaded()

  @doc """
  Same as `extract_file/1` but **raises** on error.
  """
  @spec extract_file!(Path.t()) :: {binary(), binary()}
  def extract_file!(path), do: unwrap!(extract_file(path))

  @doc """
  Same as `extract_url/1` but **raises** on error.
  """
  @spec extract_url!(String.t()) :: {binary(), binary()}
  def extract_url!(url), do: unwrap!(extract_url(url))

  @doc """
  Same as `extract_bytes/1` but **raises** on error.
  """
  @spec extract_bytes!(binary()) :: {binary(), binary()}
  def extract_bytes!(bytes), do: unwrap!(extract_bytes(bytes))

  @doc """
  Same as `extract_file_ocr/2` but **raises** on error.
  """
  @spec extract_file_ocr!(Path.t(), String.t()) :: {binary(), binary()}
  def extract_file_ocr!(path, lang \\ "eng"), do: unwrap!(extract_file_ocr(path, lang))

  # Private helpers

  defp nif_not_loaded, do: :erlang.nif_error(:nif_not_loaded)
  defp unwrap!({:ok, result}), do: result
  defp unwrap!({:error, reason}), do: raise("DocDig extraction failed: #{inspect(reason)}")
end
