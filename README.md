# sdp-rs

[![Crates.io Version](https://img.shields.io/crates/v/sdp-rs.svg)](https://crates.io/crates/sdp-rs)
[![Released API docs](https://docs.rs/sdp-rs/badge.svg)](https://docs.rs/sdp-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.44.0+-lightgray.svg)](#rust-version-requirements)

A common general purpose library for SDP. It can parse and generate all SDP
structures. Supports both [RFC8866](https://www.rfc-editor.org/rfc/rfc8866.html) and
[RFC4566](https://www.rfc-editor.org/rfc/rfc4566.html).

Like [rsip](https://github.com/vasilakisfil/rsip), this crate is a general purpose library for
common types found when working with the SDP protocol.
You will find high level types like the `SessionDescription`, `MediaDescription` and `Time` but you
will also find line-level types like `Connection` or even types found inside a line, like `Bwtype`
etc.

`sdp-rs` is capable of parsing messages from &str or String using [nom](https://github.com/Geal/nom)
parser and can also generate SDP messages using the main `SessionDescription` struct. Each type
(high level type, line type or sub-line type) can be parsed or be displayed as it would,
so you can work with part of an SDP message, if that's useful for you.

If you need parsing raw bytes (`&[u8]`) ping us. It is possible but we avoided doing that in the
first place because a) it requires tons of traits/generics which will increase complexity and
compile times b) SDP specification highly recommends that the input is UTF-8 c) performance of
converting the bytes to UTF-8 should be negligible.

## Features
* This thing is _fast_, uses nom for basic message parsing.
* Strong (new)types in most cases. Whenever for a type there is a strict specification, we opt for
  a strict (newtype) definition.
* Very simple code structure make it super easy to extend and add new SDP lines and attributes
  As long as you can do [nom](https://github.com/Geal/nom) stuff, it's straightforward.
  If you find dealing with nom difficult, you can always open an issue for the desired (missing)
  type. The goal is to add as many typed SDP attributes as possible.

## Architecture
Each type in `sdp-rs` has a related tokenizer.
This is not enforced by the type system yet, however very soon this will be the case.
In brief, for every `sdp-rs` type we have: 
* Tokenizing: in the lowest level we have the `Tokenizer` which is capable of tokenizing the input.
All common tokenizers accept the `&str` input. You shouldn't have to work directly with the
tokenizers, these are being used indirectly in the parsing level.
* Parsing: once the input has been tokenized into tokens, then there are `TryFrom` impls from the
relevant type tokenizer to the actual type. This is the parsing step where tokens (in the form of
`&str`) are transformed to integers, strings or `sdp-rs` types.
* each `sdp-rs` type implements the `Display` trait and hence has a representation.
