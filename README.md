![cdlogo](https://carefuldata.com/images/cdlogo.png)

# opal-dragon

Opal-dragon aka opal, is a server (microservice) that communicates with gRPC to clients via mTLS and custom Opal RPC (the opal protobuf).

The client can be used interactively by people, or referenced and implemented in other services if desired.

The current client is meant to be run by a person in an interactive way.

The input message is either signed or verified. To verify, the public key and signature, as well as the original message, must be supplied.

opal-dragon is made from ed25519_dalek and tonic gRPC:

https://docs.rs/ed25519-dalek/latest/ed25519_dalek/

https://docs.rs/tonic/latest/tonic/

## ephemeral one-time-use private keys

The private keys for signing are generated on demand and never exposed or used again. The public key is logged and given out. 

## open ended verify

The verification function works for any ed25519 signature, in the opal-dragon format. It does not have to be a signature created by opal-dragon.

The verification has debugging logging on the server enabled by default that captures details about the public key, signature, and payload.

## rustls mTLS

We have rustls with client and server authentication.
