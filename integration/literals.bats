#!/usr/bin/env bats

load test_helper

@test "literals: alphanumeric" {
  ROADRUNNER_PROMPT="Some string" run $ROADRUNNER_BIN

  assert_success
  assert_output "Some string"
}

@test "literals: symbols" {
  ROADRUNNER_PROMPT="[@]:()" run $ROADRUNNER_BIN

  assert_success
  assert_output "[@]:()"
}

@test "literals: newline" {
  ROADRUNNER_PROMPT="
:) " run $ROADRUNNER_BIN

  assert_success
  assert_output "
:) "
}
