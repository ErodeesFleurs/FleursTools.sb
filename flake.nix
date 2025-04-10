{
  description = "A Nix-flake-based Node.js development environment";

  inputs = {
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      ...
    }:
    let
      system = "x86_64-linux";
    in
    {
      devShells."${system}".default =
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        pkgs.mkShell {
          packages = with pkgs; [
            cmake
            # cargo
            # rust-analyzer
            # rustc
            # rustfmt
            rust-bin.nightly.latest.default
          ];
          RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          shellHook = ''
            export CARGO_HOME="$PWD/.cargo"
            export PATH="$CARGO_HOME/bin:$PATH"
          '';
        };
    };
}
