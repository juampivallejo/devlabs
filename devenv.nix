{ pkgs, ... }:

{
  env.GREET = "devenv";
  env.DATABASE_URL = "postgres://user:password@localhost:5432/devlabs";
  env.RUST_LOG = "debug";
  cachix.enable = false;

  packages = with pkgs; [
    git
    cargo
    sqlite
    sqlx-cli

    # IDE
    rust-analyzer
    rustfmt
    clippy
    vscode-extensions.vadimcn.vscode-lldb.adapter # For DAP
  ];

  languages.rust.enable = true;

  processes.cargo-watch.exec = "cargo-watch";

  # services.postgresql.enable = true;

  # https://devenv.sh/scripts/
  scripts.migrate.exec = ''
    echo "Running migrations..."
    output=$(cargo sqlx migrate run 2>&1)
    if [ -z "$output" ]; then
      echo "No migrations to run."
    else
      echo "$output"
      echo "Migrations completed successfully."
    fi
  '';

  enterShell = ''
    cargo --version
  '';

  # https://devenv.sh/tests/
  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
  '';

  # See full reference at https://devenv.sh/reference/options/
}
