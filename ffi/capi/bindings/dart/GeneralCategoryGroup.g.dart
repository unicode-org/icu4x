// generated by diplomat-tool

part of 'lib.g.dart';

final class _GeneralCategoryGroupFfi extends ffi.Struct {
  @ffi.Uint32()
  external int mask;
}

/// A mask that is capable of representing groups of `General_Category` values.
///
/// See the [Rust documentation for `GeneralCategoryGroup`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html) for more information.
final class GeneralCategoryGroup {
  int mask;

  GeneralCategoryGroup({required this.mask});

  // This struct contains borrowed fields, so this takes in a list of
  // "edges" corresponding to where each lifetime's data may have been borrowed from
  // and passes it down to individual fields containing the borrow.
  // This method does not attempt to handle any dependencies between lifetimes, the caller
  // should handle this when constructing edge arrays.
  // ignore: unused_element
  GeneralCategoryGroup._fromFfi(_GeneralCategoryGroupFfi ffi) :
    mask = ffi.mask;

  // ignore: unused_element
  _GeneralCategoryGroupFfi _toFfi(ffi.Allocator temp) {
    final struct = ffi.Struct.create<_GeneralCategoryGroupFfi>();
    struct.mask = mask;
    return struct;
  }

  /// See the [Rust documentation for `contains`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#method.contains) for more information.
  bool contains(GeneralCategory val) {
    final temp = _FinalizedArena();
    final result = _icu4x_GeneralCategoryGroup_contains_mv1(_toFfi(temp.arena), val._ffi);
    return result;
  }

  /// See the [Rust documentation for `complement`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#method.complement) for more information.
  GeneralCategoryGroup complement() {
    final temp = _FinalizedArena();
    final result = _icu4x_GeneralCategoryGroup_complement_mv1(_toFfi(temp.arena));
    return GeneralCategoryGroup._fromFfi(result);
  }

  /// See the [Rust documentation for `all`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#method.all) for more information.
  static GeneralCategoryGroup all() {
    final result = _icu4x_GeneralCategoryGroup_all_mv1();
    return GeneralCategoryGroup._fromFfi(result);
  }

  /// See the [Rust documentation for `empty`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#method.empty) for more information.
  static GeneralCategoryGroup empty() {
    final result = _icu4x_GeneralCategoryGroup_empty_mv1();
    return GeneralCategoryGroup._fromFfi(result);
  }

  /// See the [Rust documentation for `union`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#method.union) for more information.
  GeneralCategoryGroup union(GeneralCategoryGroup other) {
    final temp = _FinalizedArena();
    final result = _icu4x_GeneralCategoryGroup_union_mv1(_toFfi(temp.arena), other._toFfi(temp.arena));
    return GeneralCategoryGroup._fromFfi(result);
  }

  /// See the [Rust documentation for `intersection`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#method.intersection) for more information.
  GeneralCategoryGroup intersection(GeneralCategoryGroup other) {
    final temp = _FinalizedArena();
    final result = _icu4x_GeneralCategoryGroup_intersection_mv1(_toFfi(temp.arena), other._toFfi(temp.arena));
    return GeneralCategoryGroup._fromFfi(result);
  }

  /// See the [Rust documentation for `CasedLetter`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.CasedLetter) for more information.
  static GeneralCategoryGroup casedLetter() {
    final result = _icu4x_GeneralCategoryGroup_cased_letter_mv1();
    return GeneralCategoryGroup._fromFfi(result);
  }

  /// See the [Rust documentation for `Letter`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Letter) for more information.
  static GeneralCategoryGroup letter() {
    final result = _icu4x_GeneralCategoryGroup_letter_mv1();
    return GeneralCategoryGroup._fromFfi(result);
  }

  /// See the [Rust documentation for `Mark`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Mark) for more information.
  static GeneralCategoryGroup mark() {
    final result = _icu4x_GeneralCategoryGroup_mark_mv1();
    return GeneralCategoryGroup._fromFfi(result);
  }

  /// See the [Rust documentation for `Number`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Number) for more information.
  static GeneralCategoryGroup number() {
    final result = _icu4x_GeneralCategoryGroup_number_mv1();
    return GeneralCategoryGroup._fromFfi(result);
  }

  /// See the [Rust documentation for `Other`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Other) for more information.
  static GeneralCategoryGroup separator() {
    final result = _icu4x_GeneralCategoryGroup_separator_mv1();
    return GeneralCategoryGroup._fromFfi(result);
  }

  /// See the [Rust documentation for `Letter`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Letter) for more information.
  static GeneralCategoryGroup other() {
    final result = _icu4x_GeneralCategoryGroup_other_mv1();
    return GeneralCategoryGroup._fromFfi(result);
  }

  /// See the [Rust documentation for `Punctuation`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Punctuation) for more information.
  static GeneralCategoryGroup punctuation() {
    final result = _icu4x_GeneralCategoryGroup_punctuation_mv1();
    return GeneralCategoryGroup._fromFfi(result);
  }

  /// See the [Rust documentation for `Symbol`](https://docs.rs/icu/latest/icu/properties/props/struct.GeneralCategoryGroup.html#associatedconstant.Symbol) for more information.
  static GeneralCategoryGroup symbol() {
    final result = _icu4x_GeneralCategoryGroup_symbol_mv1();
    return GeneralCategoryGroup._fromFfi(result);
  }

  @override
  bool operator ==(Object other) =>
      other is GeneralCategoryGroup &&
      other.mask == mask;

  @override
  int get hashCode => Object.hashAll([
        mask,
      ]);
}

@meta.RecordUse()
@ffi.Native<ffi.Bool Function(_GeneralCategoryGroupFfi, ffi.Int32)>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_contains_mv1')
// ignore: non_constant_identifier_names
external bool _icu4x_GeneralCategoryGroup_contains_mv1(_GeneralCategoryGroupFfi self, int val);

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function(_GeneralCategoryGroupFfi)>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_complement_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_complement_mv1(_GeneralCategoryGroupFfi self);

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function()>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_all_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_all_mv1();

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function()>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_empty_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_empty_mv1();

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function(_GeneralCategoryGroupFfi, _GeneralCategoryGroupFfi)>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_union_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_union_mv1(_GeneralCategoryGroupFfi self, _GeneralCategoryGroupFfi other);

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function(_GeneralCategoryGroupFfi, _GeneralCategoryGroupFfi)>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_intersection_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_intersection_mv1(_GeneralCategoryGroupFfi self, _GeneralCategoryGroupFfi other);

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function()>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_cased_letter_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_cased_letter_mv1();

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function()>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_letter_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_letter_mv1();

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function()>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_mark_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_mark_mv1();

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function()>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_number_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_number_mv1();

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function()>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_separator_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_separator_mv1();

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function()>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_other_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_other_mv1();

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function()>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_punctuation_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_punctuation_mv1();

@meta.RecordUse()
@ffi.Native<_GeneralCategoryGroupFfi Function()>(isLeaf: true, symbol: 'icu4x_GeneralCategoryGroup_symbol_mv1')
// ignore: non_constant_identifier_names
external _GeneralCategoryGroupFfi _icu4x_GeneralCategoryGroup_symbol_mv1();