{
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  inputs.crane.url = "github:ipetkov/crane";
  inputs.crane.inputs.nixpkgs.follows = "nixpkgs";
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.flake-utils.inputs.nixpkgs.follows = "nixpkgs";

  description = "A creative coding framework for Rust";

  outputs = {self, nixpkgs, crane, flake-utils, ...}:
    flake-utils.lib.eachDefaultSystem (system: 
      let
        # Import nixpkgs and create the craneLib using the Rust toolchain in nixpkgs
        pkgs = import nixpkgs {
          inherit system;
        };
        inherit (pkgs) lib;
        craneLib = crane.mkLib pkgs;
        
        # Define the project source
        src = lib.cleanSourceWith {
          src = ./.;  # The original, unfiltered source
          filter = (path: type: 
            # Include anything in the "assets" folder
            (lib.hasInfix "/assets/" path) ||
            # Include shader files
            (lib.hasSuffix "\.wgsl" path) ||
            # Default filter from crane e.g. allow ".rs" files
            (craneLib.filterCargoSources path type)
          );
        };

        # Define common args to be used throughout the workspace
        commonArgs = {
          inherit src;
          pname = "nannou-workspace";
          version = "0.18.1";

          # nativeBuildInputs = [
          #   pkgs.mdbook
          # ];
          nativeBuildInputs = [
            pkgs.cmake
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            # Needed for `sw_vers` at build time
            pkgs.darwin.DarwinTools
          ];

          buildInputs = [
            pkgs.openssl
          ] ++ lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
            pkgs.darwin.apple_sdk.frameworks.Security
            pkgs.darwin.apple_sdk.frameworks.QuartzCore
            pkgs.darwin.apple_sdk.frameworks.AppKit
          ];
        };

        # Build *just* the Cargo dependencies
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;

        # Build the whole `nannou` crate
        nannou = craneLib.buildPackage (commonArgs // {
          inherit cargoArtifacts;
          pname = "nannou";
        });
      in
      {
        packages.nannou = nannou;
        packages.default = self.packages.${system}.nannou;

        devShells.default = craneLib.devShell {
          inputsFrom = [self.packages.${system}.nannou];
          packages = [
            pkgs.mdbook
          ];
        };
      }
    );
    # let
    #   supportedSystems = ["aarch64-darwin"];
    #   forEachSupportedSystem = f: nixpkgs.lib.genAttrs supportedSystems (system: f {
    #     inherit system;
    #     pkgs = nixpkgs.legacyPackages.${system};
    #   });
    # in
    # {
    #   packages = forEachSupportedSystem ({system, pkgs}: 
    #     let
    #       foo = "bar";
    #     in
    #     {
    #       default = pkgs.rustPlatform.buildRustPackage {
    #         pname = "NAME";
    #         version = "0.0.1";

    #         cargoLock.lockFile = ./Cargo.lock;

    #         buildInputs = with pkgs; [
    #           rustc
    #           cargo
    #         ] ++ pkgs.lib.optional pkgs.hostPlatform.isDarwin [
    #           libiconv
    #           darwin.apple_sdk.frameworks.Security
    #         ];
    #       };
    #     });
    # };
}
