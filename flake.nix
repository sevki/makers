{
  description = "make-sys: c2rust port of GNU make";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        # Reads channel + components from rust-toolchain.toml.
        # On first run nix will tell you the real hash to paste here.
        rustToolchain = fenix.packages.${system}.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = pkgs.lib.fakeSha256;
        };

        # Native deps needed to bootstrap + configure + build lib/libgnu.a.
        nativeDeps = with pkgs; [
          rustToolchain
          gcc
          gnumake
          autoconf
          automake
          libtool
          gettext
          gperf
          m4
          perl
          pkg-config
          texinfo
          wget        # bootstrap pulls auxiliary files
          git         # bootstrap inspects the tree
        ];
      in {
        devShells.default = pkgs.mkShell {
          packages = nativeDeps;

          # Build lib/libgnu.a on shell entry if it's missing — build.rs links
          # it via `cargo:rustc-link-lib=static=gnu` from `<manifest>/lib`.
          shellHook = ''
            if [ ! -f lib/libgnu.a ]; then
              echo "==> lib/libgnu.a missing; running bootstrap + configure + build.sh"
              if [ ! -f configure ]; then
                ./bootstrap --skip-po --no-git --gnulib-srcdir=./gl || \
                  ./bootstrap --skip-po || true
              fi
              if [ ! -f build.cfg ]; then
                ./configure
              fi
              sh ./build.sh || echo "build.sh failed; you may need to build lib/libgnu.a manually"
            fi
          '';
        };

        # `nix build .#libgnu` — produces just lib/libgnu.a as an artifact.
        packages.libgnu = pkgs.stdenv.mkDerivation {
          pname = "make-libgnu";
          version = "0.0.0";
          src = ./.;
          nativeBuildInputs = with pkgs; [
            autoconf automake libtool gettext gperf m4 perl pkg-config texinfo
          ];
          dontUseCmakeConfigure = true;
          configurePhase = ''
            ./bootstrap --skip-po --no-git --gnulib-srcdir=./gl || ./bootstrap --skip-po
            ./configure
          '';
          buildPhase = ''
            sh ./build.sh
          '';
          installPhase = ''
            mkdir -p $out/lib
            cp lib/libgnu.a $out/lib/
          '';
        };
      });
}
