{ pkgs ? import <nixpkgs> {} }:
with pkgs;

mkShell {
  buildInputs = [rustc cargo rustracer rustfmt git];


  RUST_SRC_PATH = rustPlatform.rustcSrc;
}
