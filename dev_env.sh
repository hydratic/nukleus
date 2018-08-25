# DOES NOT COME WITH:
# Python (required)
# NodeJS (required)
# Ruby (optional)
# CMake (required)
#
# --------------------------
# sudo apt-get install nasm
# sudo apt-get install mit-scheme
# sudo apt-get install git
# sudo apt-get install emscripten
#
# pacman -S nasm
# pacman -S mit-scheme
# pacman -S git
# pacman -S emscripten

git clone https://github.com/segfauIt/test_kernel
git clone https://github.com/segfauIt/nukleus
git clone https://github.com/segfauIt/wasm_std

# dependencies
git clone https://github.com/bobbins-wasm/bobbins-wasm
git clone https://github.com/juj/emsdk.git

# dependency setup
cd emsdk
./emsdk install sdk-incoming-64bit
./emsdk activate sdk-incoming-64bit
