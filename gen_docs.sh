#!/usr/bin/env bash
cargo clean
cargo doc --no-deps
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=lunar_lua\">" > target/doc/index.html
cp -r target/doc ./docs
git add ./docs/.
git commit -m "updated docs"
