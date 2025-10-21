{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    protobuf
  ];

  shellHook = ''
    echo "Development environment loaded"
    echo "- protoc version: $(protoc --version)"
  '';
}
