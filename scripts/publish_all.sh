#!/usr/bin/env bash

## exit if something fails
set -e

./test_all.sh

declare -a publish_list=(
    "ajars_core"
    "ajars_server_actix_web"
    "ajars_server_axum"
    "ajars_client_reqwest"
    "ajars_client_surf"
    "ajars_client_web"
    "."
)

for i in "${publish_list[@]}"
do
    LINE_SEPARATOR='--------------------------------------------------------'

    cd $i
    echo $LINE_SEPARATOR
    echo 'Run Cargo publish for [' $i ']'
    echo $LINE_SEPARATOR

    cargo publish
    sleep 20
    cd ..
    rc=$?
    if [[ $rc -ne 0 ]] ; then
        echo "Failure publishing $i";
    fi

done
