#!/usr/bin/env bats

load test_helper

@test "literals" {
  ROADRUNNER_PROMPT="[@]:()" run $ROADRUNNER_BIN

  assert_success
  assert_output "[@]:()"
}
