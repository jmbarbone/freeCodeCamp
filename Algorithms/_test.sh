#!/usr/bin/env bash

# simple script for testing files

echo "symmetric-differences"
echo "> R"
rscript --vanilla symmetric-differences.R
echo "> python"
python -m unittest -q symmetric-differences.py
echo "> Rust"
rustc symmetric-differences.rs
cmd.exe /C "symmetric-differences"

echo ".. clean up"
rm symmetric-differences.exe
rm symmetric-differences.pdb
