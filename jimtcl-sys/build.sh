#!/bin/sh
(
    cd jimtcl
    ./configure --with-ext="oo tree binary sqlite3" --enable-utf8 --ipv6 --disable-docs
    git am ../1508-epic-patches.patch
    make
)
