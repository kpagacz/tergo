#!bin/bash

cargo vendor

mkdir repo_vendor
tar -xvf vendor.tar.xz -C repo_vendor

find ./vendor/ -type f -exec md5sum {} + | sort -k 2 | cut -d" " -f1 >downloaded
find ./repo_vendor/vendor/ -type f -exec md5sum {} + | sort -k 2 | cut -d" " -f1 >in_repo

if [ ! -f downloaded ] || [ ! -f in_repo ]; then
  echo "Error: downloaded or in_repo file not found"
  exit 1
fi

if ! diff downloaded in_repo; then
  echo "Error: vendor files are not the same"
  echo "Please run 'vendor.sh' and commit the changes"
  exit 1
fi
