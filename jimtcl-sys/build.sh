#!/bin/sh
(
    cd jimtcl
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
