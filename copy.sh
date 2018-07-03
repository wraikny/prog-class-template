#!/bin/sh

if [ "$#" -lt 1 ]; then
    echo 'set argument'
else
    mkdir ../$1
    cd ../$1
    cp ../template/Makefile .

    mkdir output
    
    mkdir source
    cp ../template/source/example.c ./source/.

    mkdir saty
    cp ../template/saty/report.saty ./saty/.
fi
