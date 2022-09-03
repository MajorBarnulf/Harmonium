#!/bin/sh

nodemon -e "rs,toml,json,html,css,js" -x "cargo tauri dev" -w "src-tauri" -w "src" --ignore "src-tauri/target"