{craneLib, ...}:
craneLib.buildPackage {
  src = craneLib.cleanCargoSource (craneLib.path ./.);
  strictDeps = true;

  buildInputs = [];
}
