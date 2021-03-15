# https://eipi.xyz/blog/packaging-a-rust-project-for-nix/

with import <nixpkgs> { };

rustPlatform.buildRustPackage rec {
  name = "zuma-${version}";
  version = "0.1.1";

  # We don't want to include `target` directory,
  #  as it is too large and useless.
  # https://discourse.nixos.org/t/mkderivation-src-as-list-of-filenames/3537/9
  srcs = [
    ./src
    ./Cargo.toml
    ./Cargo.lock
  ];

  unpackPhase = ''
    for srcFile in $srcs; do
      cp -r $srcFile $(stripHash $srcFile)
    done
  '';

  checkPhase = "";
  cargoSha256 = "000fkidvxp0g2bfcg79wlcq16gdsf0j2vryd16n4d8ng3gai4g4g";

  meta = with stdenv.lib; {
    description = "ZUMA Vector Graphics Language";
    homepage = https://github.com/vctibor/Zuma;
    license = licenses.gpl3Only;
    maintainers = [ "Vladimír Ctibor <vladimir.ctibor@gmail.com>" ];
    platforms = platforms.all;
  };
}