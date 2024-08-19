#! /bin/bash

eval $(opam env --switch=4.14.1)
why3 replay -L /root/eval/baselines/creusot/prelude $1
