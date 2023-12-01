{
  description = "Description for the project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    flake-parts,
    crane,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];
      perSystem = {
        pkgs,
        system,
        self',
        ...
      }: let
        craneLib = crane.lib.${system};
        searx-instance-randomizer = pkgs.callPackage ./default.nix {inherit craneLib;};
      in {
        checks = {
          inherit searx-instance-randomizer;
        };

        packages = {
          default = searx-instance-randomizer;
        };

        devShells.default = craneLib.devShell {
          inherit (self') checks;
        };
      };
    };
}
