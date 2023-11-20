#!/bin/sh

set -o errexit

readonly target=x86_64-pc-windows-gnu
readonly game=spaceship_game

# cargo build --target $target
cross build --target $target

cp target/$target/debug/$game.exe .

exec ./$game.exe "$@"