use core::convert::TryFrom;

use crate::regex_automata::util::{
    bytes::{DeserializeError, SerializeError},
    DebugByte,
};

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Unit {
    U8(u8),
    EOI(u16),
}

impl Unit {
                                        pub fn u8(byte: u8) -> Unit {
        Unit::U8(byte)
    }

    pub fn eoi(num_byte_equiv_classes: usize) -> Unit {
        assert!(
            num_byte_equiv_classes <= 256,
            "max number of byte-based equivalent classes is 256, but got {}",
            num_byte_equiv_classes,
        );
        Unit::EOI(u16::try_from(num_byte_equiv_classes).unwrap())
    }

    pub fn as_u8(self) -> Option<u8> {
        match self {
            Unit::U8(b) => Some(b),
            Unit::EOI(_) => None,
        }
    }

    #[cfg(feature = "std")]
    pub fn as_eoi(self) -> Option<usize> {
        match self {
            Unit::U8(_) => None,
            Unit::EOI(eoi) => Some(eoi as usize),
        }
    }

    pub fn as_usize(self) -> usize {
        match self {
            Unit::U8(b) => b as usize,
            Unit::EOI(eoi) => eoi as usize,
        }
    }

    pub fn is_eoi(&self) -> bool {
        match *self {
            Unit::EOI(_) => true,
            _ => false,
        }
    }

    #[cfg(feature = "std")]
    pub fn is_word_byte(&self) -> bool {
        self.as_u8().map_or(false, crate::regex_automata::util::is_word_byte)
    }
}

impl core::fmt::Debug for Unit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            Unit::U8(b) => write!(f, "{:?}", DebugByte(b)),
            Unit::EOI(_) => write!(f, "EOI"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);

impl ByteClasses {
            pub fn empty() -> ByteClasses {
        ByteClasses([0; 256])
    }

            #[cfg(feature = "std")]
    pub fn singletons() -> ByteClasses {
        let mut classes = ByteClasses::empty();
        for i in 0..256 {
            classes.set(i as u8, i as u8);
        }
        classes
    }

                        pub fn from_bytes(
        slice: &[u8],
    ) -> Result<(ByteClasses, usize), DeserializeError> {
        if slice.len() < 256 {
            return Err(DeserializeError::buffer_too_small("byte class map"));
        }
        let mut classes = ByteClasses::empty();
        for (b, &class) in slice[..256].iter().enumerate() {
            classes.set(b as u8, class);
        }
        for b in classes.iter() {
            if b.as_usize() >= classes.alphabet_len() {
                return Err(DeserializeError::generic(
                    "found equivalence class greater than alphabet len",
                ));
            }
        }
        Ok((classes, 256))
    }

                    pub fn write_to(
        &self,
        mut dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("byte class map"));
        }
        for b in 0..=255 {
            dst[0] = self.get(b);
            dst = &mut dst[1..];
        }
        Ok(nwrite)
    }

        pub fn write_to_len(&self) -> usize {
        256
    }

        #[inline]
    pub fn set(&mut self, byte: u8, class: u8) {
        self.0[byte as usize] = class;
    }

        #[inline]
    pub fn get(&self, byte: u8) -> u8 {
        self.0[byte as usize]
    }

            #[inline]
    pub unsafe fn get_unchecked(&self, byte: u8) -> u8 {
        *self.0.get_unchecked(byte as usize)
    }

            #[inline]
    pub fn get_by_unit(&self, unit: Unit) -> usize {
        match unit {
            Unit::U8(b) => usize::try_from(self.get(b)).unwrap(),
            Unit::EOI(b) => usize::try_from(b).unwrap(),
        }
    }

    #[inline]
    pub fn eoi(&self) -> Unit {
        Unit::eoi(self.alphabet_len().checked_sub(1).unwrap())
    }

                #[inline]
    pub fn alphabet_len(&self) -> usize {
        // Add one since the number of equivalence classes is one bigger than
        // the last one. But add another to account for the final EOI class
        // that isn't explicitly represented.
        self.0[255] as usize + 1 + 1
    }

                                #[cfg(feature = "std")]
    pub fn stride2(&self) -> usize {
        self.alphabet_len().next_power_of_two().trailing_zeros() as usize
    }

                #[inline]
    pub fn is_singleton(&self) -> bool {
        self.alphabet_len() == 257
    }

        pub fn iter(&self) -> ByteClassIter<'_> {
        ByteClassIter { classes: self, i: 0 }
    }

                                        #[cfg(feature = "std")]
    pub fn representatives(&self) -> ByteClassRepresentatives<'_> {
        ByteClassRepresentatives { classes: self, byte: 0, last_class: None }
    }

        pub fn elements(&self, class: Unit) -> ByteClassElements {
        ByteClassElements { classes: self, class, byte: 0 }
    }

                    fn element_ranges(&self, class: Unit) -> ByteClassElementRanges {
        ByteClassElementRanges { elements: self.elements(class), range: None }
    }
}

