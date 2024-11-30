#!/bin/sh

for d in 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25;
do
    mkdir day$d
    (
        cd day$d &&
            cp -R ../day01/* . &&
            sed -i s/day01/day$d/g rs/Cargo.toml rs/benches/bench.rs rs/src/main.rs
    )
done
