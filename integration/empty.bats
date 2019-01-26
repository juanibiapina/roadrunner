#!/usr/bin/env bats

load test_helper

@test "empty: prints nothing" {
  ROADRUNNER_PROMPT= run $ROADRUNNER_BIN
  assert_success
  assert_output ""
}
