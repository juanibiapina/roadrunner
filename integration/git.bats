#!/usr/bin/env bats

load test_helper

run_with_git_config() {
  cat > "$ROADRUNNER_SCRIPT" << RHAI
result = ""
git = git_init()
if git:enabled() then
  result = result .. "("
  result = result .. git:head()
  result = result .. " "
  if git:behind() > 0 then
    result = result .. "↓" .. git:behind()
  end
  if git:ahead() > 0 then
    result = result .. "↑" .. git:ahead()
  end
  if git:index() > 0 then
    result = result .. "●" .. git:index()
  end
  if git:wt() > 0 then
    result = result .. "+" .. git:wt()
  end
  if git:untracked() > 0 then
    result = result .. "…"
  end
  if git:index() == 0 and git:wt() == 0 and git:untracked() == 0 then
    result = result .. "✓"
  end
  result = result .. ")"
end

return result
RHAI
  run $ROADRUNNER_BIN
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
  cd_origin "repo"
  touch EXTRA
  git add EXTRA
  git commit -m EXTRA
  touch EXTRA2
  git add EXTRA2
  git commit -m EXTRA2

  cd_local "repo"
  echo line > EXTRA
  git add EXTRA
  git commit -m EXTRA
  git fetch

  run_with_git_config
  assert_success
  assert_output "(master ↓2↑1✓)"

  echo "line" >> README
  run_with_git_config
  assert_success
  assert_output "(master ↓2↑1+1)"

  echo "other" >> FILE
  run_with_git_config
  assert_success
  assert_output "(master ↓2↑1+2)"

  git add README
  run_with_git_config
  assert_success
  assert_output "(master ↓2↑1●1+1)"

  touch ANOTHER
  run_with_git_config
  assert_success
  assert_output "(master ↓2↑1●1+1…)"
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

@test "git: when in a git repo without any branches" {
  create_dir "non-git"
  cd_local "non-git"
  git init .

  run_with_git_config

  assert_success
  assert_output "(master ✓)"
}

@test "git: when in a git repo with a detached head" {
  create_git_origin "repo"
  clone_origin "repo"
  cd_local "repo"
  git checkout HEAD~1

  run_with_git_config

  assert_success
  assert_output "((detached) ✓)"
}
