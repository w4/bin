with import <nixpkgs> {};
let src = fetchFromGitHub {
      owner = "mozilla";
      repo = "nixpkgs-mozilla";
      # commited 19/2/2020
      rev = "e912ed483e980dfb4666ae0ed17845c4220e5e7c";
      sha256 = "0cmvc9fnr38j3n0m4yf0k6s2x589w1rdby1qry1vh435v79gp95j";
   };
in
with import "${src.out}/rust-overlay.nix" pkgs pkgs;

stdenv.mkDerivation {
    name = "bin";
    buildInputs = [
        latest.rustChannels.nightly.cargo
        latest.rustChannels.nightly.rust
        stdenv.cc.libc
    ] ++
    pkgs.lib.optional pkgs.stdenv.isDarwin pkgs.darwin.apple_sdk.frameworks.Security;

    LIBCLANG_PATH="${llvmPackages.libclang}/lib";
}
