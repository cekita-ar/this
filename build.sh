mkdir -p build && for target in x86_64-unknown-linux-gnu x86_64-pc-windows-gnu aarch64-unknown-linux-gnu; do
  cross build --release --target "$target" && \
  arch=$(echo "$target" | cut -d'-' -f1)
  os=$(echo "$target" | cut -d'-' -f3)

  if [ "$os" = "windows" ]; then
    cp "target/$target/release/this.exe" "build/${arch}-${os}-this.exe"
  else
    cp "target/$target/release/this" "build/${arch}-${os}-this"
  fi
done