#!/usr/bin/bash

# Allow the program to exit immediately if any commands fail.
set -eo pipefail
# If the environment variable TRACE is set, enable tracing.
[ ! -z "${TRACE+x}" ] && set -x

main() {
    # Check if we are inside of a tmux session.
    if [ -z "${TMUX+x}" ]; then
        echo >&2 'error: expected script to be ran from within a tmux session'
        exit 1
    fi

    # Use '{session}:{window}.{pane}' format to reference windows or panes.
    local session_name="$(tmux list-sessions -F "${session_name}" -f "${session_attached}")"

    tmux rename-window "main"

    tmux new-window -d -n "server"
    tmux send-keys -t "$session_name:server" 'cd server' 'Enter'

    tmux new-window -d -n "client"
    tmux send-keys -t "$session_name:client" 'cd client' 'Enter'

    tmux new-window -d -n "plugin"
    tmux send-keys -t "$session_name:plugin" 'cd plugin' 'Enter'

    tmux new-window -d -n "scratch"

    $EDITOR .
}

main "$@"
