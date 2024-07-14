@echo off

cargo build --release

copy resources\config.toml target\release\ > nul 2>&1
copy resources\contents.txt target\release\ > nul 2>&1
