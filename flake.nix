{
  description = "Framework 16 LED Matrix widgets";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f (import nixpkgs { inherit system; }));
    in
    {
      homeManagerModules = {
        default = import ./nix/home-manager.nix { inherit self; };
        framework-led-matrix = import ./nix/home-manager.nix { inherit self; };
      };

      nixosModules = {
        default = import ./nix/nixos-module.nix { inherit self; };
        framework-led-matrix = import ./nix/nixos-module.nix { inherit self; };
      };

      packages = forAllSystems (
        pkgs:
        let
          framework-led-widgets = pkgs.rustPlatform.buildRustPackage {
            pname = "framework-led-widgets";
            version = "0.2.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = pkgs.lib.optionals pkgs.stdenv.isLinux [ pkgs.udev ];
          };
        in
        {
          default = framework-led-widgets;
          inherit framework-led-widgets;
        }
      );

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = [
            pkgs.rustc
            pkgs.cargo
            pkgs.rustfmt
            pkgs.clippy
            pkgs.pkg-config
          ]
          ++ pkgs.lib.optionals pkgs.stdenv.isLinux [ pkgs.udev ];
        };
      });
    };
}
