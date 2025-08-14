#!/bin/bash

cargo build --release && \
rm -rf build/partydeck
mkdir -p build/ build/res build/bin && \
cp target/release/partydeck build/ && \
cp LICENSE build/ && cp COPYING.md build/thirdparty.txt && \
cp res/splitscreen_kwin.js res/splitscreen_kwin_vertical.js build/res && \
cp res/sway.cfg build/res && \
cp deps/gamescope/build/src/gamescope build/bin/gamescope-kbm
