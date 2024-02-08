{
	description = "Bevy Pong";

	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
		utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, utils }:
	utils.lib.eachDefaultSystem(system:
		let
			pkgs = nixpkgs.legacyPackages.${system};
		in {
			devShells.default = pkgs.mkShell {
				buildInputs = [
					pkgs.cargo
					pkgs.rustc
					pkgs.rust-analyzer
					pkgs.rustfmt
				];
				nativeBuildInputs = [
					pkgs.pkg-config
					pkgs.alsa-lib
					pkgs.libudev-zero
					pkgs.xorg.libX11
					pkgs.xorg.libXcursor
					pkgs.xorg.libXrandr
					pkgs.xorg.libXi
					pkgs.vulkan-loader
				];
				shellHook = ''
					export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
						pkgs.lib.makeLibraryPath [
							pkgs.udev
							pkgs.alsaLib
							pkgs.vulkan-loader
						]
					}"
				'';
			};
		}
	);
}
