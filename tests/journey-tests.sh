#!/bin/bash

set -eu -o pipefail
exe=${1:?First argument it the executable under test}
repository=${2:?Second argument is a pre-existing clone of crates.io}

root="$(cd "${0%/*}" && pwd)"
exe="$(cd "${exe%/*}" && pwd)/${exe##*/}"

# shellcheck disable=1090
 source "$root/utilities.sh"

SUCCESSFULLY=0
WITH_FAILURE=1

fixture="$root/fixtures"
snapshot="$fixture/snapshots"

title "list"

(when "listing by user"
  (when "the user exists and has many repositories"
    user_id=980
    it "can list all entries thanks to paging" && {
      expect_run_sh $SUCCESSFULLY "test \$($exe list by-user $user_id | wc -l) -gt 200"
    }
  )
  (when "the user does not exist"
    user_id=0
    it "does not fail but lists nothing" && {
      WITH_SNAPSHOT="$snapshot/list-by-non-existing-user" \
      expect_run $SUCCESSFULLY $exe list by-user $user_id
    }
  )
)

title "recent-changes"

(when "a repository is specified"
  args=(--repository "$repository")
  (when "showing recent changes without options"
    it "produces human-redable output" && {
      expect_run $SUCCESSFULLY $exe recent-changes "${args[@]}"
    }
  )
  (when "showing recent changes with the --output=json flag"
    it "produces json output" && {
      expect_run $SUCCESSFULLY $exe recent-changes --output=json "${args[@]}"
    }
  )
)

