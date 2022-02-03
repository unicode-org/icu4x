use zerovec::ule::AsULE;
use zerovec::ZeroVec;
use zerovec_derive::ULE;

#[repr(packed)]
#[derive(ULE, Copy, Clone)]
pub struct FooULE {
    a: u8,
    b: <u32 as AsULE>::ULE,
    c: <char as AsULE>::ULE,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Foo {
    a: u8,
    b: u32,
    c: char,
}

impl AsULE for Foo {
    type ULE = FooULE;
    fn as_unaligned(self) -> FooULE {
        FooULE {
            a: self.a,
            b: self.b.as_unaligned(),
            c: self.c.as_unaligned(),
        }
    }

    fn from_unaligned(other: FooULE) -> Self {
        Self {
            a: other.a,
            b: AsULE::from_unaligned(other.b),
            c: AsULE::from_unaligned(other.c),
        }
    }
}

fn main() {
    let vec = vec![
        Foo {
            a: 101,
            b: 924,
            c: '⸘',
        },
        Foo {
            a: 217,
            b: 4228,
            c: 'ə',
        },
        Foo {
            a: 117,
            b: 9090,
            c: 'ø',
        },
    ];
    let zerovec: ZeroVec<Foo> = vec.iter().copied().collect();

    assert_eq!(zerovec, &*vec);

    let bytes = zerovec.as_bytes();
    let reparsed: ZeroVec<Foo> = ZeroVec::parse_byte_slice(bytes).unwrap();

    assert_eq!(reparsed, &*vec);
}
