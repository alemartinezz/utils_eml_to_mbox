# EML to MBOX

Usually when you want to migrate emails from one mailbox to another, you'll encounter the case where the emails are downloaded in `eml` format.

This is a problem when you want to import them into another mail client such as apple mail. The client just supports `mbox` format. So, this program allows you to, by specifying a path, will batch convert all the `eml` files into `mbox`, allowing you to import them seamlessly into you email client.

## Usage

```bash
cargo build --release && cargo run --release
```

Specify the origin folder or file, ex:

```bash
~/<username>/Downloads
```

Specify destination folder, ex:

```bash
~/<username>/Downloads
```

This will create a folder named `output.mbox` in the specified folder, containing all the converted files.

---
