rm -rf obj
mkdir obj

cd obj

~/rust32/rustc/bin/rustc --target arm-unknown-linux-gnueabihf -O --emit=obj ../src/kernel.rs


