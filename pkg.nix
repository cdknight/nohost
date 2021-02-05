# Blatantly copied from https://github.com/cargo2nix/cargo2nix/blob/master/examples/2-bigger-project/default.nix
{
  system ? builtins.currentSystem,
  nixpkgsMozilla ? builtins.fetchGit {
    url = https://github.com/mozilla/nixpkgs-mozilla;
    rev = "18cd4300e9bf61c7b8b372f07af827f6ddc835bb";
  },
  cargo2nix ? builtins.fetchGit {
    url = https://github.com/cargo2nix/cargo2nix;
    rev = "4bbd3137ff1422ef3565748eae33efe6e2ffbf39";
  },
  ... 
}:
let
  rustOverlay = import "${nixpkgsMozilla}/rust-overlay.nix";
  cargo2nixOverlay = import "${cargo2nix}/overlay";

  pkgs = import <nixpkgs> {
    inherit system;
    overlays = [ cargo2nixOverlay rustOverlay ];
  };

  rustPkgs = pkgs.rustBuilder.makePackageSet' {
    rustChannel = "1.49.0";
    # rustChannelSha256 is not nessesary, just for example
    packageFun = import ./Cargo.nix;
    # packageOverrides = pkgs: pkgs.rustBuilder.overrides.all; # Implied, if not specified
  };
in
  rustPkgs.workspace.nohost {}
