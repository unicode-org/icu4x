// I've called the primary data structure in this module a "range trie." As far
// as I can tell, there is no prior art on a data structure like this, however,
// it's likely someone somewhere has built something like it. Searching for
// "range trie" turns up the paper "Range Tries for Scalable Address Lookup,"
// but it does not appear relevant.
//
// The range trie is just like a trie in that it is a special case of a
// deterministic finite state machine. It has states and each state has a set
// of transitions to other states. It is acyclic, and, like a normal trie,
// it makes no attempt to reuse common suffixes among its elements. The key
// difference between a normal trie and a range trie below is that a range trie
// operates on *contiguous sequences* of bytes instead of singleton bytes.
// One could say say that our alphabet is ranges of bytes instead of bytes
// themselves, except a key part of range trie construction is splitting ranges
// apart to ensure there is at most one transition that can be taken for any
// byte in a given state.
//
// I've tried to explain the details of how the range trie works below, so
// for now, we are left with trying to understand what problem we're trying to
// solve. Which is itself fairly involved!
//
// At the highest level, here's what we want to do. We want to convert a
// sequence of Unicode codepoints into a finite state machine whose transitions
// are over *bytes* and *not* Unicode codepoints. We want this because it makes
// said finite state machines much smaller and much faster to execute. As a
// simple example, consider a byte oriented automaton for all Unicode scalar
// values (0x00 through 0x10FFFF, not including surrogate codepoints):
//
//     [00-7F]
//     [C2-DF][80-BF]
//     [E0-E0][A0-BF][80-BF]
//     [E1-EC][80-BF][80-BF]
//     [ED-ED][80-9F][80-BF]
//     [EE-EF][80-BF][80-BF]
//     [F0-F0][90-BF][80-BF][80-BF]
//     [F1-F3][80-BF][80-BF][80-BF]
//     [F4-F4][80-8F][80-BF][80-BF]
//
// (These byte ranges are generated via the regex-syntax::utf8 module, which
// was based on Russ Cox's code in RE2, which was in turn based on Ken
// Thompson's implementation of the same idea in his Plan9 implementation of
// grep.)
//
// It should be fairly straight-forward to see how one could compile this into
// a DFA. The sequences are sorted and non-overlapping. Essentially, you could
// build a trie from this fairly easy. The problem comes when your initial
// range (in this case, 0x00-0x10FFFF) isn't so nice. For example, the class
// represented by '\w' contains only a tenth of the codepoints that
// 0x00-0x10FFFF contains, but if we were to write out the byte based ranges
// as we did above, the list would stretch to 892 entries! This turns into
// quite a large NFA with a few thousand states. Turning this beast into a DFA
// takes quite a bit of time. We are thus left with trying to trim down the
// number of states we produce as early as possible.
//
// One approach (used by RE2 and still by the regex crate, at time of writing)
// is to try to find common suffixes while building NFA states for the above
// and reuse them. This is very cheap to do and one can control precisely how
// much extra memory you want to use for the cache.
//
// Another approach, however, is to reuse an algorithm for constructing a
// *minimal* DFA from a sorted sequence of inputs. I don't want to go into
// the full details here, but I explain it in more depth in my blog post on
// FSTs[1]. Note that the algorithm was not invented by me, but was published
// in paper by Daciuk et al. in 2000 called "Incremental Construction of
// MinimalAcyclic Finite-State Automata." Like the suffix cache approach above,
// it is also possible to control the amount of extra memory one uses, although
// this usually comes with the cost of sacrificing true minimality. (But it's
// typically close enough with a reasonably sized cache of states.)
//
// The catch is that Daciuk's algorithm only works if you add your keys in
// lexicographic ascending order. In our case, since we're dealing with ranges,
// we also need the additional requirement that ranges are either equivalent
// or do not overlap at all. For example, if one were given the following byte
// ranges:
//
//     [BC-BF][80-BF]
//     [BC-BF][90-BF]
//
// Then Daciuk's algorithm would not work, since there is nothing to handle the
// fact that the ranges overlap. They would need to be split apart. Thankfully,
// Thompson's algorithm for producing byte ranges for Unicode codepoint ranges
// meets both of our requirements. (A proof for this eludes me, but it appears
// true.)
//
// ... however, we would also like to be able to compile UTF-8 automata in
// reverse. We want this because in order to find the starting location of a
// match using a DFA, we need to run a second DFA---a reversed version of the
// forward DFA---backwards to discover the match location. Unfortunately, if
// we reverse our byte sequences for 0x00-0x10FFFF, we get sequences that are
// can overlap, even if they are sorted:
//
//     [00-7F]
//     [80-BF][80-9F][ED-ED]
//     [80-BF][80-BF][80-8F][F4-F4]
//     [80-BF][80-BF][80-BF][F1-F3]
//     [80-BF][80-BF][90-BF][F0-F0]
//     [80-BF][80-BF][E1-EC]
//     [80-BF][80-BF][EE-EF]
//     [80-BF][A0-BF][E0-E0]
//     [80-BF][C2-DF]
//
// For example, '[80-BF][80-BF][EE-EF]' and '[80-BF][A0-BF][E0-E0]' have
// overlapping ranges between '[80-BF]' and '[A0-BF]'. Thus, there is no
// simple way to apply Daciuk's algorithm.
//
// And thus, the range trie was born. The range trie's only purpose is to take
// sequences of byte ranges like the ones above, collect them into a trie and
// then spit them in a sorted fashion with no overlapping ranges. For example,
// 0x00-0x10FFFF gets translated to:
//
//     [0-7F]
//     [80-BF][80-9F][80-8F][F1-F3]
//     [80-BF][80-9F][80-8F][F4]
//     [80-BF][80-9F][90-BF][F0]
//     [80-BF][80-9F][90-BF][F1-F3]
//     [80-BF][80-9F][E1-EC]
//     [80-BF][80-9F][ED]
//     [80-BF][80-9F][EE-EF]
//     [80-BF][A0-BF][80-8F][F1-F3]
//     [80-BF][A0-BF][80-8F][F4]
//     [80-BF][A0-BF][90-BF][F0]
//     [80-BF][A0-BF][90-BF][F1-F3]
//     [80-BF][A0-BF][E0]
//     [80-BF][A0-BF][E1-EC]
//     [80-BF][A0-BF][EE-EF]
//     [80-BF][C2-DF]
//
// We've thus satisfied our requirements for running Daciuk's algorithm. All
// sequences of ranges are sorted, and any corresponding ranges are either
// exactly equivalent or non-overlapping.
//
// In effect, a range trie is building a DFA from a sequence of arbitrary
// byte ranges. But it uses an algoritm custom tailored to its input, so it
// is not as costly as traditional DFA construction. While it is still quite
// a bit more costly than the forward's case (which only needs Daciuk's
// algorithm), it winds up saving a substantial amount of time if one is doing
// a full DFA powerset construction later by virtue of producing a much much
// smaller NFA.
//
// [1] - https://blog.burntsushi.net/transducers/
// [2] - https://www.mitpressjournals.org/doi/pdfplus/10.1162/089120100561601

