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
				];
			};
		}
	);
}
