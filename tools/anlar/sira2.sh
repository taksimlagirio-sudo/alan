#!/bin/bash
# kalan 6 maç, aynı anda 3
D=/tmp/claude-0/-home-user-daire-uygulamasi/3c53acc8-0565-5ab0-ac4b-2ecdd7a18db1/scratchpad/alan-motor-6ad507e
cd /tmp/claude-0/-home-user/3c53acc8-0565-5ab0-ac4b-2ecdd7a18db1/scratchpad/anlar
printf "%s\n" "74 1" "85 0" "96 2" "107 1" "118 0" "129 2" | xargs -P 3 -L 1 sh -c 'node tara.js '$D' $0 $1 cikti 2>> log.txt'
echo HEPSI >> log.txt
