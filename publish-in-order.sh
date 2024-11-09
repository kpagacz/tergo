# If the local version is ahead of the crates
# version - publish

# Get the version of the crate
get_local_version() {
  # Parametrize by the name of the crate
  cargo metadata --format-version=1 --no-deps |
    jq --arg name "$1" '.packages[] | select (.name == $name) | .version' --raw-output
}

# Get the version from crates.io using cargo
get_crates_version() {
  cargo search $1 | grep $1 | cut -d '"' -f 2
}

local_version=$(get_local_version "tergo-tokenizer")
crates_version=$(get_crates_version "tergo-tokenizer")
if [ $local_version != $crates_version] then
  cargo publish -p tergo-tokenizer
fi

local_version=$(get_local_version "tergo-parser")
crates_version=$(get_crates_version "tergo-parser")
if [ $local_version != $crates_version] then
  cargo publish -p tergo-parser
fi

local_version=$(get_local_version "tergo-formatter")
crates_version=$(get_crates_version "tergo-formatter")
if [ $local_version != $crates_version] then
  cargo publish -p tergo-formatter
fi

local_version=$(get_local_version "tergo-lib")
crates_version=$(get_crates_version "tergo-lib")
if [ $local_version != $crates_version] then
  cargo publish -p tergo-lib
fi

local_version=$(get_local_version "tergo")
crates_version=$(get_crates_version "tergo")
if [ $local_version != $crates_version] then
  cargo publish -p tergo
fi