use core::{cell::RefCell, fmt, mem, ops::RangeInclusive, u32};

use alloc::{format, string::String, vec, vec::Vec};

use regex_syntax::utf8::Utf8Range;

type StateID = u32;

const FINAL: StateID = 0;

const ROOT: StateID = 1;

#[derive(Clone)]
pub struct RangeTrie {
                states: Vec<State>,
                free: Vec<State>,
            iter_stack: RefCell<Vec<NextIter>>,
        iter_ranges: RefCell<Vec<Utf8Range>>,
            dupe_stack: Vec<NextDupe>,
            insert_stack: Vec<NextInsert>,
}

#[derive(Clone)]
struct State {
            transitions: Vec<Transition>,
}

#[derive(Clone)]
struct Transition {
        range: Utf8Range,
        next_id: StateID,
}

impl RangeTrie {
        pub fn new() -> RangeTrie {
        let mut trie = RangeTrie {
            states: vec![],
            free: vec![],
            iter_stack: RefCell::new(vec![]),
            iter_ranges: RefCell::new(vec![]),
            dupe_stack: vec![],
            insert_stack: vec![],
        };
        trie.clear();
        trie
    }

            pub fn clear(&mut self) {
        self.free.extend(self.states.drain(..));
        self.add_empty(); // final
        self.add_empty(); // root
    }

