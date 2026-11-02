#!/bin/sh
set -eu
. /tmp/android-env.sh
export CLASSPATH=/tmp/hello.jar
exec /system/bin/app_process32 /system/bin --nice-name=tv.hello HelloScreen "${1:-120}"
