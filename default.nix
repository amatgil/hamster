{
  pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz") { },
}:
let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  packages = [
    pkgs.mold
    pkgs.cmake
    pkgs.libGL

    # X11 dependencies
    pkgs.xorg.libX11
    pkgs.xorg.libX11.dev
    pkgs.xorg.libXcursor
    pkgs.xorg.libXi
    pkgs.xorg.libXinerama
    pkgs.xorg.libXrandr
    pkgs.xorg.libxcb

    pkgs.llvmPackages.libclang
    pkgs.llvmPackages.libcxxClang
    pkgs.clang

    pkgs.pkg-config

    pkgs.libxkbcommon
    pkgs.vulkan-loader
    pkgs.wayland
    pkgs.glfw
  ];

  library_path = pkgs.lib.makeLibraryPath packages;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
  meta.description = manifest.description ? null;

  postInstall = ''
    patchelf $out/bin/${pname} --add-needed libxkbcommon.so
    patchelf $out/bin/${pname} --add-needed libwayland-client.so
    patchelf $out/bin/${pname} --add-needed libvulkan.so
    patchelf $out/bin/${pname} --add-needed libX11.so.6
    patchelf $out/bin/${pname} --add-needed libXcursor.so
    patchelf $out/bin/${pname} --add-needed libXi.so
    patchelf $out/bin/${pname} --add-needed libvulkan.so
    patchelf $out/bin/${pname} --add-needed libGLX.so

    patchelf $out/bin/${pname} --add-rpath ${library_path}
  '';
  nativeBuildInputs = packages;
  buildInputs = packages;

  LIBCLANG_PATH = with pkgs; "${llvmPackages.libclang.lib}/lib";
  BINDGEN_EXTRA_CLANG_ARGS =
    with pkgs;
    "-isystem ${llvmPackages.libclang.lib}/lib/clang/${lib.versions.major (lib.getVersion clang)}/include";

}