impl core::fmt::Debug for ByteClasses {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.is_singleton() {
            write!(f, "ByteClasses({{singletons}})")
        } else {
            write!(f, "ByteClasses(")?;
            for (i, class) in self.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?} => [", class.as_usize())?;
                for (start, end) in self.element_ranges(class) {
                    if start == end {
                        write!(f, "{:?}", start)?;
                    } else {
                        write!(f, "{:?}-{:?}", start, end)?;
                    }
                }
                write!(f, "]")?;
            }
            write!(f, ")")
        }
    }
}

#[derive(Debug)]
pub struct ByteClassIter<'a> {
    classes: &'a ByteClasses,
    i: usize,
}

impl<'a> Iterator for ByteClassIter<'a> {
    type Item = Unit;

    fn next(&mut self) -> Option<Unit> {
        if self.i + 1 == self.classes.alphabet_len() {
            self.i += 1;
            Some(self.classes.eoi())
        } else if self.i < self.classes.alphabet_len() {
            let class = self.i as u8;
            self.i += 1;
            Some(Unit::u8(class))
        } else {
            None
        }
    }
}

#[cfg(feature = "std")]
#[derive(Debug)]
pub struct ByteClassRepresentatives<'a> {
    classes: &'a ByteClasses,
    byte: usize,
    last_class: Option<u8>,
}

#[cfg(feature = "std")]
impl<'a> Iterator for ByteClassRepresentatives<'a> {
    type Item = Unit;

    fn next(&mut self) -> Option<Unit> {
        while self.byte < 256 {
            let byte = self.byte as u8;
            let class = self.classes.get(byte);
            self.byte += 1;

            if self.last_class != Some(class) {
                self.last_class = Some(class);
                return Some(Unit::u8(byte));
            }
        }
        if self.byte == 256 {
            self.byte += 1;
            return Some(self.classes.eoi());
        }
        None
    }
}

#[derive(Debug)]
pub struct ByteClassElements<'a> {
    classes: &'a ByteClasses,
    class: Unit,
    byte: usize,
}

impl<'a> Iterator for ByteClassElements<'a> {
    type Item = Unit;

