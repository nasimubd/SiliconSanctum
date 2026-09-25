#!/bin/sh
set -eu

repository="nasimubd/SiliconSanctum"
install_dir="${SANCTUM_INSTALL_DIR:-$HOME/.local/bin}"
version="${SANCTUM_VERSION:-}"

if [ "$(uname -s)" != "Darwin" ]; then
  echo "The prebuilt installer currently supports macOS only; use Docker or build from source on this platform." >&2
  exit 1
fi

case "$(uname -m)" in
  arm64) target="aarch64-apple-darwin" ;;
  x86_64) target="x86_64-apple-darwin" ;;
  *) echo "Unsupported macOS architecture: $(uname -m)" >&2; exit 1 ;;
esac

if [ -z "$version" ]; then
  version="$(curl -fsSL "https://api.github.com/repos/$repository/releases/latest" \
    | sed -n 's/.*"tag_name": *"\(v[^"]*\)".*/\1/p' | head -n 1)"
fi

case "$version" in
  v[0-9]*.[0-9]*.[0-9]*) : ;;
  *) echo "Could not determine a released version; set SANCTUM_VERSION, for example v1.15.2." >&2; exit 1 ;;
esac

asset="silicon-sanctum-${version#v}-${target}"
base_url="https://github.com/$repository/releases/download/$version"
temporary_dir="$(mktemp -d)"
trap 'rm -rf "$temporary_dir"' EXIT INT TERM

curl -fsSL "$base_url/$asset.tar.gz" -o "$temporary_dir/$asset.tar.gz"
curl -fsSL "$base_url/$asset.sha256" -o "$temporary_dir/$asset.sha256"
expected="$(awk '{print $1}' "$temporary_dir/$asset.sha256")"
actual="$(shasum -a 256 "$temporary_dir/$asset.tar.gz" | awk '{print $1}')"
[ "$expected" = "$actual" ] || {
  echo "Checksum verification failed for $asset.tar.gz" >&2
  exit 1
}

mkdir -p "$install_dir"
tar -xzf "$temporary_dir/$asset.tar.gz" -C "$temporary_dir"
install -m 0755 "$temporary_dir/$asset/sanctum" "$install_dir/sanctum"
for command in serve claude codex opencode aider doctor benchmark; do
  ln -sfn "$install_dir/sanctum" "$install_dir/sanctum-$command"
done

echo "Installed Silicon Sanctum $version to $install_dir"
case ":${PATH:-}:" in
  *:"$install_dir":*) : ;;
  *) echo "Add $install_dir to PATH before running sanctum-serve." >&2 ;;
esac
