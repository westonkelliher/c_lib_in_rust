#!/bin/bash

set -e

cargo build
gcc print_names.c target/debug/liblib_for_c_in_rust.a
./a.out
