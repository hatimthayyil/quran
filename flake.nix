{
  description = "quran: print Quranic text for a reference query";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      inherit (nixpkgs) lib;
      forAllSystems = lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          quran = pkgs.rustPlatform.buildRustPackage {
            pname = "quran";
            version = (lib.importTOML ./crates/quran/Cargo.toml).package.version;
            src = lib.fileset.toSource {
              root = ./.;
              fileset = lib.fileset.unions [
                ./Cargo.toml
                ./Cargo.lock
                ./crates
                ./data
              ];
            };
            cargoLock.lockFile = ./Cargo.lock;
            meta = {
              description = "Print Quranic text for a reference query";
              license = lib.licenses.cc0;
              mainProgram = "quran";
            };
          };
        in
        {
          inherit quran;
          default = quran;
        }
      );
    };
}
