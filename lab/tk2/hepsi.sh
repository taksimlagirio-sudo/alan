#!/bin/bash
O=/home/user/alan-arsiv/lab/tk2
xargs -P4 -d '\n' -I{} $O/kos.sh '{}' < $O/isler.txt
echo BITTI > $O/BITTI
