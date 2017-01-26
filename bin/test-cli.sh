#!/bin/bash

exe=${1:?First argument is the executable to test}
existing_crates_clone=${2:?Second argument is a pre-existing clone of crates.io}


if $exe >/dev/null; then
  echo "Invocation without any subcommand is a failure, but prints usage"
  exit 1
fi

with_repo="--repository $existing_crates_clone"
if ! $exe recent-changes $with_repo >/dev/null; then
  echo "Expecting recent-changes on existing repo to work"
  exit 2
fi

if ! $exe recent-changes $with_repo --output=json >/dev/null; then
  echo "You can change the output to json"
  exit 3
fi

if ! $exe recent-changes >/dev/null; then
  echo "It should be able to clone a non-existing crates repository"
  exit 4
fi

if [ `$exe list by-user 980 | wc -l` -lt 200 ] ; then
  echo "It can list repositories of a known user with paging"
  exit 5
fi

if ! $exe list -o json by-user 980  >/dev/null ; then
  echo "You can change the list output to json"
  exit 6
fi


echo "OK"
