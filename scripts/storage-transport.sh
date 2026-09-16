#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"

# Emit a deliberately small, machine-readable transport record.  The drive's
# serial number is not included: benchmark files may be shared for review.
device_label="${AI_STORAGE_DEVICE_LABEL:-TS-CM10G}"
inventory="$(system_profiler SPThunderboltDataType SPUSBDataType 2>/dev/null || true)"
connection="unknown"
max_link_gbps="null"
thunderbolt_device_connected=false

if print -- "$inventory" | rg -q 'Thunderbolt/USB4 Bus'; then
  if ! print -- "$inventory" | rg -q 'Status: No device connected'; then
    thunderbolt_device_connected=true
  fi
fi

if print -- "$inventory" | rg -Fq "$device_label"; then
  connection="usb"
  speed="$(print -- "$inventory" | awk -v label="$device_label" '
    $0 ~ label { found=1; next }
    found && /Speed: Up to [0-9]+ Gb\/s/ { sub(/^.*Speed: Up to /, ""); sub(/ Gb\/s.*$/, ""); print; exit }
  ')"
  [[ "$speed" == <-> ]] && max_link_gbps="$speed"
elif [[ "$thunderbolt_device_connected" == true ]]; then
  connection="thunderbolt_or_usb4"
fi

jq -n \
  --arg device_label "$device_label" \
  --arg connection "$connection" \
  --argjson max_link_gbps "$max_link_gbps" \
  --argjson thunderbolt_device_connected "$thunderbolt_device_connected" \
  '{device_label:$device_label,connection:$connection,max_link_gbps:$max_link_gbps,thunderbolt_device_connected:$thunderbolt_device_connected}'
