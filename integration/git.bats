#!/usr/bin/env bats

load test_helper

run_with_git_config() {
  ROADRUNNER_PROMPT="{git:(%branch% %index%%wt%%untracked%%clean%)}" run $ROADRUNNER_BIN
}

@test "git: when not in a git repo" {
  create_dir "non-git"
  cd_local "non-git"

  run_with_git_config

  assert_success
  assert_output ""
}

@test "git: when in a git repo" {
  create_git_origin "repo"
  clone_origin "repo"
  cd_local "repo"

  run_with_git_config
  assert_success
  assert_output "(master ✓)"

  echo "line" >> README
  run_with_git_config
  assert_success
  assert_output "(master +1)"

  echo "other" >> FILE
  run_with_git_config
  assert_success
  assert_output "(master +2)"

  git add README
  run_with_git_config
  assert_success
  assert_output "(master ●1+1)"

  touch ANOTHER
  run_with_git_config
  assert_success
  assert_output "(master ●1+1…)"
}

@test "git: when in a subdirectory of a git repo" {
  create_git_origin "repo"
  clone_origin "repo"
  cd_local "repo"
  mkdir -p "subdir"
  cd "subdir"

  run_with_git_config

  assert_success
  assert_output "(master ✓)"
}
