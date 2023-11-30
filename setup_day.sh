#!/bin/bash

set -euo pipefail

if [ "$#" -ne 1 ]; then
    echo "Usage: $(basename "$0") <day-number>" >&2
    exit 1
fi

if [ ! -d .git ]; then
    echo "Must be run from the root of the advent-of-code repository" >&2
    exit 1
fi

DAY_NUM=$1
NAME=$(printf "day_%02d" "$DAY_NUM")

if [ -d "$NAME" ]; then
    echo "The project '$NAME' already exists." >&2
else
    cargo new --vcs none "$NAME"
fi

SESSION_COOKIE_FILE="./sessionCookie"
if [ ! -f "$SESSION_COOKIE_FILE" ]; then
    echo "Session cookie file not found" >&2
    exit 1
fi

INPUT_URL="http://adventofcode.com/2023/day/${DAY_NUM}/input"
curl -Ls -b session=$(cat "$SESSION_COOKIE_FILE") "$INPUT_URL" > "${NAME}/input.txt"

echo
echo "Project '${NAME}' created successfully with input file."
echo "--------HEAD-INPUT----------"
head "${NAME}/input.txt"
echo "----------------------------"
