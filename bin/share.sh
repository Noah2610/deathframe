# shellcheck source=./util.sh source=./share.sh
_dir="$( dirname "$0" )"
[ -f "${_dir}/util.sh" ] || bash "${_dir}/download-util.sh" || exit 1
source "${_dir}/util.sh"
unset _dir

# https://stackoverflow.com/a/17841619/10927893
function join_by { local IFS="$1"; shift; echo "$*"; }

function cargo_cmd {
  check "cargo"
  local cargo_cmd="$1"
  shift
  cmd="cargo +$RUST_VERSION $cargo_cmd --features amethyst/nightly $*"
  if should_run_in_terminal; then
    run_terminal "$cmd"
  else
    $cmd
  fi
}

LOGFILE="${ROOT}/logs/$( basename "$0" ).log"
RUST_VERSION="nightly-2019-08-13"
