#!/bin/sh

set -o errexit

readonly target=x86_64-pc-windows-gnu
readonly game=spaceship_game

echo "Building..."

# cargo build --target $target
cross build --target $target

echo "Copying..."

cp target/$target/debug/$game.exe .

echo "Running..."

exec ./$game.exe "$@"