    fn next(&mut self) -> Option<Unit> {
        while self.byte < 256 {
            let byte = self.byte as u8;
            self.byte += 1;
            if self.class.as_u8() == Some(self.classes.get(byte)) {
                return Some(Unit::u8(byte));
            }
        }
        if self.byte < 257 {
            self.byte += 1;
            if self.class.is_eoi() {
                return Some(Unit::eoi(256));
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct ByteClassElementRanges<'a> {
    elements: ByteClassElements<'a>,
    range: Option<(Unit, Unit)>,
}

impl<'a> Iterator for ByteClassElementRanges<'a> {
    type Item = (Unit, Unit);

    fn next(&mut self) -> Option<(Unit, Unit)> {
        loop {
            let element = match self.elements.next() {
                None => return self.range.take(),
                Some(element) => element,
            };
            match self.range.take() {
                None => {
                    self.range = Some((element, element));
                }
                Some((start, end)) => {
                    if end.as_usize() + 1 != element.as_usize()
                        || element.is_eoi()
                    {
                        self.range = Some((element, element));
                        return Some((start, end));
                    }
                    self.range = Some((start, element));
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ByteClassSet(ByteSet);

impl ByteClassSet {
            #[cfg(feature = "std")]
    pub fn new() -> Self {
        ByteClassSet(ByteSet::empty())
    }

            #[cfg(feature = "std")]
    pub fn empty() -> Self {
        ByteClassSet(ByteSet::empty())
    }

            #[cfg(feature = "std")]
    pub fn set_range(&mut self, start: u8, end: u8) {
        debug_assert!(start <= end);
        if start > 0 {
            self.0.add(start - 1);
        }
        self.0.add(end);
    }

        #[cfg(feature = "std")]
    pub fn add_set(&mut self, set: &ByteSet) {
        for (start, end) in set.iter_ranges() {
            self.set_range(start, end);
        }
    }

                #[cfg(feature = "std")]
    pub fn byte_classes(&self) -> ByteClasses {
        let mut classes = ByteClasses::empty();
        let mut class = 0u8;
        let mut b = 0u8;
        loop {
            classes.set(b, class);
            if b == 255 {
                break;
            }
            if self.0.contains(b) {
                class = class.checked_add(1).unwrap();
            }
            b = b.checked_add(1).unwrap();
        }
        classes
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ByteSet {
    bits: BitSet,
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
struct BitSet([u128; 2]);

impl ByteSet {
        #[cfg(feature = "std")]
    pub fn empty() -> ByteSet {
        ByteSet { bits: BitSet([0; 2]) }
    }

                #[cfg(feature = "std")]
    pub fn add(&mut self, byte: u8) {
        let bucket = byte / 128;
        let bit = byte % 128;
        self.bits.0[bucket as usize] |= 1 << bit;
    }

        #[cfg(feature = "std")]
    pub fn add_all(&mut self, start: u8, end: u8) {
        for b in start..=end {
            self.add(b);
        }
    }

                #[cfg(feature = "std")]
    pub fn remove(&mut self, byte: u8) {
        let bucket = byte / 128;
        let bit = byte % 128;
        self.bits.0[bucket as usize] &= !(1 << bit);
    }

        #[cfg(feature = "std")]
    pub fn remove_all(&mut self, start: u8, end: u8) {
        for b in start..=end {
            self.remove(b);
        }
    }

        pub fn contains(&self, byte: u8) -> bool {
        let bucket = byte / 128;
        let bit = byte % 128;
        self.bits.0[bucket as usize] & (1 << bit) > 0
    }

            #[cfg(feature = "std")]
    pub fn contains_range(&self, start: u8, end: u8) -> bool {
        (start..=end).all(|b| self.contains(b))
    }

        #[cfg(feature = "std")]
    pub fn iter(&self) -> ByteSetIter {
        ByteSetIter { set: self, b: 0 }
    }

        #[cfg(feature = "std")]
    pub fn iter_ranges(&self) -> ByteSetRangeIter {
        ByteSetRangeIter { set: self, b: 0 }
    }

        #[cfg(feature = "std")]
    pub fn len(&self) -> usize {
        (self.bits.0[0].count_ones() + self.bits.0[1].count_ones()) as usize
    }

        #[cfg(feature = "std")]
    pub fn is_empty(&self) -> bool {
        self.bits.0 == [0, 0]
    }
}

impl core::fmt::Debug for BitSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut fmtd = f.debug_set();
        for b in (0..256).map(|b| b as u8) {
            if (ByteSet { bits: *self }).contains(b) {
                fmtd.entry(&b);
            }
        }
        fmtd.finish()
    }
}

#[derive(Debug)]
pub struct ByteSetIter<'a> {
    set: &'a ByteSet,
    b: usize,
}

impl<'a> Iterator for ByteSetIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        while self.b <= 255 {
            let b = self.b as u8;
            self.b += 1;
            if self.set.contains(b) {
                return Some(b);
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct ByteSetRangeIter<'a> {
    set: &'a ByteSet,
    b: usize,
}

impl<'a> Iterator for ByteSetRangeIter<'a> {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<(u8, u8)> {
        while self.b <= 255 {
            let start = self.b as u8;
            self.b += 1;
            if !self.set.contains(start) {
                continue;
            }

            let mut end = start;
            while self.b <= 255 && self.set.contains(self.b as u8) {
                end = self.b as u8;
                self.b += 1;
            }
            return Some((start, end));
        }
        None
    }
}
