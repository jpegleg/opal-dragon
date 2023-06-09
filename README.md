![cdlogo](https://carefuldata.com/images/cdlogo.png)

# opal-dragon

Opal-dragon generates an ed25519 keypair, signs your message (terminated by newline character in the defaults), throws away the private key, and gives back a UUID, signature, and public key. It will also verify a message and signature with a provided public key. Send the signature and data to the business parter or friend that is integrated with opal-dragon to automatically verify after pulling the public key from the trusted central server. We can treat this like a type of blockchain component, or we can treat it however we want. But unlike a typical blockchain, with just opal-dragon any valid ed25519 signature can be verified, regardless of who signed it, if the Opal protocol is used and the person or software requesting the verification has a valid client certificate for access. And then all verifications and signatures are logged transactional events on the server, with linked data structures provided to the client. We can use opal-dragon this way on the fly, or baked into an ecosystem. The server and/or client can be running on user workstations and/or servers. Instead of the client being a CLI tool, the client could also be a server function as part of a set of microservices.

The server is designed to easily be distributed, the server data can be collected by another service such as Kubernetes and consolidated. The UUIDv4 and time data makes the order of events in the pool not important, they can be effectively sorted by the UTC time data but also each event can be treated granularly and still be effective.

## documentation

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
2023-04-19 01:46:49.184841804 UTC f46899c2-2c4f-45fa-bf49-d1cb103f1519 opal_dragon - INFO public key: "Qi8YyX9xanvHcAOrs8fL5kpdvxx7qzmY38KeW6iAhuE"
2023-04-19 01:47:42.244975790 UTC 179e7981-d277-4172-849e-2d0ea75f6f6b opal_dragon - INFO public key: "qlS1yTr+m8QIvzJzHm9yru6m5D9xY9vl8vJ9zVABijs"
2023-04-19 01:47:42.245006673 UTC 179e7981-d277-4172-849e-2d0ea75f6f6b opal_dragon - DEBUG message: [102, 105, 103, 115, 32, 109, 101, 32, 97, 32, 103, 111, 108, 98, 97]
2023-04-19 01:47:42.245006673 UTC 179e7981-d277-4172-849e-2d0ea75f6f6b opal_dragon - DEBUG public key: PublicKey(CompressedEdwardsY: [66, 47, 24, 201, 127, 113, 106, 123, 199, 112, 3, 171, 179, 199, 203, 230, 74, 93, 191, 28, 123, 171, 57, 152, 223, 194, 158, 91, 168, 128, 134, 225]), EdwardsPoint{
        X: FieldElement51([227343918959931, 919696995717594, 712638284010017, 1244451500287507, 2250012156348963]),
        Y: FieldElement51([687743602011970, 1817906884702061, 841629638012702, 404397424491918, 1715684992465388]),
        Z: FieldElement51([1, 0, 0, 0, 0]),
        T: FieldElement51([219127912651305, 503706718163498, 1828701904867182, 518311332615125, 762606978010524])
})
2023-04-19 01:47:42.245006673 UTC 179e7981-d277-4172-849e-2d0ea75f6f6b opal_dragon - DEBUG signature: ed25519::Signature(A34D75436E1A7A29F71C867ABAB381A9E8007AFC3E1DED135D3901B13C1E93D46C4AB763112BD2D994ABB7B3E60DDB8E68B6D00A75FC6828AE56ACE8624E1F03)
2023-04-19 01:47:42.245006673 UTC 179e7981-d277-4172-849e-2d0ea75f6f6b opal_dragon - DEBUG verify: true
```


## rustls mTLS

We have rustls with client and server authentication.

The client side needs:

- opal_client_access.pem as the client identity certificate signed by the client_ca.pem pair
- opal_client_access.key as the client identity secret that generated the csr for the client identity sertificate sign
- opal_server_ca.pem as the server identity certificate ca root

And the server needs:

- server.pem as the server identity leaf certificate
- server.key as the server identity secret
- client_ca.pem as the certificate from the pair that signed the client request

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

The server generates a key pair and doesn't use it at all when a verification is processed. Removing that could be an optimization, but it is a low cost operation that helps keep the data "the same shape". This "shape" helps anonamize activity (if DEBUG is removed), so that each line is "the same" whether it was a sign or verify.

Edit the source code to change the ports or target url/ips.  They are intentionally static after compiling for security. Set them in the code per use case and then compile to change them.

The provided client can take piped data into the binary:

```
$ echo -e "foo\n\n\ns\n" | opal-client

Started opal_dragon client session.

Enter a message:
To verify, provide a public key, base64 encoded, otherwise leave empty and hit return (newline): 
To verify, provide a signature, hex encoded, otherwise leave empty and hit return (newline): 
Select an option, (s)ign or (v)erify: 
opal_dragon: 'ee7dd54d-953a-49f9-aee7-4a1aeea271c3 foo ed25519::Signature(99D1556C1EDE325A7650B83EB2382EAB606FFC6655466116D0A2CDA63BDA66DD6D13A78EAD6D98D2E44063C219C3BFA322ADB5C3375377BEF08DF6B2965F6D00) "lbiSMmwkQVwRq0+7PrImIJAJsf2UM0uY1zJmZh9sXsA"'

```

The message in that above example is `foo` via that echo of `foon\n\ns\n`. 

Here is an example of piping in a verification in the same manner, adding the linux `time` command to the front to measure the duration as well:

```
$ time echo -e "foo\n26suMHg8MVdq+zbgctZJqVr8enJnHZ37ZreDbiJi+9M\n3C21A5E9ED94B02E84DDD684879B1C3C8A8F2DA8CE56837883E129C78C62AACACFDE8B5109AF224327CE87CBFE0F07738341595B6D0319766600A32AB66F3A08\nv\n" | opal-client

Started opal_dragon client session.

Enter a message: 
To verify, provide a public key, base64 encoded, otherwise leave empty and hit return (newline): 
To verify, provide a signature, hex encoded, otherwise leave empty and hit return (newline): 
Select an option, (s)ign or (v)erify: 
opal_dragon: '1d1dffef-66d0-4396-a70b-605334257fc7 true'


real    0m0.008s
user    0m0.005s
sys     0m0.001s


```


The client uses localhost by default, but that can be a remote server, like a friend's or organization's instance of the microservice on the internet. The opal-dragon server can be used ephemerally, but can also be treated as a ledger of activity. The data dumps to STDOUT by default, which works well in an OCI container microservice, this is a cloud native design. Comment or remove the DEBUG lines to reduce server log size, or otherwise replace with your own logging needs.

The server (gRPC microservice) only accepts the custom Opal gRPC spec and only authenticated via the client signed certificate. 

The mTLS keys and certs are files in $pwd by default for both client and server. The client might be coded to a fixed path in /etc instead.

Sending no value for the unary will result in a gRPC disconnect error. Sending a value that isn't expected will be treated as an OK message. An OK message still generates a keypair and looks exactly the same as a sign on the server log, but the client will only recieve UUID and the string OK.

Another note: the default DEBUG messages do log the message body as bytes during verification. The DEBUG logging is only on verifications by default, dumping each variable in the verification process to STDOUT with a linked uuid. 

### Why call it opal protocol?

Opals are mineraloids, aka stones that contain water. The more water they contain the harder they are.
Opal gem stones have been seen as valuable and to have protective magic. In opal-dragon, it isn't magic, its math, but it is protective in terms of mTLS with rustls and providing a message authenticity service component. 

