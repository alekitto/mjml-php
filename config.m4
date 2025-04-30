PHP_ARG_ENABLE([cargo_debug], [whether to enable cargo debug mode],
[  --enable-cargo-debug           Enable cargo debug], no, no)

AC_PATH_PROG(CARGO, cargo, no)
if ! test -x "$CARGO"; then
AC_MSG_ERROR([cargo command missing, please reinstall the cargo distribution])
fi

AC_PATH_PROG(PHP_CONFIG, php-config, no)
if ! test -x "$PHP_CONFIG"; then
AC_MSG_ERROR([php-config command missing])
fi

CARGO_MODE_FLAGS="--release"
CARGO_MODE_DIR="release"
CARGO_FEATURES_FLAGS=""

if test "$PHP_CARGO_DEBUG" != "no"; then
  CARGO_MODE_FLAGS=""
  CARGO_MODE_DIR="debug"
fi

cat >>Makefile.objects<< EOF
all: cargo_build

clean: cargo_clean

cargo_build:
	$CARGO build $CARGO_MODE_FLAGS $CARGO_FEATURES_FLAGS
	if [[ -f ./target/$CARGO_MODE_DIR/libmjml.dylib ]] ; then \\
		cp ./target/$CARGO_MODE_DIR/libmjml.dylib ./modules/mjml.dylib ; fi
	if [[ -f ./target/$CARGO_MODE_DIR/libmjml.so ]] ; then \\
		cp ./target/$CARGO_MODE_DIR/libmjml.so ./modules/mjml.so ; fi

cargo_clean:
	cargo clean

.PHONY: cargo_build cargo_clean
EOF

AC_CONFIG_LINKS([ \
  Cargo.lock:Cargo.lock \
  Cargo.toml:Cargo.toml \
  src:src \
])

PHP_NEW_EXTENSION(mjml)
