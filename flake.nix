{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-23.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
      rust = pkgs.rust-bin.stable.latest.default.override {
        extensions = [ "rust-src" ];
        targets = [ "wasm32-unknown-unknown" ];
      };
      lib = pkgs.lib;
      # rustBuildDeps = with pkgs; [
      # ];
    in
    {
      devShell = with pkgs;
        mkShell {
          nativeBuildInputs = [
            rust
            trunk
            wasm-bindgen-cli


            pkg-config llvmPackages.bintools
            openssl cmake zstd
            vulkan-tools
            xorg.libX11 xorg.libXcursor xorg.libXrandr xorg.libXi
            udev alsa-lib
          ];
        };
    });
}
