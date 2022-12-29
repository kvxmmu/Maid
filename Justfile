binary := "maidvm"
arch := arch()
os := os()
ext := if os_family() == "windows" { ".exe" } else { "" }

objdump := "aarch64-unknown-linux-gnu-objdump"

alias br := build-release
alias bm := build-misc
alias dis := disassemble
alias r := run
alias rr := run-release

run-release:
    cargo run --release

run:
    cargo run

disassemble binfile:
    {{objdump}} -b binary -D -m aarch64 misc/bins/{{binfile}}.bintest

# Build test binaries for the emulator
build-misc:
    misc/barecc.sh misc/bins/add2.c

# Build release binary of an emulator
build-release:
    cargo build --release
    mkdir -p release
    mv target/release/{{binary}} release/{{binary}}.{{arch}}.{{os}}