                pub fn iter<E, F: FnMut(&[Utf8Range]) -> Result<(), E>>(
        &self,
        mut f: F,
    ) -> Result<(), E> {
        let mut stack = self.iter_stack.borrow_mut();
        stack.clear();
        let mut ranges = self.iter_ranges.borrow_mut();
        ranges.clear();

        // We do iteration in a way that permits us to use a single buffer
        // for our keys. We iterate in a depth first fashion, while being
        // careful to expand our frontier as we move deeper in the trie.
        stack.push(NextIter { state_id: ROOT, tidx: 0 });
        while let Some(NextIter { mut state_id, mut tidx }) = stack.pop() {
            // This could be implemented more simply without an inner loop
            // here, but at the cost of more stack pushes.
            loop {
                let state = self.state(state_id);
                // If we've visited all transitions in this state, then pop
                // back to the parent state.
                if tidx >= state.transitions.len() {
                    ranges.pop();
                    break;
                }

                let t = &state.transitions[tidx];
                ranges.push(t.range);
                if t.next_id == FINAL {
                    f(&ranges)?;
                    ranges.pop();
                    tidx += 1;
                } else {
                    // Expand our frontier. Once we come back to this state
                    // via the stack, start in on the next transition.
                    stack.push(NextIter { state_id, tidx: tidx + 1 });
                    // Otherwise, move to the first transition of the next
                    // state.
                    state_id = t.next_id;
                    tidx = 0;
                }
            }
        }
        Ok(())
    }

