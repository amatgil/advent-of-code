{
  description = "Default uiua flake (casenc)";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    uiua.url = "github:uiua-lang/uiua/main";
  };
  outputs =
    { self, nixpkgs, uiua }:
    let
      supportedSystems = [ "x86_64-linux" ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgs = (import nixpkgs { system = "x86_64-linux"; });
    in
    {
      devShells = forAllSystems (system: {
        default = pkgs.callPackage ./shell.nix { inherit pkgs; uiua = uiua.packages.${system}.default; };
      });
    };
}
