// This module defines some core types for dealing with accelerated DFA states.
// Briefly, a DFA state can be "accelerated" if all of its transitions except
// for a few loop back to itself. This directly implies that the only way out
// of such a state is if a byte corresponding to one of those non-loopback
// transitions is found. Such states are often found in simple repetitions in
// non-Unicode regexes. For example, consider '(?-u)[^a]+a'. We can look at its
// DFA with regex-cli:
//
//     $ regex-cli debug dfa dense '(?-u)[^a]+a' -BbC
//     dense::DFA(
//     D 000000:
//     Q 000001:
//      *000002:
//     A 000003: \x00-` => 3, a => 5, b-\xFF => 3
//      >000004: \x00-` => 3, a => 4, b-\xFF => 3
//       000005: \x00-\xFF => 2, EOI => 2
//     )
//
// In particular, state 3 is accelerated (shown via the 'A' indicator) since
// the only way to leave that state once entered is to see an 'a' byte. If
// there is a long run of non-'a' bytes, then using something like 'memchr'
// to find the next 'a' byte can be significantly faster than just using the
// standard byte-at-a-time state machine.
//
// Unfortunately, this optimization rarely applies when Unicode is enabled.
// For example, patterns like '[^a]' don't actually match any byte that isn't
// 'a', but rather, any UTF-8 encoding of a Unicode scalar value that isn't
// 'a'. This makes the state machine much more complex---far beyond a single
// state---and removes the ability to easily accelerate it. (Because if the
// machine sees a non-UTF-8 sequence, then the machine won't match through it.)
//
// In practice, we only consider accelerating states that have 3 or fewer
// non-loop transitions. At a certain point, you get diminishing returns, but
// also because that's what the memchr crate supports. The structures below
// hard-code this assumption and provide (de)serialization APIs for use inside
// a DFA.
//
// And finally, note that there is some trickery involved in making it very
// fast to not only check whether a state is accelerated at search time, but
// also to access the bytes to search for to implement the acceleration itself.
// dfa/special.rs provides more detail, but the short story is that all
// accelerated states appear contiguously in a DFA. This means we can represent
// the ID space of all accelerated DFA states with a single range. So given
// a state ID, we can determine whether it's accelerated via
//
//     min_accel_id <= id <= max_accel_id
//
// And find its corresponding accelerator with:
//
//     accels.get((id - min_accel_id) / dfa_stride)

use core::convert::{TryFrom, TryInto};

#[cfg(feature = "std")]
use alloc::{vec, vec::Vec};

use crate::regex_automata::util::bytes::{self, DeserializeError, Endian, SerializeError};

type AccelTy = u32;

const ACCEL_TY_SIZE: usize = core::mem::size_of::<AccelTy>();

const ACCEL_LEN: usize = 4;

const ACCEL_CAP: usize = 8;

#[inline(always)]
pub(crate) fn find_fwd(needles: &[u8], haystack: &[u8], at: usize) -> Option<usize> {
    let bs = needles;
    let i = match needles.len() {
        1 => memchr::memchr(bs[0], &haystack[at..])?,
        2 => memchr::memchr2(bs[0], bs[1], &haystack[at..])?,
        3 => memchr::memchr3(bs[0], bs[1], bs[2], &haystack[at..])?,
        0 => panic!("cannot find with empty needles"),
        n => panic!("invalid needles length: {}", n),
    };
    Some(at + i)
}

#[inline(always)]
pub(crate) fn find_rev(needles: &[u8], haystack: &[u8], at: usize) -> Option<usize> {
    let bs = needles;
    match needles.len() {
        1 => memchr::memrchr(bs[0], &haystack[..at]),
        2 => memchr::memrchr2(bs[0], bs[1], &haystack[..at]),
        3 => memchr::memrchr3(bs[0], bs[1], bs[2], &haystack[..at]),
        0 => panic!("cannot find with empty needles"),
        n => panic!("invalid needles length: {}", n),
    }
}

#[derive(Clone)]
pub(crate) struct Accels<A> {
    accels: A,
}

#[cfg(feature = "std")]
impl Accels<Vec<AccelTy>> {
    pub fn empty() -> Accels<Vec<AccelTy>> {
        Accels { accels: vec![0] }
    }

