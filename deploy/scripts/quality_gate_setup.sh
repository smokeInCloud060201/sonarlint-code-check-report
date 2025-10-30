#!/usr/bin/env bash
set -euo pipefail

# Config
HOST="http://sonarcuteapi:8080"
API_BASE="$HOST/api"
GATE_NAME="Kiosk Gate"

require() {
  command -v "$1" >/dev/null 2>&1 || { echo "Error: $1 is required but not installed." >&2; exit 1; }
}

require curl
require jq

echo "[1/4] Creating quality gate: $GATE_NAME"
create_payload=$(jq -n --arg name "$GATE_NAME" '{name: $name}')
curl -sS --fail --location "$API_BASE/quality-gates" \
  --header 'Content-Type: application/json' \
  --data "$create_payload" >/dev/null

echo "[2/4] Fetching quality gate details"
encoded_name=${GATE_NAME// /%20}
details_json=$(curl -sS --fail --location "$API_BASE/quality-gates/details?name=$encoded_name")

echo "[info] Raw details received:" >&2
echo "$details_json" | jq '.' >&2 || true

echo "[3/4] Finding condition id for metric=new_coverage"
# Try to find any object with metric=="new_coverage" and read its id
new_cov_id=$(echo "$details_json" | jq -r '.. | objects | select(has("metric") and .metric=="new_coverage") | .id' | head -n1 || true)

if [[ -z "${new_cov_id:-}" || "$new_cov_id" == "null" ]]; then
  echo "[warn] No condition with metric=new_coverage found. Will only add coverage condition." >&2
  delete_ids_json='[]'
else
  echo "[info] Found new_coverage condition id: $new_cov_id" >&2
  delete_ids_json=$(jq -n --arg id "$new_cov_id" '[ $id ]')
fi

echo "[4/4] Updating quality gate: delete new_coverage, add coverage<80"
update_payload=$(jq -n \
  --arg name "$GATE_NAME" \
  --argjson delete_ids "$delete_ids_json" \
  '{
     name: $name,
     add_conditions: [ { metric: "coverage", op: "LT", error: "80" } ],
     delete_condition_ids: $delete_ids
   }')

curl -sS --fail --location --request PUT "$API_BASE/quality-gates" \
  --header 'Content-Type: application/json' \
  --data "$update_payload" | jq '.'

echo "[done] Quality gate update complete."


