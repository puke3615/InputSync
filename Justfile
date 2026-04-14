default: dev

dev:
  cargo tauri dev

build:
  cargo tauri build

release:
  cargo tauri build --release

run:
  cargo tauri dev