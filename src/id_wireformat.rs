/// Macro to generate stable ID types backed by a fixed-size BLAKE3 byte array.
///
/// # Forms
///
/// ## `id_wireformat!(Foo[N])`
/// Declares a bare ID newtype `Foo([u8; N])` with `ContentHash`, `Debug`,
/// `Copy`, `PartialEq`, `Eq`, `Hash`, `PartialOrd`, `Ord`, and a hex
/// `Display`.  No `From` conversion is generated.
///
/// ## `id_wireformat!(Foo[N] <- Bar)`
/// Same as above, plus `impl From<Bar> for Foo` which content-hashes the
/// whole `Bar` value via [`crate::content_hash::blake3_hash`].
/// Requires `Bar: ContentHash`.
///
/// ## `id_wireformat!(Foo[N] |value: Bar| expr)`
/// Same bare ID as the first form, plus `impl From<Bar> for Foo` where the
/// hash input is the expression `expr` (which has `value: &Bar` in scope).
/// Use this when only part of `Bar` should contribute to identity — for
/// example when `Bar` contains mutable runtime state that must not affect
/// the stable key.
///
/// # Encoding
/// All forms produce a `[u8; N]` taken from the first `N` bytes of the
/// 32-byte BLAKE3 output.  `N` must be ≤ 32.
#[macro_export]
macro_rules! id_wireformat {
    // ------------------------------------------------------------------ //
    // Bare declaration — no From conversion                               //
    // ------------------------------------------------------------------ //
    ($ident:ident[$size:expr]) => {
        #[doc = concat!(
                                    "`", stringify!($ident), "` — a ", stringify!($size),
                                    "-byte stable content-addressed identifier."
                                )]
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $ident(pub [u8; $size]);

        impl ::std::fmt::Debug for $ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}(", stringify!($ident))?;
                for byte in &self.0 {
                    write!(f, "{byte:02x}")?;
                }
                write!(f, ")")
            }
        }

        impl ::std::fmt::Display for $ident {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                for byte in &self.0 {
                    write!(f, "{byte:02x}")?;
                }
                Ok(())
            }
        }

        impl $crate::content_hash::ContentHash for $ident {
            fn hash(&self, state: &mut impl $crate::content_hash::DigestUpdate) {
                state.update(&self.0);
            }
        }

        impl $ident {
            #[doc = concat!(
                        "Derive a `", stringify!($ident),
                        "` from raw bytes: the first ", stringify!($size),
                        " bytes of their BLAKE3 hash. Byte-exact, so inputs that are \
                 not valid UTF-8 stay distinct."
                    )]
            pub fn from_bytes(bytes: &[u8]) -> Self {
                let hash = $crate::content_hash::blake3_hash(bytes);
                let mut out = [0u8; $size];
                out.copy_from_slice(&hash.as_bytes()[..$size]);
                $ident(out)
            }
        }
    };

    // ------------------------------------------------------------------ //
    // From<Thing> by hashing the whole value                              //
    // ------------------------------------------------------------------ //
    ($ident:ident[$size:expr] <- $thing:path) => {
        $crate::id_wireformat!($ident[$size]);

        impl From<&$thing> for $ident {
            fn from(value: &$thing) -> Self {
                let hash = $crate::content_hash::blake3_hash(value);
                let mut bytes = [0u8; $size];
                bytes.copy_from_slice(&hash.as_bytes()[..$size]);
                $ident(bytes)
            }
        }

        impl From<$thing> for $ident {
            fn from(value: $thing) -> Self {
                $ident::from(&value)
            }
        }
    };

    // ------------------------------------------------------------------ //
    // From<Thing> via a custom key-extraction expression                  //
    // ------------------------------------------------------------------ //
    ($ident:ident[$size:expr] | $param:ident : $thing:path | $expr:expr) => {
        $crate::id_wireformat!($ident[$size]);

        impl From<&$thing> for $ident {
            fn from($param: &$thing) -> Self {
                let hash = $crate::content_hash::blake3_hash($expr);
                let mut bytes = [0u8; $size];
                bytes.copy_from_slice(&hash.as_bytes()[..$size]);
                $ident(bytes)
            }
        }

        impl From<$thing> for $ident {
            fn from(value: $thing) -> Self {
                $ident::from(&value)
            }
        }
    };
}
