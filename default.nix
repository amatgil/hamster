{
  pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-unstable.tar.gz") {}
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
    pkgs.xorg.libxcb

    pkgs.libxkbcommon

    pkgs.llvmPackages.libclang
    pkgs.llvmPackages.libcxxClang
    pkgs.clang

    pkgs.pkg-config


    pkgs.libxkbcommon
    pkgs.vulkan-loader
    pkgs.wayland
  ];

  library_path = pkgs.lib.makeLibraryPath packages;
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = manifest.name;
  version = manifest.version;
  cargoHash = pkgs.lib.fakeHash;
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
  meta.description = manifest.description ? null;


  postInstall = ''
    patchelf --add-needed $out/bin/${pname} libxkbcommon.so      
    patchelf --add-needed $out/bin/${pname} libwayland-client.so 
    patchelf --add-needed $out/bin/${pname} libvulkan.so         
    patchelf --add-needed $out/bin/${pname} libX11.so.6
    patchelf --add-needed $out/bin/${pname} libXlib.so           
    patchelf --add-needed $out/bin/${pname} libXcursor.so        
    patchelf --add-needed $out/bin/${pname} libXi.so             
    patchelf --add-needed $out/bin/${pname} libvulkan.so         

    patchelf --add-rpath ${library_path} $out/bin/${pname}
'';
  nativeBuildInputs = packages;
  buildInputs = packages;

  LIBCLANG_PATH = with pkgs; "${llvmPackages.libclang.lib}/lib";
  BINDGEN_EXTRA_CLANG_ARGS =
    with pkgs;
    "-isystem ${llvmPackages.libclang.lib}/lib/clang/${lib.versions.major (lib.getVersion clang)}/include";


}
