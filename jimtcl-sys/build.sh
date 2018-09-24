#!/bin/sh
set -e
(
    cd jimtcl
    echo "PWD: $(pwd)"
    ls
    git config --global user.email "you@example.com"
    git config --global user.name "Your Name"   
    echo "JIM TIME"
    ./configure --with-ext="oo tree binary sqlite3" --enable-utf8 --ipv6 --disable-docs
    echo "CONFGIURE DONE"
    ls
    git am ../1508-epic-patches.patch
    echo "PATCH APPLIED:"
    head jim-subcmd.h
    make
    echo "make supposedly finished"
)
