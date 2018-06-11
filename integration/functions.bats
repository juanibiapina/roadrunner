#!/usr/bin/env bats

load test_helper

@test "functions: username" {
  cat > "$ROADRUNNER_SCRIPT" << RHAI
username()
RHAI
  run $ROADRUNNER_BIN

  assert_success
  assert_output "$USER"
}

@test "functions: hostname" {
  cat > "$ROADRUNNER_SCRIPT" << RHAI
hostname()
RHAI
  run $ROADRUNNER_BIN

  assert_success
  assert_output "$(hostname -s)"
}

@test "functions: cwd" {
  cat > "$ROADRUNNER_SCRIPT" << RHAI
cwd()
RHAI
  run $ROADRUNNER_BIN

  assert_success
  assert_output "$(dirs +0)"
}

@test "functions: cwd inside HOME directory" {
  cat > "$ROADRUNNER_SCRIPT" << RHAI
cwd()
RHAI
  cd
  run $ROADRUNNER_BIN

  assert_success
  assert_output "~"
}
