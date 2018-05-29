create_git_origin() {
  local name="$1"
  mkdir -p "${ROADRUNNER_ORIGIN_DIR}/$name"
  cd "${ROADRUNNER_ORIGIN_DIR}/$name"
  git init .
  touch README
  git add .
  git commit -m "README"
  touch FILE
  git add .
  git commit -m "FILE"
  cd -
}

create_dir() {
  local name="$1"
  mkdir -p "${ROADRUNNER_LOCAL_DIR}/$name"
}

clone_origin() {
  local name="$1"
  cd "${ROADRUNNER_LOCAL_DIR}"
  git clone "${ROADRUNNER_ORIGIN_DIR}/$name"
  cd -
}

cd_origin() {
  local name="$1"
  cd "${ROADRUNNER_ORIGIN_DIR}/$name"
}

cd_local() {
  local name="$1"
  cd "${ROADRUNNER_LOCAL_DIR}/$name"
}
