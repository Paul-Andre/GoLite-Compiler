#!/bin/bash

# Usage: ./run.sh <mode> <file>
# 	mode: scan|tokens|parse|pretty|symbol|typecheck|codegen
                                
# Check the command-line arguments are valid

if [ $# -lt 2 ]
then
	echo "Missing arguments"
	echo "Usage: $0 <mode> <file>"
	echo " + mode: scan|tokens|parse|pretty|rename|obfuscate|symbol|typecheck|codegen"
	echo " + file: path to file (absolute or relative)"
	exit 1
fi

if [[ "|scan|tokens|parse|pretty|rename|obfuscate|symbol|typecheck|codegen|runjs|interpret|" != *"|$1|"* ]]
then
	echo "Unknown mode \"$1\""
	echo "Usage: $0 <mode> <file>"
	echo " + mode: scan|tokens|parse|pretty|symbol|typecheck|codegen"
	echo " + file: path to file (absolute or relative)"
	exit 1

fi

# Invoke the compiler with the provided arguments: mode ($1) and file ($2)
#
# You MUST replace the following command with the command for invoking your compiler

if [[ "$1" == codegen ]]
then
    ./target/debug/golite "$1" < "$2" > "${2%.*}.js" && echo OK
  elif [[ "$1" == runjs ]]
  then
    ./target/debug/golite codegen < "$2" > "${2%.*}.js" && node "${2%.*}.js"
  else
    ./target/debug/golite "$1" < "$2"
fi
#cargo run --quiet "$1" < "$2"
#~cs520/golitec "$1" < "$2"
