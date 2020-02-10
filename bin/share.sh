# shellcheck source=./util.sh disable=SC2155
function _dl_util_sh {
    local UTIL_VERSION="v2.2.2"
    local dir="$( dirname "$( realpath "$1" )" )"
    [ -f "${dir}/util.sh" ] || bash "${dir}/download-util.sh" "$UTIL_VERSION" || exit 1
    source "${dir}/util.sh"
}; _dl_util_sh "$0"

check "rustup"

function cargo_cmd {
    check_installed_toolchain

    local cargo_sub_cmd
    local args
    local cmd

    cargo_sub_cmd="$1"
    shift
    args=( \
        --features "$RUST_FEATURES" \
        "$@" \
    )
    cmd=( \
        rustup run "$RUST_TOOLCHAIN" \
        cargo "$cargo_sub_cmd" \
        "${args[@]}" \
    )

    msg "$(clr yellow black bold)Running$(clrrs) $(clr "${CLR_CODE[@]}")${cmd[*]}$(clrrs)"
    if should_run_in_terminal; then
        run_terminal "${cmd[@]}"
    else
        ${cmd[*]}
    fi
}

function check_installed_toolchain {
    (rustup toolchain list \
        | grep -Ex "${RUST_TOOLCHAIN}-.+" \
        &> /dev/null) \
        || err_toolchain_not_installed
}

function err_toolchain_not_installed {
    local errmsg="\
$(clr "${CLR_WARN[@]}")Warning:$(clrrs) Rust toolchain $(clr "${CLR_CODE[@]}")${RUST_TOOLCHAIN}$(clrrs) is not installed.
Install automatically with $(clr "${CLR_CODE[@]}")rustup$(clrrs)?"

    prompt_question "$errmsg" || exit 0

    try_run \
        "rustup install $RUST_TOOLCHAIN"
}

LOGFILE="${ROOT}/logs/$( basename "$0" ).log"
RUST_TOOLCHAIN="nightly-2020-02-06"
[ -z "$RUST_FEATURES" ] && RUST_FEATURES="nightly,vulkan,physics"
