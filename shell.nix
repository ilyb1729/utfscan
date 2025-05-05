{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.pkg-config
    pkgs.openssl
    pkgs.git
  ];

  shellHook = ''
    echo "Welcome to the utfscan dev shell!"

    # Set TMPDIR environment variable
    export TMPDIR="/tmp"

    # Install stable Rust toolchain if not already installed
    if [ ! -f "$HOME/.cargo/bin/rustc" ]; then
      rustup install stable
    fi

    # Set the default toolchain to stable
    rustup default stable

    # Add cargo and rustup to the PATH if needed
    export PATH="$HOME/.local/share/nvim/mason/bin:$HOME/.cargo/bin:$PATH"
  '';
}
