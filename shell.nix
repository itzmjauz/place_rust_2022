{ pkgs ? import (fetchTarball "https://github.com/NixOS/nixpkgs/archive/nixos-21.11.tar.gz") {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [rustc cargo pkgconfig alsa-lib xorg.libX11 libudev xorg.libXcursor xorg.libXrandr x11 xorg.libXi libGL];
  shellHook = ''
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
      pkgs.xorg.libXcursor
      pkgs.xorg.libXrandr
      pkgs.xorg.libXi
      pkgs.libGL
    ]}"'';
}
