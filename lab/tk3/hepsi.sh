#!/bin/bash
O=/home/user/alan-arsiv/lab/tk3
xargs -P4 -d '\n' -I{} $O/kos.sh '{}' < $O/isler.txt
echo BITTI > $O/BITTI
