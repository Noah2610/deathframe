# shellcheck source=./util.sh source=./share.sh
_dir="$( dirname "$0" )"
[ -f "${_dir}/util.sh" ] || bash "${_dir}/download-util.sh" || exit 1
source "${_dir}/util.sh"
unset _dir

check "rustup"

function cargo_cmd {
    check_installed_toolchain

    local cargo_sub_cmd
    local args
    local cmd

    cargo_sub_cmd="$1"
    shift
    args=( --features "vulkan,nightly" "$@" )
    cmd=( \
        rustup run "$RUST_TOOLCHAIN" \
        cargo "$cargo_sub_cmd" \
        "${args[@]}" \
    )

    msg_strong "Running $( colored "$COLOR_CODE" "${cmd[*]}" )"
    if should_run_in_terminal; then
        run_terminal "${cmd[*]}"
    else
        ${cmd[*]}
    fi
}

function check_installed_toolchain {
    local errmsg
    errmsg="\
Rust toolchain $( colored "$COLOR_CODE" "${RUST_TOOLCHAIN}" ) is not installed.
Install with ...
    $( colored "$COLOR_CODE" "rustup install ${RUST_TOOLCHAIN}" )"

    (rustup toolchain list \
        | grep -Ex "${RUST_TOOLCHAIN}-.+" \
        &> /dev/null) \
        || err "$errmsg"
}

LOGFILE="${ROOT}/logs/$( basename "$0" ).log"
RUST_TOOLCHAIN="nightly-2019-12-12"
