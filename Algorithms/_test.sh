#!/usr/bin/env bash

# simple script for testing files
# Need to loop this out

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

echo "inventory-update"
echo "> R"
rscript --vanilla inventory-update.R
echo "> python"
python -m unittest -q inventory-update.py


# echo "> Rust"
# rustc inventory-update.rs
# cmd.exe /C "inventory-update"
#
# echo ".. clean up"
# rm inventory-update.exe
# rm inventory-update.pdb
