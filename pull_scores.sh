#!/bin/bash
set -euo pipefail

PASSWORD="${1:?Usage: $0 <password>}"
BASE_URL="http://localhost:41549"

# Get all tournaments
TOURNAMENTS=$(curl -sf "$BASE_URL/api/tournaments")
COUNT=$(echo "$TOURNAMENTS" | jq 'length')

if [ "$COUNT" -eq 0 ]; then
  echo "No tournaments found." >&2
  exit 1
fi

# Display tournaments
echo "Tournaments:"
for i in $(seq 0 $((COUNT - 1))); do
  NAME=$(echo "$TOURNAMENTS" | jq -r ".[$i].name")
  START=$(echo "$TOURNAMENTS" | jq -r ".[$i].start_date")
  IS_ACTIVE=$(echo "$TOURNAMENTS" | jq -r ".[$i].is_active")
  ACTIVE_FLAG=""
  if [ "$IS_ACTIVE" = "true" ]; then
    ACTIVE_FLAG=" [ACTIVE]"
  fi
  echo "  $((i + 1))) $NAME  ($START)$ACTIVE_FLAG"
done

echo ""
read -rp "Select tournament [1-$COUNT]: " SELECTION

if ! [[ "$SELECTION" =~ ^[0-9]+$ ]] || [ "$SELECTION" -lt 1 ] || [ "$SELECTION" -gt "$COUNT" ]; then
  echo "Invalid selection." >&2
  exit 1
fi

IDX=$((SELECTION - 1))
TOURNAMENT_ID=$(echo "$TOURNAMENTS" | jq -r ".[$IDX].id")
TOURNAMENT_NAME=$(echo "$TOURNAMENTS" | jq -r ".[$IDX].name")

echo ""
echo "Pulling scores for: $TOURNAMENT_NAME"
curl -s -X POST \
  -H "Authorization: Basic $(echo -n "admin:$PASSWORD" | base64)" \
  "$BASE_URL/api/admin/tournaments/$TOURNAMENT_ID/scores/refresh"
echo ""
