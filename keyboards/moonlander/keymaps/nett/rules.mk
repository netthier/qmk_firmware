# Set any rules.mk overrides for your specific keymap here.
# See rules at https://docs.qmk.fm/#/config_options?id=the-rulesmk-file
TAP_DANCE_ENABLE = no
RAW_ENABLE = yes

RUST_CRATE = qmk_ext
RUST_TARGET = thumbv7em-none-eabihf
RUST_TOOLCHAIN = nightly # for alloc crate support
RUST_QMK_FEATURES = malloc
RUST_QMK_HEADERS = raw_hid.h version.h

# required for malloc
EXTRALDFLAGS += -specs=nosys.specs
