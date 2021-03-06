#!/usr/bin/env bash

## exit if something fails
set -e

declare -a publish_list=(
    "ajars_core"
    "ajars_actix_web"
    "ajars_axum"
    "ajars_reqwest"
    "ajars_surf"
    "ajars_web"
    "."
)

echo 'Attempt ''cargo check'' before publishing'
cargo check

echo 'Attempt ''cargo check --all-features'' before publishing'
cargo check --all-features

echo 'Attempt ''cargo test'' before publishing'
cargo test --workspace

echo 'Attempt ''cargo test --all-features'' before publishing'
cargo test --workspace --all-features

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
