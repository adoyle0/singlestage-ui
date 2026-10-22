# Singlestage Test Harness

This is lighter weight test project for working on singlestage without having
to wait for all the macros in the docs site for code blocks and shit

todo: playwright or something

| Server | Port |
| - | - |
| `cargo leptos` | 3000 |
| `trunk` | 3010 |

## Usage

### Dev build in ssr mode

```bash
cargo install cargo-leptos
```

```bash
./dev ssr
```

### Dev build in csr mode

```bash
cargo install trunk
```

```bash
./dev csr
```

### Dev build native

```bash
cargo install tauri-cli
```

```bash
./dev native
```

### Dev build android

See [https://tauri.app/start/prerequisites/#android](https://tauri.app/start/prerequisites/#android)

```bash
cargo install tauri-cli
```

```bash
cargo tauri android init
```

```bash
./dev android
```
