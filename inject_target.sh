#!/bin/sh

set -ev

mkdir -p .cargo
CONFIG_FILE=.cargo/config

echo "CONFIG_FILE" > ${CONFIG_FILE}

case $MP_TARGET_CONFIG in
    default)
      echo '# Nothing to inject' >> ${CONFIG_FILE}
      ;;
    popcnt)
      echo 'rustflags = ["-C", "target-feature=+popcnt"]' >> ${CONFIG_FILE}
      ;;
    native)
      echo 'rustflags = ["-C", "target-cpu=native"]' >> ${CONFIG_FILE}
      ;;
    *)
      exit 1;
      ;;
esac
