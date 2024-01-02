# My Website.
I created this website in two parts:

- Front end: HTML/JavaScript/Tailwins
- Back end: Rust using Rocket

example :
![a](/example.png)


# Features

to setup your password for login admin : 
create a environment variable named `ADMIN_PASSWORD` with your password

on windows : 
go on `edit the system environment variables` then click on the `environment variables...` and add `ADMIN_PASSWORD` with an hashed password

to get your hashed password on need to copy paste a rust project :

```rs
use base64::{engine::general_purpose, Engine as _};
use sha2::{Digest, Sha256};

fn main() {
    let hashed = sha1_hash("YOUR PASSWORD");
    println!("{:?}", hashed);
}

fn sha1_hash(input: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(input);
    // `update` can be called repeatedly and is generic over `AsRef<[u8]>`
    hasher.update("String data");
    // Note that calling `finalize()` consumes hasher
    hasher.finalize().into();
    let passwd = general_purpose::STANDARD_NO_PAD.encode(hasher)
}
```
and run it :
```bash
cargo new hashing
cd hashing
cargo add base64
cargo add sha2
cargo run
```
then when you create your password in an environment variable named `ADMIN_PASSWORD`

you will be able to login as admin

on this website, you can acces to admin page :
```
http://localhost/admin
```

if when tou login as admin, you see in sever terminal :

```bash
env password not found
```

you don't save your password in an environment variable as well

----
you can read article with a beautiful UI
when you login as admin :
you can : 
- post article with an image, title, introdution and the article : a markdown
- suppr article
