set shell := ["powershell.exe", "-c"]

alias r := run

run:
    cargo run --features bevy/dynamic_linking
