#!/usr/bin/env bats

load test_helper

@test "arguments: variable name" {
  ROADRUNNER_PROMPT=a NEW_PROMPT=b run $ROADRUNNER_BIN -v NEW_PROMPT
  assert_success
  assert_output "b"
}
