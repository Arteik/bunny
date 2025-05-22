{ pkgs ? import <nixpkgs> { } }:
let
  # Build bunny with naersk
  naersk = pkgs.callPackage (pkgs.fetchFromGitHub {
    owner = "nix-community";
    repo = "naersk";
    rev = "master"; # TODO: Pin this, I guess
    sha256 = "sha256-cyAAMal0aPrlb1NgzMxZqeN1mAJ2pJseDhm2m6Um8T0=";
  }) { };

in naersk.buildPackage { src = ./.; }
