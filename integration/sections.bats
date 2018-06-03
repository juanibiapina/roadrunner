#!/usr/bin/env bats

load test_helper

@test "sections: with only literals inside" {
  ROADRUNNER_PROMPT="{literals}" run $ROADRUNNER_BIN

  assert_success
  assert_output ""
}

@test "sections: with empty placeholder" {
  ROADRUNNER_PROMPT="{%nothing% doesn't render}" run $ROADRUNNER_BIN

  assert_success
  assert_output ""
}

@test "sections: with empty and non-empty placeholder" {
  ROADRUNNER_PROMPT="{%nothing%%username%yes}" run $ROADRUNNER_BIN

  assert_success
  assert_output "${USER}yes"
}
