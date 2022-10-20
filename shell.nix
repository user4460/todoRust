#shell.nixとは、nix-shellで実行されるコマンドを記述するファイルです。
#nixとは、NixOSのシェルを使うための設定ファイルです。
{ dappPkgs ? (
    import (fetchTarball "https://github.com/makerdao/makerpkgs/tarball/master") {}
  ).dappPkgsVersions.hevm-0_43_1
}: with dappPkgs;

#mkShellとは、nix-shellで実行されるコマンドを記述する関数です。
mkShell {
  DAPP_SOLC = solc-static-versions.solc_0_6_12 + "/bin/solc-0.6.12";
  DAPP_TEST_ADDRESS = "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B";
  # No optimizations
  SOLC_FLAGS = "";
  buildInputs = [
    dapp
  ];
}