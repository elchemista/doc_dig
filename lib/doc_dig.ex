defmodule DocDig do
  @moduledoc """
  High‑level Elixir API for [`extractous`](https://github.com/DianaSuvorova/extractous)
  powered by Rust NIFs (see `native/doc_dig/src/lib.rs`).

  Each function returns either:

    * `{:ok, {text, metadata_string}}` — on success;
    * `{:error, reason}` — on failure.

  """

  use Rustler, otp_app: :doc_dig, crate: "doc_dig"
  @doc false
  defp nif_not_loaded, do: :erlang.nif_error(:nif_not_loaded)

  @spec extract_file(Path.t()) :: {:ok, {binary(), binary()}} | {:error, binary()}
  def extract_file(_path), do: nif_not_loaded()

  @spec extract_url(String.t()) :: {:ok, {binary(), binary()}} | {:error, binary()}
  def extract_url(_url), do: nif_not_loaded()

  @spec extract_bytes(binary()) :: {:ok, {binary(), binary()}} | {:error, binary()}
  def extract_bytes(_bytes), do: nif_not_loaded()

  @spec extract_file_ocr(Path.t(), String.t()) :: {:ok, {binary(), binary()}} | {:error, binary()}
  def extract_file_ocr(_path, _lang \\ "eng"), do: nif_not_loaded()

  @spec extract_file!(Path.t()) :: {binary(), binary()}
  def extract_file!(path), do: unwrap!(extract_file(path))

  @spec extract_url!(String.t()) :: {binary(), binary()}
  def extract_url!(url), do: unwrap!(extract_url(url))

  @spec extract_bytes!(binary()) :: {binary(), binary()}
  def extract_bytes!(bytes), do: unwrap!(extract_bytes(bytes))

  @spec extract_file_ocr!(Path.t(), String.t()) :: {binary(), binary()}
  def extract_file_ocr!(path, lang \\ "eng"), do: unwrap!(extract_file_ocr(path, lang))

  defp unwrap!({:ok, result}), do: result
  defp unwrap!({:error, reason}), do: raise("DocDig extraction failed: #{inspect(reason)}")
end
