{
  description = "archive-drive dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        # Cross toolchains — for local cross-compile sanity checks only.
        # Final CI builds happen natively per-arch, these are just to
        # catch obvious breakage before pushing.
        pkgsAarch64 = import nixpkgs {
          inherit system;
          crossSystem = { config = "aarch64-unknown-linux-gnu"; };
        };
        pkgsRiscv64 = import nixpkgs {
          inherit system;
          crossSystem = { config = "riscv64-unknown-linux-gnu"; };
        };
      in
      {
        devShells.default = pkgs.mkShell {
          name = "archive-drive";
          buildInputs = with pkgs; [
            rustc
            cargo
            clippy
            rust-analyzer
            rustfmt
            pkg-config
          ];
          RUST_BACKTRACE = "1";
        };

        # Enter with: nix develop .#arm64
        devShells.arm64 = pkgs.mkShell {
          name = "archive-drive-arm64";
          buildInputs = [ pkgs.cargo pkgs.rustc ];
          nativeBuildInputs = [ pkgsAarch64.stdenv.cc ];
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER =
            "${pkgsAarch64.stdenv.cc}/bin/${pkgsAarch64.stdenv.cc.targetPrefix}cc";
        };

        # Enter with: nix develop .#riscv64
        devShells.riscv64 = pkgs.mkShell {
          name = "archive-drive-riscv64";
          buildInputs = [ pkgs.cargo pkgs.rustc ];
          nativeBuildInputs = [ pkgsRiscv64.stdenv.cc ];
          CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_LINKER =
            "${pkgsRiscv64.stdenv.cc}/bin/${pkgsRiscv64.stdenv.cc.targetPrefix}cc";
        };
      });
}
