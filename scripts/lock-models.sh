#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"

mkdir -p "$repo_dir/config/locks"
for source_file in "$AI_ROOT"/manifests/*.(json|Modelfile)(N); do
  [[ -s "$source_file" ]] || continue
  name="${source_file:t}"
  destination="$repo_dir/config/locks/$name"
  if [[ "$source_file" -nt "$destination" || ! -f "$destination" ]]; then
    cp "$source_file" "$destination"
    print -- "locked $name"
  fi
done
