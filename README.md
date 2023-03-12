PSORT - Picture Sorts
=====================

A quick program to take all photos from the nested _picked directories and symlink them into a new directory.

I want to compile if for the PI...

PI 2 compile
------------
Here is what came back from a search to compile rust for a raspberry pi.
`
$ rustup target add arm-unknown-linux-gnueabihf
$ sudo apt-get install gcc-arm-linux-gnueabihf
$ echo '[target.arm-unknown-linux-gnueabihf]' >> ~/.cargo/config
$ echo 'linker = "arm-linux-gnueabihf-gcc"' >> ~/.cargo/config
$ cd <project dir>
$ cargo build --target=arm-unknown-linux-gnueabihf
`

But this doesn't help keep getting `linking with 'cc' failed: exit status: 1` problems with a `collect2: error: ld returned 1 exit status` ending. So it looks like I have the wrong tool chain? So do I need to just get a docker image to do the compiling for a Pi... More complex than I would like.