#!/usr/bin/env bash

# simple script for testing files

# Should be run in the agorithms directory. I don't have a check for that

algorithms=( "symmetric-differences" "inventory-update" "no-repeats-please" "pairwise")

for al in "${algorithms[@]}"
do
    echo ""
    n=$(echo -n "$al" | wc -m)
    n=$((30 - n))
    printf "== Running $al programs "
    printf '=%.0s' $(seq 1 $n)
    echo ""
    echo ""
    # for file in "."/$al*
    for file in $al.*
    do
        if [ -f $file ] 
        then
        echo "$file"
            case "${file##*.}" in 
                R )
                    Rscript --vanilla $file
                    ;;
                py )
                    python -m unittest -q $file 2> .pylog
                    if grep -q FAIL ".pylog"
                    then
                        echo ""
                        head -n 50 .pylog
                        echo ""
                    fi
                    rm ".pylog"
                    ;;
                rs )
                    cargo script $file
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

