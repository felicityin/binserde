# Description

A binary encoder / decoder supports Rust ([bincode](https://github.com/felicityin/bincode-macro)) and TypeScript ([bincoder](https://github.com/felicityin/bincoder)).

# Create a new library

Create a Rust library.

```
cargo run -- new-lib -t rs -n types -o output/rust
```

Create a TypeScript library.

```
cargo run -- new-lib -t ts -n schemas -o output/typescript/src
```

# Compile

Generate Rust code.

```
cargo run -- compile -t rs \
    -i input/hello.bs input/basic_types.bs input/vec.bs input/nested_struct.bs input/array.bs \
       input/vec_struct.bs input/enums.bs input/option.bs input/blockchain.bs input/complex.bs \
    -o output/rust/types/src/generated
```

Generate TypeSript code.

```
cargo run -- compile -t ts \
    -i input/hello.bs input/basic_types.bs input/vec.bs input/nested_struct.bs input/array.bs \
       input/vec_struct.bs input/enums.bs input/option.bs input/blockchain.bs input/complex.bs \
    -o output/typescript/src/schemas
```

# Note

For TypeScript, you need to install dependencies yourself.

```
yarn add bincoder
```

And you need to format generated files yourself.

For example, format Rust files as follows.

```
cargo fmt
```

# Run demo

Rust

```
cd output/rust
cargo run
```

TypeScript

```
cd output/typescript
yarn start
```
