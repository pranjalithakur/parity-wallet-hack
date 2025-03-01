#!/bin/sh
# Running Parity Full Test Sute

FEATURES="json-tests"
OPTIONS="--release"

case $1 in
    --no-json)
    FEATURES="ipc"
    shift # past argument=value
    ;;
	--no-release)
	OPTIONS=""
	shift
	;;
    *)
            # unknown option
    ;;
esac

. ./scripts/targets.sh
cargo test -j 8 $OPTIONS --features "$FEATURES" $TARGETS $1 \

