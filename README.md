# concerto-nom

A parser for [Concerto](https://github.com/accordproject/concerto) modeling language, written in Rust.

It's not an official parser, neither is it complete. I am learning [nom](https://github.com/rust-bakery/nom/tree/main) parser-combinator library by applying it to Concerto.

## Current state

Input:

```
namespace com.example.foo@1.3.5-pre

concept Person {
  o String name
  o Integer age optional
  o Address mainAddress
}

concept Address {
  o String street
  o Integer number optional
  o String city default=\"Dublin\"
}
```

AST:

```json
{
  "namespace": "com.example.foo@1.3.5-pre",
  "declarations": [
    {
      "name": "Person",
      "properties": [
        {
          "$class": "StringProperty",
          "name": "name",
          "isOptional": false,
          "isArray": false
        },
        {
          "$class": "IntegerProperty",
          "name": "age",
          "isOptional": true,
          "isArray": false
        },
        {
          "$class": "Address",
          "name": "mainAddress",
          "isOptional": false,
          "isArray": false
        }
      ]
    },
    {
      "name": "Address",
      "properties": [
        {
          "$class": "StringProperty",
          "name": "street",
          "isOptional": false,
          "isArray": false
        },
        {
          "$class": "IntegerProperty",
          "name": "number",
          "isOptional": true,
          "isArray": false
        },
        {
          "$class": "StringProperty",
          "name": "city",
          "isOptional": false,
          "isArray": false,
          "default": "Dublin"
        }
      ]
    }
  ]
}
```

Internal representation:

```rust
Model {
  namespace: Namespace {
    name: "com.example.foo",
    version: VersionWithRelease(VersionNumber { major: 1, minor: 3, patch: 5 }, "pre")
  },
  declarations: [
    Declaration {
      name: "Person",
      properties: [
        String(StringProperty {
          name: "name",
          is_optional: false,
          is_array: false,
          default_value: None,
          regex_validator: None,
          length_validator: None
        }),
        Integer(IntegerProperty {
          name: "age",
          is_optional: true,
          is_array: false,
          default_value: None,
          domain_validator: None
        }),
        Concept(Property {
          class: "Address",
          name: "mainAddress",
          is_optional: false,
          is_array: false
        })
      ]
    },
    Declaration {
      name: "Address",
      properties: [
        String(StringProperty {
          name: "street",
          is_optional: false,
          is_array: false,
          default_value: None,
          regex_validator: None,
          length_validator: None
        }),
        Integer(IntegerProperty {
          name: "number",
          is_optional: true,
          is_array: false,
          default_value: None,
          domain_validator: None
        }),
        String(StringProperty {
          name: "city",
          is_optional: false,
          is_array: false,
          default_value: Some("Dublin"),
          regex_validator: None,
          length_validator: None
        })
      ]
    }
  ]
}
```

## Change log

### 0.0.1

- Fully qualified name parser
- Namespace parser

### 0.0.2

- Property parser

### 0.0.3 (ongoing)

- Declaration parser
