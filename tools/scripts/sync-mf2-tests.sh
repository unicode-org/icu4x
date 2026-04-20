#!/usr/bin/env bash
# Sync MessageFormat 2 conformance fixtures from a local unicode-org/message-format-wg clone.
# Invoked by: cargo make sync-mf2-tests
#
# Environment:
#   MESSAGE_FORMAT_WG  Path to the message-format-wg repository (default: ../message-format-wg
#                      relative to this ICU4X checkout).

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DEST="$ROOT/components/experimental/tests/messageformat/fixtures"

WG="${MESSAGE_FORMAT_WG:-}"
if [[ -z "$WG" ]]; then
    WG="$ROOT/../message-format-wg"
fi

if [[ ! -f "$WG/test/tests/syntax.json" ]]; then
    echo "error: WG tests not found under $WG/test/tests/" >&2
    echo "Clone https://github.com/unicode-org/message-format-wg and set MESSAGE_FORMAT_WG" >&2
    echo "to that directory, or place it at $ROOT/../message-format-wg" >&2
    exit 1
fi

mkdir -p "$DEST/tests" "$DEST/schemas"

# Preserve ICU4X-only witness rows (see fixtures/README.md) across rsync --delete.
BACKUP_DIR="$DEST/.icu4x-fixture-backup"
mkdir -p "$BACKUP_DIR/functions"
for f in number.json currency.json; do
    if [[ -f "$DEST/tests/functions/$f" ]]; then
        cp "$DEST/tests/functions/$f" "$BACKUP_DIR/functions/$f"
    fi
done

rsync -a --delete "$WG/test/tests/" "$DEST/tests/"
rsync -a --delete "$WG/test/schemas/" "$DEST/schemas/"

for f in number.json currency.json; do
    if [[ -f "$BACKUP_DIR/functions/$f" && -f "$DEST/tests/functions/$f" ]]; then
        if ! cmp -s "$BACKUP_DIR/functions/$f" "$DEST/tests/functions/$f"; then
            echo "notice: tests/functions/$f changed upstream — diff against $BACKUP_DIR/functions/$f"
            echo "        and merge any ICU4X-only cases before dropping the backup."
        fi
    fi
done

SHA="$(git -C "$WG" rev-parse HEAD 2>/dev/null || echo unknown)"
printf '%s\n' "$SHA" >"$DEST/UPSTREAM_SHA"

echo "Synced MF2 fixtures from $WG (UPSTREAM_SHA=$SHA) -> $DEST"
echo "Next: merge ICU4X-only rows in tests/functions/number.json and currency.json if needed;"
echo "      update the pin line in fixtures/README.md; then run:"
echo "  cargo test -p icu_experimental --test messageformat_conformance --all-features"
