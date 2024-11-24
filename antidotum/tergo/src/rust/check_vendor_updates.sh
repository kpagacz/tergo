#!bin/bash

cargo vendor

mkdir repo_vendor
tar -xvf vendor.tar.xz -C repo_vendor

find ./vendor -name 'Cargo.toml' -exec cargo generate-lockfile --manifest-path {} \;
find ./vendor -name 'Cargo.toml' -exec cargo pkgid --manifest-path {} \; | rev | cut -d'/' -f1 | rev | sort >downloaded
find ./repo_vendor -name 'Cargo.toml' -exec cargo generate-lockfile --manifest-path {} \;
find ./repo_vendor -name 'Cargo.toml' -exec cargo pkgid --manifest-path {} \; | rev | cut -d'/' -f1 | rev | sort >in_repo

if [ ! -f downloaded ] || [ ! -f in_repo ]; then
  echo "Error: downloaded or in_repo file not found"
  exit 1
fi

if ! diff downloaded in_repo; then
  echo "Error: vendored dependencies are not the same"
  echo "Please run 'vendor.sh' and commit the changes"
  exit 1
fi
