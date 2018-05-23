#!/usr/bin/env bats

load test_helper

@test "placeholders: username" {
  ROADRUNNER_PROMPT="%username%" run $ROADRUNNER_BIN

  assert_success
  assert_output "$USER"
}

@test "placeholders: hostname" {
  ROADRUNNER_PROMPT="%hostname%" run $ROADRUNNER_BIN

  assert_success
  assert_output "$(hostname -s)"
}
