# If the local version is ahead of the crates
# version - publish

# Get the version of the crate
get_local_version() {
  cargo metadata --format-version=1 --no-deps |
    jq --arg name "$1" '.packages[] | select (.name == $name) | .version' --raw-output
}

# Get the version from crates.io using cargo
get_crates_version() {
  cargo search $1 | grep "$1 =" | cut -d '"' -f 2
}

local_version=$(get_local_version "tergo-tokenizer")
crates_version=$(get_crates_version "tergo-tokenizer")
echo "tergo-tokenizer: local=$local_version, crates=$crates_version"
if [[ $local_version != $crates_version ]]; then
  echo "Publishing tergo-tokenizer $local_version"
  cargo publish -p tergo-tokenizer
fi

local_version=$(get_local_version "tergo-parser")
crates_version=$(get_crates_version "tergo-parser")
echo "tergo-parser: local=$local_version, crates=$crates_version"
if [[ $local_version != $crates_version ]]; then
  echo "Publishing tergo-parser $local_version"
  cargo publish -p tergo-parser
fi

local_version=$(get_local_version "tergo-formatter")
crates_version=$(get_crates_version "tergo-formatter")
echo "tergo-formatter: local=$local_version, crates=$crates_version"
if [[ $local_version != $crates_version ]]; then
  echo "Publishing tergo-formatter $local_version"
  cargo publish -p tergo-formatter
fi

local_version=$(get_local_version "tergo-lib")
crates_version=$(get_crates_version "tergo-lib")
echo "tergo-lib: local=$local_version, crates=$crates_version"
if [[ $local_version != $crates_version ]]; then
  echo "Publishing tergo-lib $local_version"
  cargo publish -p tergo-lib
fi

local_version=$(get_local_version "tergo")
crates_version=$(get_crates_version "tergo")
echo "tergo: local=$local_version, crates=$crates_version"
if [[ $local_version != $crates_version ]]; then
  echo "Publishing tergo $local_version"
  cargo publish -p tergo
fi
