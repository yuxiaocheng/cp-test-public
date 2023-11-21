#!/usr/bin/env bash
CURDIR=$(cd $(dirname $0); pwd)

if [ "X$1" != "X" ]; then
    RUNTIME_ROOT=$1
else
    RUNTIME_ROOT=${CURDIR}
fi

if [ "${IS_TCE_DOCKER_ENV}" == 1 ] && [ -n "${RUNTIME_LOGDIR}" ]; then
	RUNTIME_LOG_ROOT=$RUNTIME_LOGDIR
else
	RUNTIME_LOG_ROOT=$RUNTIME_ROOT/log
fi

if [ ! -d $RUNTIME_LOG_ROOT/app ]; then
    mkdir -p $RUNTIME_LOG_ROOT/app
fi

if [ ! -d $RUNTIME_LOG_ROOT/rpc ]; then
    mkdir -p $RUNTIME_LOG_ROOT/rpc
fi

export RUST_LOG_DIR=$RUNTIME_LOG_ROOT

exec "$CURDIR/bin/server"
