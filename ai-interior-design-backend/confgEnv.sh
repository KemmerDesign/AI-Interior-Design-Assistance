#!/bin/bash

# Establece la variable LLVM_CONFIG_PATH
export LLVM_CONFIG_PATH="/usr/bin/llvm-config"

# Establece la variable LIBCLANG_PATH
export LIBCLANG_PATH="/usr/lib64"

# Establece la variable PATH
export PATH="/usr/lib64:/usr/bin:/home/kdesign/.pyenv/plugins/pyenv-virtualenv/shims:/home/kdesign/.pyenv/shims:/home/kdesign/.pyenv/bin:/home/linuxbrew/.linuxbrew/bin:/home/linuxbrew/.linuxbrew/sbin:/home/linuxbrew/.linuxbrew/opt/node@22/bin:/home/kdesign/.cargo/bin:/home/kdesign/.local/bin:/home/kdesign/bin:/usr/local/bin:/usr/local/sbin:/usr/sbin:/var/lib/snapd/snap/bin"

echo "Variables de entorno configuradas para RustRover:"
echo "LLVM_CONFIG_PATH: $LLVM_CONFIG_PATH"
echo "LIBCLANG_PATH: $LIBCLANG_PATH"
echo "PATH: $PATH"

