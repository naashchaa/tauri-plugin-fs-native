{
  description = "silly wardrobe flake";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";

    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            xdg-utils
            desktop-file-utils
            wrapGAppsHook4
            rustup
            cargo-tauri
            nodejs
            pnpm
          ];

          buildInputs = with pkgs; [
            pkg-config
            xdg-utils
            desktop-file-utils
            librsvg
            gtk3
            webkitgtk_4_1
          ];

          GIO_MODULE_DIR = "${pkgs.glib-networking}/lib/gio/modules/";

          shellHook = ''
            export XDG_DATA_DIRS="$GSETTINGS_SCHEMAS_PATH" # Needed on Wayland to report the correct display scale

            # fixes for some wayland nvidia bug
            export WEBKIT_DISABLE_DMABUF_RENDERER=1
          '';
        };
      }
    );
}
