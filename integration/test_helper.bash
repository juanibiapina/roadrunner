load vendor/bats-support/load
load vendor/bats-assert/load

export ROADRUNNER_TEST_DIR="${BATS_TMPDIR}/roadrunner"
export ROADRUNNER_ORIGIN_DIR="${ROADRUNNER_TEST_DIR}/origin"
export ROADRUNNER_LOCAL_DIR="${ROADRUNNER_TEST_DIR}/local"

export ROADRUNNER_ROOT="${BATS_TEST_DIRNAME}/.."

export ROADRUNNER_BIN=$ROADRUNNER_ROOT/target/debug/roadrunner

mkdir -p $ROADRUNNER_ORIGIN_DIR
mkdir -p $ROADRUNNER_LOCAL_DIR

teardown() {
  rm -rf "$ROADRUNNER_TEST_DIR"
}

load lib/helpers
