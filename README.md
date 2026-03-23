# Boopass
Experimental card emulator for Mario Kart Arcade GP DX

# Installing
> [!NOTE]
> As this is experimental, I do not have releases available.
> If you want to test, please compile from source.

Take both `dinput8.dll` and `boopass.dll` from either building or a release and put it in the folder with `MK_AGP3_FINAL.exe`.

# Building
- Install Rust from [rustup.rs](https://rustup.rs) if you haven't already.
- If you haven't already, add the i686-windows target with `rustup target add i686-pc-windows-msvc`.
- Clone the repostory and enter it with `git clone --recursive https://github.com/Deltara3/Boopass && cd Boopass`.
- Compile with `cargo build`, if you want a release build add `--release`.
- Once done, the `dinput8.dll` proxy and `boopass.dll` will in `target/i686-pc-windows-msvc/<configuration>`

The GNU toolchain may work as well but I've not tested it.

# License
All code is licensed under the MIT License.