                    pub fn insert(&mut self, ranges: &[Utf8Range]) {
        assert!(!ranges.is_empty());
        assert!(ranges.len() <= 4);

        let mut stack = mem::replace(&mut self.insert_stack, vec![]);
        stack.clear();

        stack.push(NextInsert::new(ROOT, ranges));
        while let Some(next) = stack.pop() {
            let (state_id, ranges) = (next.state_id(), next.ranges());
            assert!(!ranges.is_empty());

            let (mut new, rest) = (ranges[0], &ranges[1..]);

            // i corresponds to the position of the existing transition on
            // which we are operating. Typically, the result is to remove the
            // transition and replace it with two or more new transitions
            // corresponding to the partitions generated by splitting the
            // 'new' with the ith transition's range.
            let mut i = self.state(state_id).find(new);

            // In this case, there is no overlap *and* the new range is greater
            // than all existing ranges. So we can just add it to the end.
            if i == self.state(state_id).transitions.len() {
                let next_id = NextInsert::push(self, &mut stack, rest);
                self.add_transition(state_id, new, next_id);
                continue;
            }

            // The need for this loop is a bit subtle, buf basically, after
            // we've handled the partitions from our initial split, it's
            // possible that there will be a partition leftover that overlaps
            // with a subsequent transition. If so, then we have to repeat
            // the split process again with the leftovers and that subsequent
            // transition.
            'OUTER: loop {
                let old = self.state(state_id).transitions[i].clone();
                let split = match Split::new(old.range, new) {
                    Some(split) => split,
                    None => {
                        let next_id = NextInsert::push(self, &mut stack, rest);
                        self.add_transition_at(i, state_id, new, next_id);
                        continue;
                    }
                };
                let splits = split.as_slice();
                // If we only have one partition, then the ranges must be
                // equivalent. There's nothing to do here for this state, so
                // just move on to the next one.
                if splits.len() == 1 {
                    // ... but only if we have anything left to do.
                    if !rest.is_empty() {
                        stack.push(NextInsert::new(old.next_id, rest));
                    }
                    break;
                }
                // At this point, we know that 'split' is non-empty and there
                // must be some overlap AND that the two ranges are not
                // equivalent. Therefore, the existing range MUST be removed
                // and split up somehow. Instead of actually doing the removal
                // and then a subsequent insertion---with all the memory
                // shuffling that entails---we simply overwrite the transition
                // at position `i` for the first new transition we want to
                // insert. After that, we're forced to do expensive inserts.
                let mut first = true;
                let mut add_trans =
                    |trie: &mut RangeTrie, pos, from, range, to| {
                        if first {
                            trie.set_transition_at(pos, from, range, to);
                            first = false;
                        } else {
                            trie.add_transition_at(pos, from, range, to);
                        }
                    };
                for (j, &srange) in splits.iter().enumerate() {
                    match srange {
                        SplitRange::Old(r) => {
                            // Deep clone the state pointed to by the ith
                            // transition. This is always necessary since 'old'
                            // is always coupled with at least a 'both'
                            // partition. We don't want any new changes made
                            // via the 'both' partition to impact the part of
                            // the transition that doesn't overlap with the
                            // new range.
                            let dup_id = self.duplicate(old.next_id);
                            add_trans(self, i, state_id, r, dup_id);
                        }
                        SplitRange::New(r) => {
                            // This is a bit subtle, but if this happens to be
                            // the last partition in our split, it is possible
                            // that this overlaps with a subsequent transition.
                            // If it does, then we must repeat the whole
                            // splitting process over again with `r` and the
                            // subsequent transition.
                            {
                                let trans = &self.state(state_id).transitions;
                                if j + 1 == splits.len()
                                    && i < trans.len()
                                    && intersects(r, trans[i].range)
                                {
                                    new = r;
                                    continue 'OUTER;
                                }
                            }

                            // ... otherwise, setup exploration for a new
                            // empty state and add a brand new transition for
                            // this new range.
                            let next_id =
                                NextInsert::push(self, &mut stack, rest);
                            add_trans(self, i, state_id, r, next_id);
                        }
                        SplitRange::Both(r) => {
                            // Continue adding the remaining ranges on this
                            // path and update the transition with the new
                            // range.
                            if !rest.is_empty() {
                                stack.push(NextInsert::new(old.next_id, rest));
                            }
                            add_trans(self, i, state_id, r, old.next_id);
                        }
                    }
                    i += 1;
                }
                // If we've reached this point, then we know that there are
                // no subsequent transitions with any overlap. Therefore, we
                // can stop processing this range and move on to the next one.
                break;
            }
        }
        self.insert_stack = stack;
    }

    pub fn add_empty(&mut self) -> StateID {
        if self.states.len() as u64 > u32::MAX as u64 {
            // This generally should not happen since a range trie is only
            // ever used to compile a single sequence of Unicode scalar values.
            // If we ever got to this point, we would, at *minimum*, be using
            // 96GB in just the range trie alone.
            panic!("too many sequences added to range trie");
        }
        let id = self.states.len() as StateID;
        // If we have some free states available, then use them to avoid
        // more allocations.
        if let Some(mut state) = self.free.pop() {
            state.clear();
            self.states.push(state);
        } else {
            self.states.push(State { transitions: vec![] });
        }
        id
    }

                                                                    fn duplicate(&mut self, old_id: StateID) -> StateID {
        if old_id == FINAL {
            return FINAL;
        }

        let mut stack = mem::replace(&mut self.dupe_stack, vec![]);
        stack.clear();

        let new_id = self.add_empty();
        // old_id is the state we're cloning and new_id is the ID of the
        // duplicated state for old_id.
        stack.push(NextDupe { old_id, new_id });
        while let Some(NextDupe { old_id, new_id }) = stack.pop() {
            for i in 0..self.state(old_id).transitions.len() {
                let t = self.state(old_id).transitions[i].clone();
                if t.next_id == FINAL {
                    // All final states are the same, so there's no need to
                    // duplicate it.
                    self.add_transition(new_id, t.range, FINAL);
                    continue;
                }

                let new_child_id = self.add_empty();
                self.add_transition(new_id, t.range, new_child_id);
                stack.push(NextDupe {
                    old_id: t.next_id,
                    new_id: new_child_id,
                });
            }
        }
        self.dupe_stack = stack;
        new_id
    }

                    fn add_transition(
        &mut self,
        from_id: StateID,
        range: Utf8Range,
        next_id: StateID,
    ) {
        self.state_mut(from_id)
            .transitions
            .push(Transition { range, next_id });
    }

            fn add_transition_at(
        &mut self,
        i: usize,
        from_id: StateID,
        range: Utf8Range,
        next_id: StateID,
    ) {
        self.state_mut(from_id)
            .transitions
            .insert(i, Transition { range, next_id });
    }

        fn set_transition_at(
        &mut self,
        i: usize,
        from_id: StateID,
        range: Utf8Range,
        next_id: StateID,
    ) {
        self.state_mut(from_id).transitions[i] = Transition { range, next_id };
    }

        fn state(&self, id: StateID) -> &State {
        &self.states[id as usize]
    }

        fn state_mut(&mut self, id: StateID) -> &mut State {
        &mut self.states[id as usize]
    }
}

