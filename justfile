RESET := "\\x1b[0m"
YELLOW := "\\x1b[33;1m"
MAGENTA := "\\x1b[35;1m"
RED := "\\x1b[31;1m"

dist: check dist_macos dist_linux_x86-64

dist_macos:
    @echo "{{ MAGENTA }}Distributing for MacOS{{ RESET }}"
    @echo "{{ YELLOW }}Building for MacOS...{{ RESET }}"
    cross build --release --target aarch64-apple-darwin
    @echo "{{ YELLOW }}Creating Roketz.app structure...{{ RESET }}"
    mkdir -p dist/macos/Roketz.app/Contents/MacOS
    cp target/aarch64-apple-darwin/release/roketz dist/macos/Roketz.app/Contents/MacOS/
    cp target/aarch64-apple-darwin/release/assets.rdss dist/macos/Roketz.app/Contents/MacOS/
    cp config/Info.plist dist/macos/Roketz.app/Contents/
    @echo "{{ YELLOW }}Creating Roketz.dmg...{{ RESET }}"
    hdiutil create -volname "Roketz" -srcfolder dist/macos/Roketz.app -ov -format UDZO dist/Roketz.dmg
    @echo ""

dist_linux_x86-64:
    @echo "{{ MAGENTA }}Distributing for x86_64 Linux{{ RESET }}"
    @echo "{{ YELLOW }}Building for x86_64 Linux...{{ RESET }}"
    cross build --release --target x86_64-unknown-linux-gnu
    @echo "{{ YELLOW }}Creating Roketz binary...{{ RESET }}"
    mkdir -p dist/linux_x86-64
    cp target/x86_64-unknown-linux-gnu/release/roketz dist/linux_x86-64/
    cp target/x86_64-unknown-linux-gnu/release/assets.rdss dist/linux_x86-64/
    @echo "{{ YELLOW }}Creating Roketz.tar.gz...{{ RESET }}"
    tar -czf dist/Roketz_linux_x86-64.tar.gz -C dist/linux_x86-64 roketz assets.rdss
    @echo ""

check:
    @echo "{{ MAGENTA }}Checking if everything is installed...{{ RESET }}"
    @echo "{{ YELLOW }}Checking for tools...{{ RESET }}"
    ./scripts/check_tools.sh
    @echo "{{ YELLOW }}Checking for rustup targets...{{ RESET }}"
    ./scripts/check_targets.sh
    @echo ""
