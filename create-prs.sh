#!/bin/bash

# Number of PRs to create
NUM_PRS=${1:-5}

# Base branch to create PRs against
BASE_BRANCH="master"

for i in $(seq 1 $NUM_PRS); do
    BRANCH_NAME="test-pr-$i-$(date +%s)"

    # Create a new branch
    git checkout -b "$BRANCH_NAME" "$BASE_BRANCH"

    # Create an empty commit (PRs need at least one commit difference)
    git commit --allow-empty -m "Test PR #$i"

    # Push the branch
    git push origin "$BRANCH_NAME"

    # Create the PR
    gh pr create --repo AvalancheHQ/anyhow --title "Test PR #$i" --body "This is a test PR for testing purposes." --base "$BASE_BRANCH"

    # Go back to base branch
    git checkout "$BASE_BRANCH"

    echo "Created PR #$i"
done

echo "Done! Created $NUM_PRS PRs."
