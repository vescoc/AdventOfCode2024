#!/bin/bash

for d in day[0-2][0-9]; do (echo ">>> $d"; cd $d/rsui; time trunk build --release --filehash false --public-url /AdventOfCode2023/$d/); done
