Rust Web Push
=============

[![Cargo tests](https://github.com/pimeys/rust-web-push/actions/workflows/test.yml/badge.svg)](https://github.com/pimeys/rust-web-push/actions/workflows/test.yml)
[![crates.io](http://meritbadge.herokuapp.com/web_push)](https://crates.io/crates/web_push)
[![docs.rs](https://docs.rs/web-push/badge.svg)](https://docs.rs/web-push)

[Matrix chat](https://matrix.to/#/#rust-push:nauk.io?via=nauk.io&via=matrix.org&via=shine.horse)

Web push notification sender.

## Requirements

Needs a Tokio executor version 0.2 or later and Rust compiler version 1.39.0 or later.

## Usage

To send a web push from command line, first subscribe to receive push
notifications with your browser and store the subscription info into a json
file. It should have the following content:

``` json
{
  "endpoint": "https://updates.push.services.mozilla.com/wpush/v1/TOKEN",
  "keys": {
    "auth": "####secret####",
    "p256dh": "####public_key####"
  }
}
```

Google has
[good instructions](https://developers.google.com/web/updates/2015/03/push-notifications-on-the-open-web) for
building a frontend to receive notifications.

Store the subscription info to `examples/test.json` and send a notification with
`cargo run --example simple_send -- -f examples/test.json -p "It works!"`. If
using Google Chrome, you need to register yourself
into [Firebase](https://firebase.google.com/) and provide a GCM API Key with
parameter `-k GCM_API_KEY`.

Examples
--------

To see it used in a real project, take a look to the [XORC
Notifications](https://github.com/xray-tech/xorc-notifications), which is a
full-fledged consumer for sending push notifications.

VAPID
-----

VAPID authentication prevents unknown sources sending notifications to the
client and allows sending notifications to Chrome without signing in to Firebase
and providing a GCM API key.

The private key to be used by the server can be generated with OpenSSL:

```
openssl ecparam -genkey -name prime256v1 -out private_key.pem
```

To derive a public key from the just-generated private key, to be used in the
JavaScript client:

```
openssl ec -in private_key.pem -pubout -outform DER|tail -c 65|base64|tr '/+' '_-'|tr -d '\n'
```

The signature is created with `VapidSignatureBuilder`. It automatically adds the
required claims `aud` and `exp`. Adding these claims to the builder manually
will override the default values.

Overview
--------

Currently implements
[HTTP-ECE Draft-3](https://datatracker.ietf.org/doc/draft-ietf-httpbis-encryption-encoding/03/?include_text=1)
content encryption for notification payloads and
[HTTP-ECE RFC 8188](https://datatracker.ietf.org/doc/rfc8188/). The client requires
[Tokio](https://tokio.rs) for asynchronious requests.

Tested with Google's and Mozilla's push notification services.

Debugging
--------
If you get an error or the push notification doesn't work you can try to debug using the following instructions:

Add the following to your Cargo.toml:
```cargo
log = "0.4"
pretty_env_logger = "0.3"
```

Add the following to your main.rs:
```rust
extern crate pretty_env_logger;
// ...
fn main() {
  pretty_env_logger::init();
  // ...
}
```

Or use any other logging library compatible with https://docs.rs/log/

Then run your program with the following environment variables:
```bash
RUST_LOG="web_push::client=trace" cargo run
```

This should print some more information about the requests to the push service which may aid you or somebody else in finding the error.
