with import <nixpkgs> {};
let src = fetchFromGitHub {
      owner = "mozilla";
      repo = "nixpkgs-mozilla";
      # commited 19/2/2020
      rev = "e912ed483e980dfb4666ae0ed17845c4220e5e7c";
      sha256 = "08fvzb8w80bkkabc1iyhzd15f4sm7ra10jn32kfch5klgl0gj3j3";
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
