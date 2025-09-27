#!/usr/bin/fish

rsync -avr --exclude .git --exclude .idea --exclude target ../lock_survey/ clear:~/repo; or echo "failed: $host"
rsync -avr --exclude .git --exclude .idea --exclude target ../lock_survey/ plastic:~/repo; or echo "failed: $host"
rsync -avr --exclude .git --exclude .idea --exclude target ../lock_survey/ silver:~/repo; or echo "failed: $host"
rsync -avr --exclude .git --exclude .idea --exclude target ../lock_survey/ black:~/repo; or echo "failed: $host"
rsync -avr --exclude .git --exclude .idea --exclude target ../lock_survey/ blue:~/repo; or echo "failed: $host"

