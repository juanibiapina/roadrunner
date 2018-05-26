#!/usr/bin/env bats

load test_helper

run_with_rbenv() {
  ROADRUNNER_PROMPT="{rbenv:(%version%)}" run $ROADRUNNER_BIN
}

@test "rbenv: when not in a rbenv repo" {
  create_dir "non-ruby-project"
  cd_local "non-ruby-project"

  run_with_rbenv

  assert_success
  assert_output ""
}

@test "rbenv: when in a rbenv repo" {
  create_dir "ruby-project"
  cd_local "ruby-project"
  echo 2.5.1 > .ruby-version

  run_with_rbenv

  assert_success
  assert_output "(2.5.1)"
}

@test "rbenv: when in a subdirectory of an rbenv repo" {
  create_dir "ruby-project"
  cd_local "ruby-project"
  echo 2.5.1 > .ruby-version
  mkdir "child"
  cd "child"

  run_with_rbenv

  assert_success
  assert_output "(2.5.1)"
}
