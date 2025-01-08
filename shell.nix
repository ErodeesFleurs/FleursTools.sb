with import <nixpkgs> { };

mkShell {
  buildInputs = [
    cargo
    rustc
    rust-analyzer
    rustfmt
  ];
  RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
}