impl State {
                                                                fn find(&self, range: Utf8Range) -> usize {
                                                                                fn binary_search<T, F>(xs: &[T], mut pred: F) -> usize
        where
            F: FnMut(&T) -> bool,
        {
            let (mut left, mut right) = (0, xs.len());
            while left < right {
                // Overflow is impossible because xs.len() <= 256.
                let mid = (left + right) / 2;
                if pred(&xs[mid]) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            left
        }

        // Benchmarks suggest that binary search is just a bit faster than
        // straight linear search. Specifically when using the debug tool:
        //
        //   hyperfine "regex-cli debug nfa thompson --quiet --reverse '\w{90} ecurB'"
        binary_search(&self.transitions, |t| range.start <= t.range.end)
    }

        fn clear(&mut self) {
        self.transitions.clear();
    }
}

#[derive(Clone, Debug)]
struct NextDupe {
        old_id: StateID,
        new_id: StateID,
}

#[derive(Clone, Debug)]
struct NextIter {
    state_id: StateID,
    tidx: usize,
}

#[derive(Clone, Debug)]
struct NextInsert {
            state_id: StateID,
            ranges: [Utf8Range; 4],
        len: u8,
}

impl NextInsert {
                    fn new(state_id: StateID, ranges: &[Utf8Range]) -> NextInsert {
        let len = ranges.len();
        assert!(len > 0);
        assert!(len <= 4);

        let mut tmp = [Utf8Range { start: 0, end: 0 }; 4];
        tmp[..len].copy_from_slice(ranges);
        NextInsert { state_id, ranges: tmp, len: len as u8 }
    }

                    fn push(
        trie: &mut RangeTrie,
        stack: &mut Vec<NextInsert>,
        ranges: &[Utf8Range],
    ) -> StateID {
        if ranges.is_empty() {
            FINAL
        } else {
            let next_id = trie.add_empty();
            stack.push(NextInsert::new(next_id, ranges));
            next_id
        }
    }

        fn state_id(&self) -> StateID {
        self.state_id
    }

