#!/usr/bin/env bash
set -e

RUSTC="$1"
shift

CMD=("$RUSTC")

# Codex's sandbox cannot connect to the host sccache daemon.
if [[ -z "$NO_SCCACHE" && -z "$CODEX_THREAD_ID" ]] && command -v sccache >/dev/null 2>&1; then
	CMD=(sccache "$RUSTC")
fi

MOLD_ARGS=()
if command -v mold >/dev/null 2>&1; then
	# Mold is an ELF linker. Do not override an explicitly configured linker or
	# inject it for non-Linux cross targets (for example windows-msvc).
	target=""
	for ((i = 1; i <= $#; i++)); do
		arg="${!i}"
		if [[ "$arg" == "--target" || "$arg" == "-target" ]]; then
			next=$((i + 1))
			target="${!next}"
		elif [[ "$arg" == --target=* || "$arg" == -target=* ]]; then
			target="${arg#*=}"
		fi
	done
	if [[ "$*" == *"-C linker="* ]] || [[ "$*" == *"wasm32"* ]] || \
		[[ "$*" == *"riscv"* ]] || [[ -n "$target" && "$target" != *"linux"* ]]; then
		: # skip mold
	else
		MOLD_ARGS=("-C" "link-arg=-fuse-ld=mold")
	fi
fi

# Our injected flags must come *after* the passthrough args ("$@"), not
# before: when invoked through clippy-driver, $RUSTC is clippy-driver
# itself and the real rustc path is the first element of "$@" -- it must
# immediately follow, or clippy-driver misparses it as an input filename.
CPU_ARGS=()
if [[ -z "$target" || "$target" == *"linux"* ]]; then
	CPU_ARGS=("-C" "target-cpu=native")
fi
exec "${CMD[@]}" "$@" "${CPU_ARGS[@]}" "${MOLD_ARGS[@]}"
