# Orandja CBOR

A simple serializer/deserializer implementation of the "[Concise Binary Object Representation](https://tools.ietf.org/html/rfc7049)" format using serde.

# Usage



# Limitations

Due to early developpement, some parts of the protocol and options are not covered. Those are: 

- No TAG support; My goal is to use `#[serde(with = "module")]` to handle that correctly because some tags holds [encoded](https://tools.ietf.org/html/rfc7049#section-2.4.4.2) data that can be decoded on the fly. (e.g. base64 to utf-8)
- No Infinite BYTE, TEXT, ARRAY, MAP; I think (I may be wrong), if an information is split into multiples chunks it's meant to not be decoded as a single bloc. Thus decoding an infinite array into a vector might not be the right choice. So my will is to also use `#[serde(with = "module")]` to describe a clear way to handle that.
- No encode/decode enums as integer. All enums are encoded as string.
- No pretty print for encoded data.

# Technical differences from `serde_cbor` crate

## Unit

Unlike [`serde_cbor`](https://docs.rs/serde_cbor/0.9.0/serde_cbor/) Unit is encoded with the primitive [`0xF7` (Undefined)](https://tools.ietf.org/html/rfc7049#section-3.8) instead of the primitive [`0xF6` (Null)](https://tools.ietf.org/html/rfc7049#section-2.3). It allows `Option<()>` to be treated correctly by the deserializer.

## Enums

Enum's variants are serialized differently. It use a map of one element instead of a list.
From all the possible enum's variants:

```rust
enum Foo {
    First,
    Second(),
    Third(u8),
    Fourth(u8, i8),
    Fifth {
        u8: u8,
        i8: i8,
    },
}
```

1. `Foo::First` No changes for this one (data directly encoded as expected).
```
json: "First"
cbor: [65, 46, 69, 72, 73, 74]
```

2. `Foo::Second()`
```
serde_cbor: 
    json: [ "Second" ]
    cbor: [81, 66, 53, 65, 63, 6f, 6e, 64]
orandja_cbor:
    json: { "Second": [] }
    cbor: [a1, 66, 53, 65, 63, 6f, 6e, 64, 80]
```

3. `Foo::Third(11)`
```
serde_cbor: 
    json: [ "Third", 11 ]
    cbor: [82, 65, 54, 68, 69, 72, 64, 0b]
orandja_cbor:
    json: { "Third": 11 }
    cbor: [a1, 65, 54, 68, 69, 72, 64, 0b]
```

4. `Foo::Forth(11, -7)`
```
serde_cbor: 
    json: [ "Fourth", 11, -7 ]
    cbor: [83, 66, 46, 6f, 75, 72, 74, 68, 0b, 26]
orandja_cbor
    json: { "Fourth": [ 11, -7 ] }
    cbor: [a1, 66, 46, 6f, 75, 72, 74, 68, 82, 0b, 26]
```

5. `Foo::Fifth { u8: 11, i8: -7 }`
```
serde_cbor: 
    json: [ "Fifth", { "u8": 11, "i8", -7 } ]
    cbor: [82, 65, 46, 69, 66, 74, 68, a2, 62, 75, 38, 0b, 62, 69, 38, 26]
orandja_cbor
    json: { "Fifth": { "u8": 11, "i8": -7 } }
    cbor: [a1, 65, 46, 69, 66, 74, 68, a2, 62, 75, 38, 0b, 62, 69, 38, 26]
```  
