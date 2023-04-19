![cdlogo](https://carefuldata.com/images/cdlogo.png)

Make a signature and a public key to verify it. Send the signature and data to the business parter or friend and the public key to a central location for all to access. We can treat this like a type of blockchain, or we can treat it however we want. On the fly, or baked into an ecosystem. Instead of the client being a CLI tool, the client could also be a server function as part of a set of microservices.

The server is designed to easily be distributed, the server data can be collected by another service such as Kubernetes and consolidated. The UUIDv4 and time data makes the order of events in the pool not important, each event can be treated granularly and still be effective.

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

```
2023-04-19 01:46:49.184841804 UTC f46899c2-2c4f-45fa-bf49-d1cb103f1519 opal_dragon - Public key used: "Qi8YyX9xanvHcAOrs8fL5kpdvxx7qzmY38KeW6iAhuE"
2023-04-19 01:47:42.244975790 UTC 179e7981-d277-4172-849e-2d0ea75f6f6b opal_dragon - Public key used: "qlS1yTr+m8QIvzJzHm9yru6m5D9xY9vl8vJ9zVABijs"
2023-04-19 01:47:42.245006673 UTC 179e7981-d277-4172-849e-2d0ea75f6f6b opal_dragon - DEBUG message: [102, 105, 103, 115, 32, 109, 101, 32, 97, 32, 103, 111, 108, 98, 97]
2023-04-19 01:47:42.245006673 UTC 179e7981-d277-4172-849e-2d0ea75f6f6b opal_dragon - DEBUG public key bytes: [66, 47, 24, 201, 127, 113, 106, 123, 199, 112, 3, 171, 179, 199, 203, 230, 74, 93, 191, 28, 123, 171, 57, 152, 223, 194, 158, 91, 168, 128, 134, 225]
2023-04-19 01:47:42.245006673 UTC 179e7981-d277-4172-849e-2d0ea75f6f6b opal_dragon - DEBUG public key: PublicKey(CompressedEdwardsY: [66, 47, 24, 201, 127, 113, 106, 123, 199, 112, 3, 171, 179, 199, 203, 230, 74, 93, 191, 28, 123, 171, 57, 152, 223, 194, 158, 91, 168, 128, 134, 225]), EdwardsPoint{
        X: FieldElement51([227343918959931, 919696995717594, 712638284010017, 1244451500287507, 2250012156348963]),
        Y: FieldElement51([687743602011970, 1817906884702061, 841629638012702, 404397424491918, 1715684992465388]),
        Z: FieldElement51([1, 0, 0, 0, 0]),
        T: FieldElement51([219127912651305, 503706718163498, 1828701904867182, 518311332615125, 762606978010524])
})
2023-04-19 01:47:42.245006673 UTC 179e7981-d277-4172-849e-2d0ea75f6f6b opal_dragon - DEBUG signature bytes: [65, 51, 52, 68, 55, 53, 52, 51, 54, 69, 49, 65, 55, 65, 50, 57, 70, 55, 49, 67, 56, 54, 55, 65, 66, 65, 66, 51, 56, 49, 65, 57, 69, 56, 48, 48, 55, 65, 70, 67, 51, 69, 49, 68, 69, 68, 49, 51, 53, 68, 51, 57, 48, 49, 66, 49, 51, 67, 49, 69, 57, 51, 68, 52, 54, 67, 52, 65, 66, 55, 54, 51, 49, 49, 50, 66, 68, 50, 68, 57, 57, 52, 65, 66, 66, 55, 66, 51, 69, 54, 48, 68, 68, 66, 56, 69, 54, 56, 66, 54, 68, 48, 48, 65, 55, 53, 70, 67, 54, 56, 50, 56, 65, 69, 53, 54, 65, 67, 69, 56, 54, 50, 52, 69, 49, 70, 48, 51]
2023-04-19 01:47:42.245006673 UTC 179e7981-d277-4172-849e-2d0ea75f6f6b opal_dragon - DEBUG signature: ed25519::Signature(A34D75436E1A7A29F71C867ABAB381A9E8007AFC3E1DED135D3901B13C1E93D46C4AB763112BD2D994ABB7B3E60DDB8E68B6D00A75FC6828AE56ACE8624E1F03)
2023-04-19 01:47:42.245006673 UTC 179e7981-d277-4172-849e-2d0ea75f6f6b opal_dragon - DEBUG verify: true
```


## rustls mTLS

We have rustls with client and server authentication.

The client side needs:

opal_client_access.pem as the client identity certificate signed by the client_ca.pem pair
opal_client_access.key as the client identity secret that generated the csr for the client identity sertificate sign
opal_server_ca.pem as the server identity certificate ca root

And the server needs:

server.pem as the server identity leaf certificate
server.key as the server identity secret
client_ca.pem as the certificate from the pair that signed the client request

## detached ed25519 as a service

Generate an ephemeral signature on the command line example:

```
$ opal-client

Started opal_dragon client session.

Enter a message: 
Thanks for checking in with us at the future gate! Use ticket 19460-BB2 for your stay with us.
To verify, provide a public key, base64 encoded, otherwise leave empty and hit return (newline): 

To verify, provide a signature, hex encoded, otherwise leave empty and hit return (newline): 

Select an option, (s)ign or (v)erify: 
s
opal_dragon: '945131ea-1b9c-4606-af3a-d9e27835af42 Thanks for checking in with us at the future gate! Use ticket 19460-BB2 for your stay with us. ed25519::Signature(7193742E492F25CEEE2E87DFF3450478E00AF24AB34259AB0812C4AD353200A8BFC0475C7BC88533105CEA08CE475206866B939C7D9EE3790B903FC659B04B04) "dPSAe6CeQAk2oyPHjF50dIo4lnN6ICYMIwI1Oo4Qwl8"'
```

Verify any ed25519 detached signature, such as one made like above:

```
$ opal-client

Started opal_dragon client session.

Enter a message: 
Thanks for checking in with us at the future gate! Use ticket 19460-BB2 for your stay with us.
To verify, provide a public key, base64 encoded, otherwise leave empty and hit return (newline): 
dPSAe6CeQAk2oyPHjF50dIo4lnN6ICYMIwI1Oo4Qwl8
To verify, provide a signature, hex encoded, otherwise leave empty and hit return (newline): 
7193742E492F25CEEE2E87DFF3450478E00AF24AB34259AB0812C4AD353200A8BFC0475C7BC88533105CEA08CE475206866B939C7D9EE3790B903FC659B04B04
Select an option, (s)ign or (v)erify: 
v
opal_dragon: 'b876a5d1-13c0-42fe-a38b-bfbbf5526286 true'

```

## defaults - local client, server externally bound port 3042

The private keys are used quickly and then thrown away after a signature generation: they are one-time use.

The server generates a key pair and doesn't use it at all when a verification is processed. Removing that could be an optimization.

Edit the source code to change the ports or target url/ips.  They are intentionally static after compiling for security. Set them in the code per use case and then compile to change them.

The client uses localhost by default, but that can be a remote server, like a friend's or organization's instance of the microservice on the internet. The opal-dragon server can be used ephemerally, but can also be treated as a ledger of activity. The data dumps to STDOUT by default, which works well in an OCI container microservice, this is a cloud native design. Comment or remove the DEBUG lines to reduce server log size, or otherwise replace with your own logging needs.

The server (gRPC microservice) only accepts the custom Opal gRPC spec and only authenticated via the client signed certificate. 

The mTLS keys and certs are files in $pwd by default for both client and server. The client might be coded to a fixed path in /etc instead.
