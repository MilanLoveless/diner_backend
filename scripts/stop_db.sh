#!/usr/bin/env bash
PG_DOCKER="$(docker ps | grep postgres)"
if [ -n "$PG_DOCKER" ]; then
    echo $PG_DOCKER
    PG_D_ARR=($PG_DOCKER)
    PG_CONTAINER="${PG_D_ARR[${#PG_D_ARR[@]} - 1]}"
    docker stop $PG_CONTAINER
fi
