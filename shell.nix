{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.pkg-config
    pkgs.openssl
  ];

  shellHook = ''
    echo "Welcome to the utfscan dev shell!"
  '';
}
