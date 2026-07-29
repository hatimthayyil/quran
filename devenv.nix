_: {
  languages.rust.enable = true;

  treefmt = {
    enable = true;
    config.programs = {
      deadnix.enable = true;
      statix.enable = true;
      rustfmt.enable = true;
    };
  };

  git-hooks.hooks = {
    deadnix.enable = true;
    statix.enable = true;
    convco.enable = true;
  };
}
