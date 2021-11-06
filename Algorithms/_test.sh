#!/usr/bin/env bash

# simple script for testing files

# Should be run in the agorithms directory. I don't have a check for that

algorithms=( "symmetric-differences" "inventory-update" )

for al in "${algorithms[@]}"
do
    echo "== Running $al programs =========="
    # for file in "."/$al*
    for file in $al.*
    do
        if [ -f $file ] 
        then
            case "${file##*.}" in 
                R )
                    echo ""
                    echo "-- Executing Rscript for $file ----------"
                    Rscript --vanilla $file
                    echo "i: no outputs from Rscript means success!"
                    ;;
                py )
                    echo ""
                    echo "-- Executing python for $file ----------"
                    python -m unittest -q $file
                    ;;
                rs )
                    echo ""
                    echo "-- Executing rustc for $file ----------"
                    rustc --verbose $file
                    # remove file extension
                    file_sans_ext="${file%.*}"
                    windows_file=${file_sans_ext//\//\\}
                    
                    echo "-- Executing $windows_file ----------"
                    cmd.exe /C "$windows_file.exe"

                    echo "-- Cleaning up Rust ----------"
                    rm $file_sans_ext.exe
                    rm $file_sans_ext.pdb
                    ;;
                exe ) 
                    rm $file
                    ;;
                pdb )
                    rm $file
                    ;;
                * )
                    echo "Ignoring $file"
                    ;;
            esac
        fi
    done
done

