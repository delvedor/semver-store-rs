# semver-store

[![Crates.io](https://img.shields.io/crates/v/semver-store)](https://crates.io/crates/semver-store)  ![Crates.io](https://img.shields.io/badge/maintenance-experimental-blue.svg)

An HashMap structure that uses [semver](https://semver.org) strings as keys.

# Install

```sh
cargo add semver-store
```

# API

### `insert`
Add a value to the store for a given version.

```rust
let mut store = SemverStore::<String>::new();
store.insert(&"1.0.0".to_string(), "hello!".to_string());
```

### `contains_key`
Checks if a key has been inserted.

```rust
let mut store = SemverStore::<String>::new();
store.insert(&"1.0.0".to_string(), "hello!".to_string());
assert_eq!(store.contains_key(&"1.0.0".to_string()), true);
```

### `get`
Get the reference fo a stored value.

```rust
let mut store = SemverStore::<String>::new();
store.insert(&"1.0.0".to_string(), "hello!".to_string());
assert_eq!(store.get(&"1.0.0".to_string()).unwrap(), &"hello");
```

Wildcards are supported! If you use a wildcard you will always get the maximum version for a give major/minor.

```rust
let mut store = SemverStore::<String>::new();
store.insert(&"1.0.0".to_string(), "hello!".to_string());
store.insert(&"1.1.0".to_string(), "world!".to_string());
assert_eq!(store.get(&"1.x".to_string()).unwrap(), &"world");

store.insert(&"2.1.1".to_string(), "hello!".to_string());
store.insert(&"2.1.2".to_string(), "world!".to_string());
assert_eq!(store.get(&"2.1.x".to_string()).unwrap(), &"world");
assert_eq!(store.get(&"2.1".to_string()).unwrap(), &"world");
```

### `remove`
Removes a given version from the store.

```rust
let mut store = SemverStore::<String>::new();
store.insert(&"1.0.0".to_string(), "hello!".to_string());
assert_eq!(store.get(&"1.0.0".to_string()).unwrap(), &"hello");

store.remove(&"1.0.0".to_string());
assert_eq!(store.get(&"1.0.0".to_string()), None);
```

Wildcards are supported! If you use a wildcard you will always delete the maximum version for a give major/minor.

```rust
let mut store = SemverStore::<String>::new();
store.insert(&"1.0.0".to_string(), "hello!".to_string());
store.insert(&"1.1.0".to_string(), "hello!".to_string());
assert_eq!(store.get(&"1.0.0".to_string()).unwrap(), &"hello");

store.remove(&"1.x".to_string());
assert_eq!(store.get(&"1.0.0".to_string()).unwrap(), &"hello");
assert_eq!(store.get(&"1.1.0".to_string()), None);
```

### `empty`
Empties the store.

```rust
let mut store = SemverStore::<String>::new();
store.insert(&"1.0.0".to_string(), "hello!".to_string());
assert_eq!(store.get(&"1.0.0".to_string()).unwrap(), &"hello");

store.empty();
assert_eq!(store.get(&"1.0.0".to_string()), None);
```

# License
[MIT](./LICENSE)
