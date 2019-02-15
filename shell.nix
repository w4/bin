with import <nixpkgs> {};
let src = fetchFromGitHub {
      owner = "mozilla";
      repo = "nixpkgs-mozilla";
      # commited 12/2/2019
      rev = "37f7f33ae3ddd70506cd179d9718621b5686c48d";
      sha256 = "0cmvc9fnr38j3n0m4yf0k6s2x589w1rdby1qry1vh435v79gp95j";
   };
in
with import "${src.out}/rust-overlay.nix" pkgs pkgs;

stdenv.mkDerivation {
    name = "bin";
    buildInputs = [
        latest.rustChannels.nightly.cargo
        latest.rustChannels.nightly.rust
    ] ++
    pkgs.lib.optional pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.Security;
}
