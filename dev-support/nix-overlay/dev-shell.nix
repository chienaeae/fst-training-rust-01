{ pkgs, stdenv, mkShell }:

let
  llvmPackages = pkgs.llvmPackages_13;
  clang-tools = pkgs.clang-tools.override { inherit llvmPackages; };
  nodejs = pkgs.nodejs-16_x;
  yarn = pkgs.yarn.override { inherit nodejs; };
in
mkShell.override { stdenv = llvmPackages.libcxxStdenv; } {
  buildInputs = with pkgs;
    [
      clang-tools
      codespell
      nixpkgs-fmt

      yarn
      nodePackages."@commitlint/cli"
      nodePackages.prettier
      nodePackages.sql-formatter

      convco

      gitAndTools.git-extras
      gitAndTools.pre-commit
      tokei

      rustup
      sccache
      cargo-deny
      cargo-edit
      cargo-udeps

      cmake
      pkg-config

      openssl

      # shell
      checkbashisms
      shfmt

      # Helm chart testing
      chart-testing
      kubernetes-helm
      yamale
      yamllint

      # TODO: figure out who use libiconv
      libiconv
    ] ++ lib.optionals stdenv.isDarwin [
      darwin.apple_sdk.frameworks.SystemConfiguration
    ] ++ lib.optionals (!stdenv.isDarwin || !stdenv.isAarch64) [
      # ref: https://github.com/NixOS/nixpkgs/blob/nixos-22.05/pkgs/development/python-modules/pyopenssl/default.nix#L78
      # issue: https://github.com/pyca/pyopenssl/issues/873
      shellcheck
    ] ++ lib.optionals (stdenv.isx86_64 && stdenv.isLinux) [
      # Officially cargo-tarpaulin only supports x86_64-linux (ref: https://github.com/NixOS/nixpkgs/pull/173049)
      cargo-tarpaulin
    ];

  shellHook = ''
    export PATH=$PWD/dev-support/bin:$PWD/target/release:$PWD/target/debug:$PATH
  '';

  RUST_BACKTRACE = 1;

  COMMITLINT_PRESET = "${
  pkgs.nodePackages."@commitlint/config-conventional"
  }/lib/node_modules/@commitlint/config-conventional";
}
