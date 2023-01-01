{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk/master";
  };

  outputs = { nixpkgs, naersk, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      naersk-lib = pkgs.callPackage naersk { };
    in
    {
      packages.${system}.default = naersk-lib.buildPackage ./.;

      devShells.${system}.default = with pkgs; mkShell {
        buildInputs = [ cargo rustc rustfmt rustPackages.clippy ];
        RUST_SRC_PATH = rustPlatform.rustLibSrc;
      };
    };
}
