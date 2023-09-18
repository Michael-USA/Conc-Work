#!/bin/bash

# Check if a file is provided as an argument
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 filename.sh"
    exit 1
fi

# Open the file in Vim with the desired settings
vim "$1" -c "set number" \
         -c "syntax enable" \
         -c "set smartindent" \
         -c "set showmatch" \
         -c "set tabstop=4" \
         -c "set expandtab" \
         -c "set ruler" \
         -c "set incsearch" \
         -c "set ignorecase"
