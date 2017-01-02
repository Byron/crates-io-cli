#!/bin/bash

exe=${1:?First argument is the executable to test}
existing_crates_clone=${2:?Second argument is a pre-existing clone of crates.io}


if $exe >/dev/null; then
  echo "Invocation without any subcommand is a failure, but prints usage"
  exit 1
fi

with_repo="--repository $existing_crates_clone"
if ! $exe $with_repo recent-changes >/dev/null; then
  echo "Expecting recent-changes on existing repo to work"
  exit 2
fi

if ! $exe $with_repo recent-changes --output=json >/dev/null; then
  echo "You can change the output to json"
  exit 3
fi

if ! $exe recent-changes >/dev/null; then
  echo "It should be able to clone a non-existing crates repository"
  exit 4
fi

if [ `$exe $with_repo list by-user byron | wc -l` -lt 200 ] ; then
  echo "It can list repositories of a known user with paging"
  exit 5
fi

echo "OK"
