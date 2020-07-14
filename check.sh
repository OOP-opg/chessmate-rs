#!/usr/bin/env sh

cargo clippy --release -- -W clippy::pedantic
echo "==TODO=="
grep --color=auto -r "TODO" src/common src/tic_tac_toe
echo "==FIXME==="
grep --color=auto -r "FIXME" src/common src/tic_tac_toe
