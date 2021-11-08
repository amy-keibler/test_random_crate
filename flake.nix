{
  description = "Fetch crates and run a command against them for testing purposes";

  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      # `nix build`
      packages.test_random_crate = naersk-lib.buildPackage {
        pname = "test_random_crate";
        root = ./.;
      };
      defaultPackage = packages.test_random_crate;

      # `nix run`
      apps.test_random_crate = utils.lib.mkApp {
        drv = packages.test_random_crate;
        exePath = "/bin/test_random_crate";
      };
      defaultApp = apps.test_random_crate;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo clippy rustfmt rust-analyzer ];

        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };
    });
}
