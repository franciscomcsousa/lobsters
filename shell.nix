{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust 
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
    # SDL2
    SDL2
    pkg-config
  ];

  RUST_BACKTRACE = 1;
}
