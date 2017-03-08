#!/bin/sh

# masked_permute — Fast lexicographically next permutation *within* a bitmask
# Copyright (C) 2017  Ben Wiederhake
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.

set -ev

mkdir -p .cargo
CONFIG_FILE=.cargo/config

echo "[build]" > ${CONFIG_FILE}

case $MP_TARGET_CONFIG in
    default)
      # Actually, now that you mention it …
      rm ${CONFIG_FILE}
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