    pub fn add(&mut self, accel: Accel) {
        self.accels.extend_from_slice(&accel.as_accel_tys());
        let len = self.len();
        self.set_len(len + 1);
    }

    fn set_len(&mut self, new_len: usize) {
        // The only way an accelerator gets added is if a state exists for
        // it, and if a state exists, then its index is guaranteed to be
        // representable by a AccelTy by virtue of the guarantees provided by
        // StateID.
        let new_len = AccelTy::try_from(new_len).unwrap();
        self.accels[0] = new_len;
    }
}

impl<'a> Accels<&'a [AccelTy]> {
    pub unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(Accels<&'a [AccelTy]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr() as usize;

        let (count, _) = bytes::try_read_u32_as_usize(slice, "accelerators count")?;
        // The accelerator count is part of the accel_tys slice that
        // we deserialize. This is perhaps a bit idiosyncratic. It would
        // probably be better to split out the count into a real field.

        let accel_tys_count = bytes::add(
            bytes::mul(count, 2, "total number of accelerator accel_tys")?,
            1,
            "total number of accel_tys",
        )?;
        let accel_tys_len = bytes::mul(
            ACCEL_TY_SIZE,
            accel_tys_count,
            "total number of bytes in accelerators",
        )?;
        bytes::check_slice_len(slice, accel_tys_len, "accelerators")?;
        bytes::check_alignment::<AccelTy>(slice)?;
        let accel_tys = &slice[..accel_tys_len];
        slice = &slice[accel_tys_len..];
        // SAFETY: We've checked the length and alignment above, and since
        // slice is just bytes, we can safely cast to a slice of &[AccelTy].
        #[allow(unused_unsafe)]
        let accels = unsafe {
            core::slice::from_raw_parts(accel_tys.as_ptr() as *const AccelTy, accel_tys_count)
        };
        Ok((Accels { accels }, slice.as_ptr() as usize - slice_start))
    }
}

impl<A: AsRef<[AccelTy]>> Accels<A> {
    #[cfg(feature = "std")]
    pub fn to_owned(&self) -> Accels<Vec<AccelTy>> {
        Accels {
            accels: self.accels.as_ref().to_vec(),
        }
    }

    pub fn as_ref(&self) -> Accels<&[AccelTy]> {
        Accels {
            accels: self.accels.as_ref(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        let accels = self.accels.as_ref();
        // SAFETY: This is safe because accels is a just a slice of AccelTy,
        // and u8 always has a smaller alignment.
        unsafe {
            core::slice::from_raw_parts(accels.as_ptr() as *const u8, accels.len() * ACCEL_TY_SIZE)
        }
    }

    pub fn memory_usage(&self) -> usize {
        self.as_bytes().len()
    }

    #[inline(always)]
    pub fn needles(&self, i: usize) -> &[u8] {
        if i >= self.len() {
            panic!("invalid accelerator index {}", i);
        }
        let bytes = self.as_bytes();
        let offset = ACCEL_TY_SIZE + i * ACCEL_CAP;
        let len = bytes[offset] as usize;
        &bytes[offset + 1..offset + 1 + len]
    }

    pub fn len(&self) -> usize {
        // This should never panic since deserialization checks that the
        // length can fit into a usize.
        usize::try_from(self.accels.as_ref()[0]).unwrap()
    }

    fn get(&self, i: usize) -> Option<Accel> {
        if i >= self.len() {
            return None;
        }
        let offset = ACCEL_TY_SIZE + i * ACCEL_CAP;
        let accel = Accel::from_slice(&self.as_bytes()[offset..])
            .expect("Accels must contain valid accelerators");
        Some(accel)
    }

    fn iter(&self) -> IterAccels<'_, A> {
        IterAccels { accels: self, i: 0 }
    }

    pub fn write_to<E: Endian>(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        assert_eq!(
            nwrite % ACCEL_TY_SIZE,
            0,
            "expected accelerator bytes written to be a multiple of {}",
            ACCEL_TY_SIZE,
        );
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("accelerators"));
        }

