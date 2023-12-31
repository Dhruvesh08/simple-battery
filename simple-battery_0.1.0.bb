# Auto-Generated by cargo-bitbake 0.3.16
#
inherit cargo

# If this is git based prefer versioned ones if they exist
# DEFAULT_PREFERENCE = "-1"

# how to get simple-battery could be as easy as but default to a git checkout:
# SRC_URI += "crate://crates.io/simple-battery/0.1.0"
SRC_URI += "git://github.com/Dhruvesh08/simple-battery.git;protocol=https;nobranch=1;branch=main"
SRCREV = "8995dd18262d28d366253d1a35defe0110344b80"
S = "${WORKDIR}/git"
CARGO_SRC_DIR = ""
PV:append = ".AUTOINC+8995dd1826"

# please note if you have entries that do not begin with crate://
# you must change them to how that package can be fetched
SRC_URI += " \
    crate://crates.io/autocfg/1.1.0 \
    crate://crates.io/battery/0.7.8 \
    crate://crates.io/bitflags/1.3.2 \
    crate://crates.io/cc/1.0.83 \
    crate://crates.io/cfg-if/1.0.0 \
    crate://crates.io/core-foundation-sys/0.7.0 \
    crate://crates.io/core-foundation/0.7.0 \
    crate://crates.io/lazycell/1.3.0 \
    crate://crates.io/libc/0.2.148 \
    crate://crates.io/mach/0.3.2 \
    crate://crates.io/nix/0.19.1 \
    crate://crates.io/num-traits/0.2.16 \
    crate://crates.io/typenum/1.17.0 \
    crate://crates.io/uom/0.30.0 \
    crate://crates.io/winapi-i686-pc-windows-gnu/0.4.0 \
    crate://crates.io/winapi-x86_64-pc-windows-gnu/0.4.0 \
    crate://crates.io/winapi/0.3.9 \
"



# FIXME: update generateme with the real MD5 of the license file
LIC_FILES_CHKSUM = " \
    "

SUMMARY = "simple-battery"
HOMEPAGE = "https://github.com/Dhruvesh08/simple-battery"
LICENSE = "CLOSED"

# includes this file if it exists but does not fail
# this is useful for anything you may want to override from
# what cargo-bitbake generates.
include simple-battery-${PV}.inc
include simple-battery.inc
