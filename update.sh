#!/usr/bin/env bash

# This script generates the Rust library code from the bl616/618 SVD file.

fail() {
  echo "Error: $*"
  exit 1
}

if [ $# -lt 2 ]; then
  echo "Usage: $0 <path to svd> <path to src output dir> <path to yaml>"
  exit 1
fi

command -v svdtools > /dev/null || fail "Please install svdtools (https://github.com/rust-embedded/svdtools)"
command -v svd2rust > /dev/null || fail "Please install rust2vd (https://github.com/rust-embedded/svd2rust)"
command -v form > /dev/null || fail "Please install form (https://github.com/djmcgill/form)"
command -v rustfmt > /dev/null || fail "Please install rustfmt"

svd_path="${1}"
src_path="${2}"
yaml_path="${3}"

if [ ! -d "${src_path}" ]; then
  fail "src output dir (\`${src_path}') is not a directory"
fi

if [ ! -f "${src_path}/lib.rs" ]; then
  fail "The src output dir (\`${src_path}') does not contain a lib.rs - exiting to avoid accidentally deleting files"
fi

set -x

# Remove the existing code in the src directory
rm -v -rf "${src_path}/"

# Generate the new code
mkdir -v "${src_path}/"

# 生成修补后的SVD文件
svdtools patch $yaml_path

# Generate the Rust code from the SVD
svd2rust -i $svd_path --target riscv

# Split the single generated lib.rs file into Rust modules
form -i lib.rs -o "${src_path}/" && rm lib.rs

# Reformat the code with rustfmt
# find "${src_path}/" -name \*.rs -exec rustfmt -v {} \;
cargo fmt

# 替换链接文件
sed -i 's/device\.x/memory-bl618\.x/g' build.rs