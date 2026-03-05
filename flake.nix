{
  description = "Flake para compilação deste projeto no NixOS";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      devShells.${system}.default = pkgs.mkShell {
          buildInputs = with pkgs; [
              rustup
              cargo
              rust-analyzer
              trunk
              wasm-pack
              binaryen
              pkg-config
            ];

            shellHook = ''
              export RUSTUP_HOME=$PWD/.rustup
              export CARGO_HOME=$PWD/.cargo
              export PATH=$CARGO_HOME/bin:$PATH

              # idempotent: install stable toolchain + wasm target if missing
              rustup toolchain install stable >/dev/null 2>&1 || true
              rustup target add wasm32-unknown-unknown >/dev/null 2>&1 || true

              echo "Using rustup toolchain: $(rustc --version)"
            '';
      };
    };
}
