{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [ geckodriver cargo rustc rustfmt pre-commit rustPackages.clippy ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
          # shellHook = ''
            # ${pkgs.geckodriver}/bin/geckodriver  -p 9515 &
          # '';

          configurePhase = ''
          '';

          buildPhase = ''
            ${pkgs.geckodriver}/bin/geckodriver  -p 9515 &
            cargo build
          '';

          installPhase = ''
          '';
        };
      });
}
