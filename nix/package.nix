{
  lib,
  craneLib,
  stdenv,
  mold,
  versionCheckHook,
}: let
  pname = "searx-randomizer";
  inherit ((lib.importTOML ../Cargo.toml).package) version;
  src = let
    fs = lib.fileset;
    s = ../.;
  in
    fs.toSource {
      root = s;
      fileset = fs.unions [
        (fs.fileFilter (file: builtins.any file.hasExt ["rs"]) (s + /src))
        (s + /Cargo.lock)
        (s + /Cargo.toml)
      ];
    };

  cargoArtifacts = craneLib.buildDepsOnly {
    name = "${pname}-deps";
    strictDeps = true;
    inherit src;
  };
in
  craneLib.buildPackage {
    inherit pname src version cargoArtifacts;

    strictDeps = true;
    nativeInstallCheckInputs = [versionCheckHook];
    doInstallCheck = true;

    env = lib.optionalAttrs (stdenv.isLinux && !stdenv.hostPlatform.isAarch) {
      CARGO_LINKER = "clang";
      CARGO_RUSTFLAGS = "-Clink-arg=-fuse-ld=${mold}/bin/mold";
    };

    meta = {
      description = "Searx(ng) instance randomizer for Schizofox ";
      homepage = "https://github.com/schizofox/searx-randomizer";
      license = lib.licenses.mpl20;
      maintainers = [lib.maintainers.NotAShelf];
      mainProgram = "searx-randomizer";
    };
  }
