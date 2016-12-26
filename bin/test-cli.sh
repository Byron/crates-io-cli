#!/bin/bash

exe=${1:?First argument is the executable to test}
existing_crates_clone=${2:?Second argument is a pre-existing clone of crates.io}

if $exe >/dev/null; then
  echo "Invocation without any subcommand is a failure, but prints usage"
  exit 1
fi

if ! $exe --repository $existing_crates_clone recent-changes >/dev/null; then
  echo "Expecting recent-changes on existing repo to work"
fi
