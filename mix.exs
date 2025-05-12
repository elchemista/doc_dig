defmodule DocDig.MixProject do
  use Mix.Project

  @version "0.1.0"

  def project do
    [
      app: :doc_dig,
      name: "DocDig",
      version: @version,
      elixir: "~> 1.18",
      elixirc_paths: elixirc_paths(Mix.env()),
      build_embedded: Mix.env() == :prod,
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      description: description(),
      package: package(),
      rustler_precompiled: [
        provider: :github,
        owner: "elchemista",
        repo: "doc_dig",
        tag: "v#{@version}"
      ],
      docs: [
        main: "readme",
        extras: [
          "README.md",
          "LICENSE"
        ]
      ],
      source_url: "https://github.com/elchemista/doc_dig",
      homepage_url: "https://github.com/elchemista/doc_dig"
    ]
  end

  defp elixirc_paths(:test), do: ["lib", "test/support"]
  defp elixirc_paths(_), do: ["lib"]

  defp description() do
    "DocDig: is an Elixir wrapper around the Rust-based extractous library, exposing high-performance document and web-page text extraction via Rustler NIFs."
  end

  defp package() do
    [
      name: "doc_dig",
      maintainers: ["Yuriy Zhar"],
      files: ~w(
               lib
               mix.exs
               README.md
               LICENSE
               checksum-Elixir.DocDig.exs
               native/doc_dig/Cargo.toml
               native/doc_dig/build.rs
               native/doc_dig/Cross.toml
               native/doc_dig/.cargo/config.toml
               native/doc_dig/src/lib.rs
        ),
      licenses: ["Apache-2.0"],
      links: %{
        "GitHub" => "https://github.com/elchemista/doc_dig"
      }
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.36.1"},
      # {:rustler_precompiled, "~> 0.8"},
      {:credo, "~> 1.6", only: [:dev, :test], runtime: false},
      {:benchee, "~> 1.0", only: [:dev, :test]},
      {:ex_doc, "~> 0.28.3", only: [:dev, :test], optional: true, runtime: false},
      {:dialyxir, "~> 1.4", only: [:dev, :test], runtime: false}
    ]
  end
end
