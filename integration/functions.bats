#!/usr/bin/env bats

load test_helper

@test "functions: username" {
  ROADRUNNER_PROMPT="#{username()}" run $ROADRUNNER_BIN

  assert_success
  assert_output "$USER"
}

@test "functions: hostname" {
  ROADRUNNER_PROMPT="#{hostname()}" run $ROADRUNNER_BIN

  assert_success
  assert_output "$(hostname -s)"
}

@test "functions: cwd" {
  ROADRUNNER_PROMPT="#{cwd()}" run $ROADRUNNER_BIN

  assert_success
  assert_output "$(dirs +0)"
}

@test "functions: cwd inside HOME directory" {
  cd
  ROADRUNNER_PROMPT="#{cwd()}" run $ROADRUNNER_BIN

  assert_success
  assert_output "~"
}
