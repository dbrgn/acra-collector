#!/bin/bash

# Print the last logcat.
# Dependencies: jq

cat crashes.txt | tail -n 1 | jq .LOGCAT | sed 's/\\t/\t/g' | sed 's/\\n/\n/g'
