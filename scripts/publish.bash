#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.."

# -----------------------------------------------------------------------------
# Helpers
# -----------------------------------------------------------------------------

die() { echo "error: $*" >&2; exit 1; }

confirm() {
    local response
    read -r -p "$1 [y/N] " response
    [[ "$response" =~ ^[Yy]$ ]]
}

# Portable in-place sed (macOS requires an explicit empty backup suffix)
sedi() {
    if [[ "$(uname)" == "Darwin" ]]; then
        sed -i '' "$@"
    else
        sed -i "$@"
    fi
}

wait_for_index() {
    local secs=30
    echo "Waiting ${secs}s for crates.io to index..."
    for i in $(seq "$secs" -1 1); do
        printf "\r  %2ds remaining..." "$i"
        sleep 1
    done
    printf "\r%-40s\n" "  Done."
}

# -----------------------------------------------------------------------------
# Version argument
# -----------------------------------------------------------------------------

[[ $# -eq 1 ]] || die "usage: $0 major|minor|patch|X.Y.Z"

current=$(grep '^version = ' Cargo.toml | head -1 | sed 's/version = "\(.*\)"/\1/')
IFS='.' read -r maj min pat <<< "$current"

case "$1" in
    major) new="$((maj + 1)).0.0" ;;
    minor) new="${maj}.$((min + 1)).0" ;;
    patch) new="${maj}.${min}.$((pat + 1))" ;;
    [0-9]*.[0-9]*.[0-9]*) new="$1" ;;
    *) die "argument must be major, minor, patch, or a version like 1.2.3" ;;
esac

echo "Bumping version: ${current} → ${new}"

# Require a clean working tree so the version-bump commit is unambiguous.
if ! git diff --quiet || ! git diff --cached --quiet; then
    die "working tree is not clean; commit or stash changes before publishing"
fi

# -----------------------------------------------------------------------------
# Update Cargo.toml files
# -----------------------------------------------------------------------------

TOMLS=(
    Cargo.toml
    identifiers-derive/Cargo.toml
    identifiers-uuid/Cargo.toml
    identifiers-uri/Cargo.toml
)

for toml in "${TOMLS[@]}"; do
    sedi "s/\"${current}\"/\"${new}\"/g" "$toml"
done

# -----------------------------------------------------------------------------
# Verify
# -----------------------------------------------------------------------------

cargo check --all
cargo test --all

# -----------------------------------------------------------------------------
# Commit and tag
# -----------------------------------------------------------------------------

git add "${TOMLS[@]}"
# Include Cargo.lock if it was modified by the version bump.
if ! git diff --quiet Cargo.lock 2>/dev/null; then
    git add Cargo.lock
fi

git commit -m "🔖 Release v${new}"
git tag "v${new}"

echo
confirm "Push commits and tag v${new} to origin?" || die "Aborted before push."
git push
git push --tags

# -----------------------------------------------------------------------------
# Publish
# -----------------------------------------------------------------------------

publish_crate() {
    local pkg="$1"
    echo
    echo "=== ${pkg} ==="
    echo "--- package contents ---"
    cargo package -p "$pkg" --list
    echo
    echo "--- dry run ---"
    cargo publish -p "$pkg" --dry-run 2>&1
    echo
    confirm "Publish ${pkg} to crates.io?" || { echo "Skipping ${pkg}."; return 1; }
    cargo publish -p "$pkg"
}

publish_crate identifiers-derive
wait_for_index
publish_crate identifiers
wait_for_index
# identifiers-uuid and identifiers-uri only depend on identifiers-derive,
# so no wait is needed between them.
publish_crate identifiers-uuid
publish_crate identifiers-uri

echo
echo "✓ Published v${new}."