        fn ranges(&self) -> &[Utf8Range] {
        &self.ranges[..self.len as usize]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Split {
    partitions: [SplitRange; 3],
    len: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SplitRange {
    Old(Utf8Range),
    New(Utf8Range),
    Both(Utf8Range),
}

impl Split {
                fn new(o: Utf8Range, n: Utf8Range) -> Option<Split> {
        let range = |r: RangeInclusive<u8>| Utf8Range {
            start: *r.start(),
            end: *r.end(),
        };
        let old = |r| SplitRange::Old(range(r));
        let new = |r| SplitRange::New(range(r));
        let both = |r| SplitRange::Both(range(r));

        // Use same names as the comment above to make it easier to compare.
        let (a, b, x, y) = (o.start, o.end, n.start, n.end);

        if b < x || y < a {
            // case 1, case 2
            None
        } else if a == x && b == y {
            // case 3
            Some(Split::parts1(both(a..=b)))
        } else if a == x && b < y {
            // case 4
            Some(Split::parts2(both(a..=b), new(b + 1..=y)))
        } else if b == y && a > x {
            // case 5
            Some(Split::parts2(new(x..=a - 1), both(a..=b)))
        } else if x == a && y < b {
            // case 6
            Some(Split::parts2(both(x..=y), old(y + 1..=b)))
        } else if y == b && x > a {
            // case 7
            Some(Split::parts2(old(a..=x - 1), both(x..=y)))
        } else if a > x && b < y {
            // case 8
            Some(Split::parts3(new(x..=a - 1), both(a..=b), new(b + 1..=y)))
        } else if x > a && y < b {
            // case 9
            Some(Split::parts3(old(a..=x - 1), both(x..=y), old(y + 1..=b)))
        } else if b == x && a < y {
            // case 10
            Some(Split::parts3(old(a..=b - 1), both(b..=b), new(b + 1..=y)))
        } else if y == a && x < b {
            // case 11
            Some(Split::parts3(new(x..=y - 1), both(y..=y), old(y + 1..=b)))
        } else if b > x && b < y {
            // case 12
            Some(Split::parts3(old(a..=x - 1), both(x..=b), new(b + 1..=y)))
        } else if y > a && y < b {
            // case 13
            Some(Split::parts3(new(x..=a - 1), both(a..=y), old(y + 1..=b)))
        } else {
            unreachable!()
        }
    }

            fn parts1(r1: SplitRange) -> Split {
        // This value doesn't matter since it is never accessed.
        let nada = SplitRange::Old(Utf8Range { start: 0, end: 0 });
        Split { partitions: [r1, nada, nada], len: 1 }
    }

        fn parts2(r1: SplitRange, r2: SplitRange) -> Split {
        // This value doesn't matter since it is never accessed.
        let nada = SplitRange::Old(Utf8Range { start: 0, end: 0 });
        Split { partitions: [r1, r2, nada], len: 2 }
    }

        fn parts3(r1: SplitRange, r2: SplitRange, r3: SplitRange) -> Split {
        Split { partitions: [r1, r2, r3], len: 3 }
    }

        fn as_slice(&self) -> &[SplitRange] {
        &self.partitions[..self.len]
    }
}

impl fmt::Debug for RangeTrie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "")?;
        for (i, state) in self.states.iter().enumerate() {
            let status = if i == FINAL as usize { '*' } else { ' ' };
            writeln!(f, "{}{:06}: {:?}", status, i, state)?;
        }
        Ok(())
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rs = self
            .transitions
            .iter()
            .map(|t| format!("{:?}", t))
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", rs)
    }
}

impl fmt::Debug for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.range.start == self.range.end {
            write!(f, "{:02X} => {:02X}", self.range.start, self.next_id)
        } else {
            write!(
                f,
                "{:02X}-{:02X} => {:02X}",
                self.range.start, self.range.end, self.next_id
            )
        }
    }
}

fn intersects(r1: Utf8Range, r2: Utf8Range) -> bool {
    !(r1.end < r2.start || r2.end < r1.start)
}
