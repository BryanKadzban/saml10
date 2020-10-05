#!/usr/bin/env bash

set -euo pipefail
shopt -s extglob
shopt -s globstar

export RUST_LOG=info
template="$(pwd)/Cargo.toml.template"
patch="$(pwd)/svd.patch"
toml="$(pwd)/Cargo.toml"
export PYTHONPATH=$(pwd)

cat <<-EOF > "$toml"
[workspace]

members = [
EOF

for f in "$1"/*.svd
do
	name="`basename -- "$f"`"
	name="${name%.*}"
	name="${name,,}"
	name="${name##at}"

	mkdir -p svd
	patch -n "$f" -o - < "$patch" > svd/"$name".svd
	svd patch patches/"$name".yaml

	mkdir "$name"
	pushd "$name"
	svd2rust -i ../svd/"$name".svd.patched

	form -i lib.rs -o src
  rm lib.rs

	rustfmt **/*.rs
	sed -i 's/fn async/fn r#async/' src/ac/compctrl.rs
	sed -i 's/pub mode2_alarm0: MODE2_ALARM,/pub mode2_alarm0: self::mode2::MODE2_ALARM,/' src/rtc.rs

  <"$template" sed "s/xxx/$name/" | sed "s/XXX/${name^^}/" > Cargo.toml

  echo "    \"$name\"," >> "$toml"

	popd
done

echo ']' >> "$toml"

cargo build
