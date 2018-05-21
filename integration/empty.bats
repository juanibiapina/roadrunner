#!/usr/bin/env bats

load test_helper

@test "Runs successfully" {
  ROADRUNNER_CONFIG= run $ROADRUNNER
  assert_success
  assert_output ""
}
