#!/bin/sh

set -e

PATH=$PATH:$HOME/.cargo/bin
echo "$PATH"

cargo lipo
