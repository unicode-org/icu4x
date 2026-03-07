#!/usr/bin/env python3
import argparse
import subprocess
import json
import re
import sys

def run_command(command):
    """Runs a shell command and returns its output."""
    try:
        result = subprocess.run(command, capture_output=True, text=True, check=True)
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        # Silencing errors for search as it's common to find nothing
        return None

def get_commits(revset):
    """Gets a list of commit hashes in the given revset, in chronological order."""
    output = run_command(["git", "rev-list", "--reverse", revset])
    if output:
        return output.splitlines()
    return []

def get_pr_info(commit_hash):
    """Fetches PR information for a given commit hash using 'gh'."""
    # Search for PRs associated with this commit
    output = run_command(["gh", "pr", "list", "--search", commit_hash, "--state", "merged", "--json", "title,body,url,number"])
    if output:
        prs = json.loads(output)
        if prs:
            return prs[0]

    # Fallback to looking for PR number in commit message (e.g. "Title (#123)")
    msg = run_command(["git", "log", "-1", "--format=%s", commit_hash])
    if msg:
        match = re.search(r"\(#(\d+)\)$", msg)
        if match:
            pr_number = match.group(1)
            output = run_command(["gh", "pr", "view", pr_number, "--json", "title,body,url,number"])
            if output:
                return json.loads(output)
    return None

def extract_changelog(body):
    """Extracts the '## Changelog' section from the PR body."""
    # Matches a header like '## Changelog' or '## Changelog (N/A)'
    # Captures until the next header starting with '#' or the end of the body.
    pattern = re.compile(r"(?im)^#+\s*Changelog.*(?:\r?\n)+(.*?)(?=\r?\n#+\s+|$)", re.DOTALL)
    match = pattern.search(body)
    if match:
        content = match.group(1).strip()
        if content:
            return content
    return "NO CHANGELOG FOUND"

def main():
    parser = argparse.ArgumentParser(description="Collate changelog entries from GitHub PRs.")
    parser.add_argument("revset", help="The git revset to scan (e.g., 'v1.0..v1.1' or 'HEAD~5..HEAD')")
    args = parser.parse_args()

    commits = get_commits(args.revset)
    if not commits:
        print(f"No commits found in revset: {args.revset}")
        return

    processed_prs = set()

    for commit_hash in commits:
        pr_info = get_pr_info(commit_hash)

        if pr_info:
            pr_number = pr_info['number']
            if pr_number in processed_prs:
                continue
            processed_prs.add(pr_number)

            title = pr_info['title']
            url = pr_info['url']
            body = pr_info['body'] or ""
            changelog = extract_changelog(body)

            print(f"## {title}")
            print()
            print(f"{url}")
            print()
            print(changelog)
            print()
        else:
            # Fallback for commits without an associated PR
            msg = run_command(["git", "log", "-1", "--format=%s", commit_hash])
            print(f"## {msg}")
            print()
            print(f"Commit: {commit_hash[:10]}")
            print()
            print("NO PR FOUND / NO CHANGELOG FOUND")
            print()

if __name__ == "__main__":
    main()
