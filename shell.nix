{ pkgs ? import <nixpkgs> {} }:

let

  fenix = import (fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz") {};

in

pkgs.mkShell {
  shellHook = ''export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
    pkgs.alsaLib
    pkgs.udev
    pkgs.vulkan-loader
  ]}"'';

  buildInputs = with pkgs; [
    (
      with fenix;
      combine (
        with default; [
          cargo
          clippy-preview
          latest.rust-src
          rust-analyzer
          rust-std
          rustc
          rustfmt-preview
          targets.wasm32-unknown-unknown.latest.rust-std
        ]
      )
    )
    cargo-edit
    cargo-watch
    cargo-make

    lld
    clang

    # # bevy-specific deps (from https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md)
    pkgconfig
    udev
    alsaLib
    lutris
    x11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    vulkan-tools
    vulkan-headers
    vulkan-loader
    vulkan-validation-layers

    # wasm(?) requirements
    openssl
    wasm-bindgen-cli
  ];

}
