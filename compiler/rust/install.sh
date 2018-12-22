if ! rustup_loc="$(type -p "$rustup")" || [[ -z $rustup_loc ]]; then
  curl https://sh.rustup.rs -sSf | sh
fi

if ! rustfmt_loc="$(type -p "$rustfmt")" || [[ -z $rustfmt_loc ]]; then
  rustup component add rustfmt
fi
