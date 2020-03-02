# serde_utils

Serde helper library


### Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1", features = ["derive"]}
serde_utils = { git = "https://github.com/mchesser/serde_utils", features = ["hex", "base64"] }
```

Then annotate structs using:

```rust

#[derive(serde::Serialize)]
struct Data {
    #[serde(with = "serde_utils::hex::u8x6")]
    mac_addr: [u8; 6]

    #[serde(with = "serde_utils::hex::bytes")]
    payload_hex: Vec<u8>

    #[serde(with = "serde_utils::base64::bytes")]
    payload_b64: Vec<u8>

    #[serde(with = "serde_utils::base64::f32")]
    numbers_b64: Vec<f32>
}
```
