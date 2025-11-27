#!/bin/bash

# Script to update all story files with completion status

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
STORIES_DIR="$SCRIPT_DIR/../docs/stories"

echo "Updating story completion status..."

for story_file in "$STORIES_DIR"/*.story.md; do
    if [ -f "$story_file" ]; then
        # Check if already marked as completed
        if ! grep -q "Status.*Completed" "$story_file"; then
            # Update status to Completed
            sed -i 's/^## Status$/\n## Status\nCompleted/' "$story_file"
            echo "Updated: $(basename "$story_file")"
        fi
    fi
done

echo "Done!"

