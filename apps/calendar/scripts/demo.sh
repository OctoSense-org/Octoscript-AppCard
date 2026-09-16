#!/bin/bash
# Start the calendar sync server (SQLite) and a static server for the browser
# preview, then print the two device URLs. Ctrl-C stops both.
#   bash scripts/demo.sh            # server on 8190, preview on 8194
#   PYTHON=/path/to/python3 bash scripts/demo.sh
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PYTHON="${PYTHON:-${BEAUTY_PYTHON:-python3}}"
SERVER_PORT="${SERVER_PORT:-8190}"
PREVIEW_PORT="${PREVIEW_PORT:-8194}"
DB="$ROOT/runtime/calendar.sqlite"
mkdir -p "$ROOT/runtime"
"$PYTHON" "$ROOT/scripts/export_preview.py" >/dev/null
"$PYTHON" "$ROOT/server/calendar_server.py" --db "$DB" --port "$SERVER_PORT" --allow-origin "http://127.0.0.1:$PREVIEW_PORT" --allow-origin "http://localhost:$PREVIEW_PORT" &
SERVER_PID=$!
sleep 0.5
TOKEN="$(cat "$ROOT/runtime/calendar.token")"
( cd "$ROOT" && "$PYTHON" -m http.server "$PREVIEW_PORT" --bind 127.0.0.1 >/dev/null 2>&1 ) &
STATIC_PID=$!
trap 'kill $SERVER_PID $STATIC_PID 2>/dev/null' EXIT INT TERM
cat <<EOF
calendar sync server: http://127.0.0.1:$SERVER_PORT  (db $DB)
phone tab:   http://127.0.0.1:$PREVIEW_PORT/wizard/preview/?device=alex-phone&server=http://127.0.0.1:$SERVER_PORT&token=$TOKEN
desktop tab: http://127.0.0.1:$PREVIEW_PORT/wizard/preview/?device=sam-desktop&server=http://127.0.0.1:$SERVER_PORT&token=$TOKEN&locale=en
offline demo (no server): http://127.0.0.1:$PREVIEW_PORT/wizard/preview/
EOF
wait $SERVER_PID
