#!/bin/sh

# https://github.com/linux4life798/bash-includes/blob/main/bash_include.d/1-msg-print.bash
mrun() {
    printf '\033[34m%s\033[m\n' "> $*" >&2
    "$@"
}

mrun cargo run
echo >&2

mrun sqlite3 db.sqlite .dump
