@echo off

copy resources\config.toml target\debug\ > nul 2>&1
copy resources\contents.txt target\debug\contents.txt > nul 2>&1

cargo run
