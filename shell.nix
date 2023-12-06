let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  # rustVersion = "latest";
  rustVersion = "1.74.0";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
    ];
  };
in
pkgs.mkShell {
  name = "aoc-dev";

  buildInputs = [
    rust
  ] ++ (with pkgs; [
    pkg-config
    rust-analyzer
    sccache
  ]);
  # RUST_BACKTRACE = 1;
}
