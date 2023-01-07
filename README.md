# hider-rust
Implementation of the hider/cypher algorithm by Jorge Blom in Rust, for educational purposes.

# Examples

```
use simple_hider::{hide, unhide};

let text = "Hello";
let salt = "salt";
let encrypted = hide(salt, text);
let decrypted = unhide(salt, encrypted);

assert_eq!(text, decrypted);
```