        // The number of accelerators can never exceed AccelTy::MAX.
        E::write_u32(AccelTy::try_from(self.len()).unwrap(), dst);
        // The actual accelerators are just raw bytes and thus their endianness
        // is irrelevant. So we can copy them as bytes.
        dst[ACCEL_TY_SIZE..nwrite].copy_from_slice(&self.as_bytes()[ACCEL_TY_SIZE..nwrite]);
        Ok(nwrite)
    }

    pub fn validate(&self) -> Result<(), DeserializeError> {
        for chunk in self.as_bytes()[ACCEL_TY_SIZE..].chunks(ACCEL_CAP) {
            let _ = Accel::from_slice(chunk)?;
        }
        Ok(())
    }

    pub fn write_to_len(&self) -> usize {
        self.as_bytes().len()
    }
}

impl<A: AsRef<[AccelTy]>> core::fmt::Debug for Accels<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Accels(")?;
        let mut list = f.debug_list();
        for a in self.iter() {
            list.entry(&a);
        }
        list.finish()?;
        write!(f, ")")
    }
}

#[derive(Debug)]
struct IterAccels<'a, A: AsRef<[AccelTy]>> {
    accels: &'a Accels<A>,
    i: usize,
}

impl<'a, A: AsRef<[AccelTy]>> Iterator for IterAccels<'a, A> {
    type Item = Accel;

    fn next(&mut self) -> Option<Accel> {
        let accel = self.accels.get(self.i)?;
        self.i += 1;
        Some(accel)
    }
}

#[derive(Clone)]
pub(crate) struct Accel {
    bytes: [u8; ACCEL_CAP],
}

impl Accel {
    #[cfg(feature = "std")]
    pub fn new() -> Accel {
        Accel {
            bytes: [0; ACCEL_CAP],
        }
    }

    pub fn from_slice(mut slice: &[u8]) -> Result<Accel, DeserializeError> {
        slice = &slice[..core::cmp::min(ACCEL_LEN, slice.len())];
        let bytes = slice
            .try_into()
            .map_err(|_| DeserializeError::buffer_too_small("accelerator"))?;
        Accel::from_bytes(bytes)
    }

    fn from_bytes(bytes: [u8; 4]) -> Result<Accel, DeserializeError> {
        if bytes[0] as usize >= ACCEL_LEN {
            return Err(DeserializeError::generic(
                "accelerator bytes cannot have length more than 3",
            ));
        }
        Ok(Accel::from_bytes_unchecked(bytes))
    }

    fn from_bytes_unchecked(bytes: [u8; 4]) -> Accel {
        Accel {
            bytes: [bytes[0], bytes[1], bytes[2], bytes[3], 0, 0, 0, 0],
        }
    }

    #[cfg(feature = "std")]
    pub fn add(&mut self, byte: u8) -> bool {
        if self.len() >= 3 {
            return false;
        }
        assert!(
            !self.contains(byte),
            "accelerator already contains {:?}",
            crate::regex_automata::util::DebugByte(byte)
        );
        self.bytes[self.len() + 1] = byte;
        self.bytes[0] += 1;
        true
    }

    pub fn len(&self) -> usize {
        self.bytes[0] as usize
    }

    #[cfg(feature = "std")]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn needles(&self) -> &[u8] {
        &self.bytes[1..1 + self.len()]
    }

    #[cfg(feature = "std")]
    fn contains(&self, byte: u8) -> bool {
        self.needles().iter().position(|&b| b == byte).is_some()
    }

    #[cfg(feature = "std")]
    fn as_accel_tys(&self) -> [AccelTy; 2] {
        assert_eq!(ACCEL_CAP, 8);
        // These unwraps are OK since ACCEL_CAP is set to 8.
        let first = AccelTy::from_ne_bytes(self.bytes[0..4].try_into().unwrap());
        let second = AccelTy::from_ne_bytes(self.bytes[4..8].try_into().unwrap());
        [first, second]
    }
}

impl core::fmt::Debug for Accel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Accel(")?;
        let mut set = f.debug_set();
        for &b in self.needles() {
            set.entry(&crate::regex_automata::util::DebugByte(b));
        }
        set.finish()?;
        write!(f, ")")
    }
}
