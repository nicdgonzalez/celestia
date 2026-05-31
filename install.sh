#!/bin/bash

if [[ ! -e "/usr/bin/tmux" ]]; then
    echo "tmux is not installed. Please install tmux."
    sudo dnf install tmux
    exit 1
fi

if [[ ! -e "/usr/bin/java" ]]; then
    echo "Java is not installed. Please install Java 17."
    sudo dnf install java-17-openjdk java-17-openjdk-devel
    exit 1
fi

if [[ ! -e "/usr/bin/git" ]]; then
    echo "Git is not installed. Please install Git."
    sudo dnf install git
    exit 1
fi

FUJI_REPO="$HOME/.fuji/lib/fuji"
FUJI_EXEC="$FUJI_REPO/fuji"
FUJI_TARGET="$HOME/.local/bin/fuji"

mkdir -p "$( dirname "$FUJI_REPO" )"
mkdir -p "$( dirname "$FUJI_TARGET" )"

git clone --branch main \
    https://github.com/nicdgonzalez/fuji.git "$FUJI_REPO"

chmod u+x "$FUJI_EXEC"
ln -s "$FUJI_EXEC" "$FUJI_TARGET"
