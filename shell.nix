{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust 
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer
  ];

  RUST_BACKTRACE = 1;
}
