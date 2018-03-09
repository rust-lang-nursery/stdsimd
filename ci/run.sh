#!/bin/sh

#set -ex

# Tests are all super fast anyway, and they fault often enough on travis that
# having only one thread increases debuggability to be worth it.
export RUST_TEST_THREADS=1
#export RUST_BACKTRACE=1
export RUST_TEST_NOCAPTURE=1

# FIXME(rust-lang-nursery/stdsimd#120) run-time feature detection for ARM Neon
case ${TARGET} in
    aarch*)
        export RUSTFLAGS="${RUSTFLAGS} -C target-feature=+neon"
        ;;
    mips64*)
        export RUSTFLAGS="${RUSTFLAGS} -C target-cpu=mips64r6"
        ;;
    *)
        ;;
esac

FEATURES="strict,$FEATURES"

echo "RUSTFLAGS=${RUSTFLAGS}"
echo "FEATURES=${FEATURES}"
echo "OBJDUMP=${OBJDUMP}"

cargo_test() {
    cmd="cargo test --target=$TARGET --features $FEATURES $1"
    cmd="$cmd -p coresimd -p stdsimd --manifest-path crates/stdsimd/Cargo.toml"
    cmd="$cmd -- $2"
    $cmd
    more /checkout/target/mips64-unknown-linux-gnuabi64/debug/build/backtrace-sys-3428ad8e56e1b724/out/config.log
}

cargo_test
cargo_test "--release"
