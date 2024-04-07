{ pkgs ? import <nixpkgs> { } }: pkgs.mkShell {
  buildInputs = [ ];
  packages = with pkgs; [ opentofu protobuf ];
}
