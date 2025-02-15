#!/bin/bash

# i can't actually get this to work as a script to its just here for me to remember how to do this.
# saves the output of "time" to a file, ideally in that day's folder

COMMAND=$1
OUTFILE=$2

{ time "${COMMAND}" &> /dev/tty; } 2>&1 | grep real > "$2"
