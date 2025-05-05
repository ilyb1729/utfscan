let
  rustOverlay = import (builtins.fetchTarball https://github.com/oxalica/rust-overlay/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ rustOverlay ]; };
in
pkgs.mkShell {
  buildInputs = [
    pkgs.rust-bin.stable.latest.default
    pkgs.pkg-config
    pkgs.openssl
    pkgs.git
  ];

  shellHook = ''
    echo "Welcome to the utfscan dev shell!"
    export TMPDIR="/tmp"
    export PATH="$HOME/.local/share/nvim/mason/bin:$PATH"
  '';
}
