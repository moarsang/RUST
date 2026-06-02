#!/usr/bin/env bash
set -euo pipefail

# Rust setup script for Ubuntu 26.x students.
# Installs Rust with rustup, then verifies that cargo run and cargo test work.

info() {
  printf '\n[INFO] %s\n' "$*"
}

warn() {
  printf '\n[WARN] %s\n' "$*" >&2
}

fail() {
  printf '\n[ERROR] %s\n' "$*" >&2
  exit 1
}

tmp_installer=""
workdir=""
cleanup() {
  if [[ -n "${tmp_installer}" ]]; then
    rm -f "$tmp_installer"
  fi
  if [[ -n "${workdir}" ]]; then
    rm -rf "$workdir"
  fi
}
trap cleanup EXIT

if [[ "${EUID}" -eq 0 ]]; then
  SUDO=""
else
  command -v sudo >/dev/null 2>&1 || fail "sudo is required. Please install sudo or run this from an admin account."
  SUDO="sudo"
fi

if [[ -r /etc/os-release ]]; then
  # shellcheck disable=SC1091
  . /etc/os-release
  if [[ "${ID:-}" != "ubuntu" ]]; then
    warn "This script is intended for Ubuntu 26.x, but this system reports ID=${ID:-unknown}."
  elif [[ "${VERSION_ID:-}" != 26.* ]]; then
    warn "This script is intended for Ubuntu 26.x, but this system reports VERSION_ID=${VERSION_ID:-unknown}."
  fi
else
  warn "Could not read /etc/os-release; continuing anyway."
fi

info "Installing Ubuntu packages needed by Rust and common Cargo projects."
$SUDO apt-get update
$SUDO apt-get install -y \
  build-essential \
  ca-certificates \
  curl \
  git \
  libssl-dev \
  pkg-config

export CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export RUSTUP_HOME="${RUSTUP_HOME:-$HOME/.rustup}"
export PATH="$CARGO_HOME/bin:$PATH"

if command -v rustup >/dev/null 2>&1; then
  info "rustup is already installed. Updating the stable toolchain."
  rustup self update || true
  rustup toolchain install stable
else
  info "Installing Rust with rustup."
  tmp_installer="$(mktemp)"
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o "$tmp_installer"
  sh "$tmp_installer" -y --default-toolchain stable --profile default
fi

# Make cargo/rustc available in this script and in future login shells.
if [[ -f "$CARGO_HOME/env" ]]; then
  # shellcheck disable=SC1091
  . "$CARGO_HOME/env"
fi

for profile in "$HOME/.profile" "$HOME/.bashrc"; do
  touch "$profile"
  if ! grep -Fq '. "$HOME/.cargo/env"' "$profile"; then
    {
      printf '\n# Rust toolchain environment\n'
      printf 'if [ -f "$HOME/.cargo/env" ]; then\n'
      printf '  . "$HOME/.cargo/env"\n'
      printf 'fi\n'
    } >> "$profile"
  fi
done

info "Selecting stable Rust and installing standard components."
rustup default stable
rustup component add rustfmt clippy

info "Verifying cargo run and cargo test with a temporary project."
workdir="$(mktemp -d)"

cargo new "$workdir/rust_setup_check" --bin --quiet
cat > "$workdir/rust_setup_check/src/main.rs" <<'RS'
fn add(left: i32, right: i32) -> i32 {
    left + right
}

fn main() {
    println!("Rust setup check: {}", add(20, 22));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
    }
}
RS

(
  cd "$workdir/rust_setup_check"
  cargo run --quiet
  cargo test --quiet
)

info "Rust installation completed."
rustc --version
cargo --version
rustup --version

printf '\nOpen a new terminal, or run this now:\n'
printf '  source "$HOME/.cargo/env"\n\n'
printf 'Then student projects should work with:\n'
printf '  cargo run\n'
printf '  cargo test\n'
