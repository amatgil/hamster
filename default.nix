{
  pkgs ? import <nixpkgs> { },
}:
let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  packages = [ 
    pkgs.mold
    pkgs.cmake
    pkgs.pkgs.libGL

    # X11 dependencies
    pkgs.xorg.libX11
    pkgs.xorg.libX11.dev
    pkgs.xorg.libXcursor
    pkgs.xorg.libXi
    pkgs.xorg.libXinerama
    pkgs.xorg.libXrandr
];
in
pkgs.rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;
  cargoHash = pkgs.lib.fakeHash;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
  meta.description = manifest.description ? null;

  nativeBuildInputs = packages;
  BuildInputs = packages;
}
