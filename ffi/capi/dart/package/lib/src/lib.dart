// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:typed_data';
import 'package:ffi/ffi.dart' as ffi2;

late final ffi.Pointer<T> Function<T extends ffi.NativeType>(String) _capi;
void init(String path) => _capi = ffi.DynamicLibrary.open(path).lookup;

final _callocFree = Finalizer(ffi2.calloc.free);

/// An iterator over code point ranges, produced by `ICU4XCodePointSetData` or
/// one of the `ICU4XCodePointMapData` types
class CodePointRangeIterator implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  CodePointRangeIterator._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('CodePointRangeIterator_destroy'));

  /// Advance the iterator by one and return the next range.
  ///
  /// If the iterator is out of items, `done` will be true
  CodePointRangeIteratorResult next() {
    final result = _nextFfi(_underlying);
    return CodePointRangeIteratorResult._(result);
  }

  static final _nextFfi = _capi<
          ffi.NativeFunction<
              _CodePointRangeIteratorResultFfi Function(
                  ffi.Pointer<ffi.Opaque>)>>('CodePointRangeIterator_next')
      .asFunction<
          _CodePointRangeIteratorResultFfi Function(
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// Result of a single iteration of [`CodePointRangeIterator`].
/// Logically can be considered to be an `Option<RangeInclusive<u32>>`,
///
/// `start` and `end` represent an inclusive range of code points [start, end],
/// and `done` will be true if the iterator has already finished. The last contentful
/// iteration will NOT produce a range done=true, in other words `start` and `end` are useful
/// values if and only if `done=false`.
class _CodePointRangeIteratorResultFfi extends ffi.Struct {
  @ffi.Uint32()
  external int start;
  @ffi.Uint32()
  external int end;
  @ffi.Bool()
  external bool done;
}

class CodePointRangeIteratorResult {
  final _CodePointRangeIteratorResultFfi _underlying;

  // ignore: unused_element
  CodePointRangeIteratorResult._(this._underlying);

  factory CodePointRangeIteratorResult() {
    final pointer = ffi2.calloc<_CodePointRangeIteratorResultFfi>();
    final result = CodePointRangeIteratorResult._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  int get start => _underlying.start;
  set start(int start) {
    _underlying.start = start;
  }

  int get end => _underlying.end;
  set end(int end) {
    _underlying.end = end;
  }

  bool get done => _underlying.done;
  set done(bool done) {
    _underlying.done = done;
  }

  @override
  bool operator ==(Object other) =>
      other is CodePointRangeIteratorResult &&
      other._underlying.start == _underlying.start &&
      other._underlying.end == _underlying.end &&
      other._underlying.done == _underlying.done;

  @override
  int get hashCode => Object.hashAll([
        _underlying.start,
        _underlying.end,
        _underlying.done,
      ]);
}

/// The various calendar types currently supported by [`ICU4XCalendar`]
///
/// See the [Rust documentation for `AnyCalendarKind`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendarKind.html) for more information.
enum ICU4XAnyCalendarKind {
  /// The kind of an Iso calendar
  iso.__(0),

  /// The kind of a Gregorian calendar
  gregorian.__(1),

  /// The kind of a Buddhist calendar
  buddhist.__(2),

  /// The kind of a Japanese calendar with modern eras
  japanese.__(3),

  /// The kind of a Japanese calendar with modern and historic eras
  japaneseExtended.__(4),

  /// The kind of an Ethiopian calendar, with Amete Mihret era
  ethiopian.__(5),

  /// The kind of an Ethiopian calendar, with Amete Alem era
  ethiopianAmeteAlem.__(6),

  /// The kind of a Indian calendar
  indian.__(7),

  /// The kind of a Coptic calendar
  coptic.__(8),

  /// The kind of a Dangi calendar
  dangi.__(9),

  /// The kind of a Chinese calendar
  chinese.__(10),

  /// The kind of a Hebrew calendar
  hebrew.__(11),

  /// The kind of a Islamic civil calendar
  islamicCivil.__(12),

  /// The kind of a Islamic observational calendar
  islamicObservational.__(13),

  /// The kind of a Islamic tabular calendar
  islamicTabular.__(14),

  /// The kind of a Islamic Umm al-Qura calendar
  islamicUmmAlQura.__(15),

  /// The kind of a Persian calendar
  persian.__(16),

  /// The kind of a Roc calendar
  roc.__(17);

  const ICU4XAnyCalendarKind.__(this._id);

  // ignore: unused_element
  factory ICU4XAnyCalendarKind._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;

  /// Read the calendar type off of the -u-ca- extension on a locale.
  ///
  /// Errors if there is no calendar on the locale or if the locale's calendar
  /// is not known or supported.
  ///
  /// See the [Rust documentation for `get_for_locale`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendarKind.html#method.get_for_locale) for more information.
  factory ICU4XAnyCalendarKind.getForLocale(ICU4XLocale locale) {
    final result = _getForLocaleFfi(locale._underlying);
    return result.isOk
        ? ICU4XAnyCalendarKind._(result.union.ok)
        : throw VoidError();
  }
  static final _getForLocaleFfi = _capi<
              ffi.NativeFunction<
                  _ResultUint32Void Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XAnyCalendarKind_get_for_locale')
      .asFunction<_ResultUint32Void Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Obtain the calendar type given a BCP-47 -u-ca- extension string.
  ///
  /// Errors if the calendar is not known or supported.
  ///
  /// See the [Rust documentation for `get_for_bcp47_value`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendarKind.html#method.get_for_bcp47_value) for more information.
  factory ICU4XAnyCalendarKind.getForBcp47(String s) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final result = _getForBcp47Ffi(sSlice._bytes, sSlice._length);
    alloc.releaseAll();
    return result.isOk
        ? ICU4XAnyCalendarKind._(result.union.ok)
        : throw VoidError();
  }
  static final _getForBcp47Ffi = _capi<
          ffi.NativeFunction<
              _ResultUint32Void Function(ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XAnyCalendarKind_get_for_bcp47')
      .asFunction<_ResultUint32Void Function(ffi.Pointer<ffi2.Utf8>, int)>(
          isLeaf: true);

  /// Obtain the string suitable for use in the -u-ca- extension in a BCP47 locale.
  ///
  /// See the [Rust documentation for `as_bcp47_string`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendarKind.html#method.as_bcp47_string) for more information.
  String get bcp47 {
    final writeable = _Writeable();
    final result = _bcp47Ffi(_id, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _bcp47Ffi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Uint32,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XAnyCalendarKind_bcp47')
      .asFunction<_ResultVoidUint32 Function(int, ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
}

/// An object capable of mapping from a BCP-47 time zone ID to an IANA ID.
///
/// See the [Rust documentation for `IanaBcp47RoundTripMapper`](https://docs.rs/icu/latest/icu/timezone/struct.IanaBcp47RoundTripMapper.html) for more information.
class ICU4XBcp47ToIanaMapper implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XBcp47ToIanaMapper._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XBcp47ToIanaMapper_destroy'));

  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/timezone/struct.IanaBcp47RoundTripMapper.html#method.new) for more information.
  factory ICU4XBcp47ToIanaMapper(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XBcp47ToIanaMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XBcp47ToIanaMapper_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Writes out the canonical IANA time zone ID corresponding to the given BCP-47 ID.
  ///
  /// See the [Rust documentation for `bcp47_to_iana`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.IanaBcp47RoundTripMapper.html#method.bcp47_to_iana) for more information.
  String get(String value) {
    final alloc = ffi2.Arena();
    final valueSlice = _SliceFfi2Utf8._fromDart(value, alloc);

    final writeable = _Writeable();
    final result = _getFfi(_underlying, valueSlice._bytes, valueSlice._length,
        writeable._underlying);
    alloc.releaseAll();
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _getFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XBcp47ToIanaMapper_get')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An ICU4X Bidi object, containing loaded bidi data
///
/// See the [Rust documentation for `BidiClassAdapter`](https://docs.rs/icu/latest/icu/properties/bidi/struct.BidiClassAdapter.html) for more information.
class ICU4XBidi implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XBidi._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XBidi_destroy'));

  /// Creates a new [`ICU4XBidi`] from locale data.
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/properties/bidi/struct.BidiClassAdapter.html#method.new) for more information.
  factory ICU4XBidi(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XBidi._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XBidi_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Use the data loaded in this object to process a string and calculate bidi information
  ///
  /// Takes in a Level for the default level, if it is an invalid value it will default to LTR
  ///
  /// See the [Rust documentation for `new_with_data_source`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.BidiInfo.html#method.new_with_data_source) for more information.
  ICU4XBidiInfo forText(String text, int defaultLevel) {
    final alloc = ffi2.Arena();
    final textSlice = _SliceFfi2Utf8._fromDart(text, alloc);

    final result = _forTextFfi(
        _underlying, textSlice._bytes, textSlice._length, defaultLevel);
    alloc.releaseAll();
    return ICU4XBidiInfo._(result);
  }

  static final _forTextFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Uint8)>>('ICU4XBidi_for_text')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>, int, int)>(isLeaf: true);

  /// Utility function for producing reorderings given a list of levels
  ///
  /// Produces a map saying which visual index maps to which source index.
  ///
  /// The levels array must not have values greater than 126 (this is the
  /// Bidi maximum explicit depth plus one).
  /// Failure to follow this invariant may lead to incorrect results,
  /// but is still safe.
  ///
  /// See the [Rust documentation for `reorder_visual`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.BidiInfo.html#method.reorder_visual) for more information.
  ICU4XReorderedIndexMap reorderVisual(Uint8List levels) {
    final alloc = ffi2.Arena();
    final levelsSlice = _SliceFfiUint8._fromDart(levels, alloc);

    final result =
        _reorderVisualFfi(_underlying, levelsSlice._bytes, levelsSlice._length);
    alloc.releaseAll();
    return ICU4XReorderedIndexMap._(result);
  }

  static final _reorderVisualFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Uint8>,
                  ffi.Size)>>('ICU4XBidi_reorder_visual')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);

  /// Check if a Level returned by level_at is an RTL level.
  ///
  /// Invalid levels (numbers greater than 125) will be assumed LTR
  ///
  /// See the [Rust documentation for `is_rtl`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.Level.html#method.is_rtl) for more information.
  static bool levelIsRtl(int level) {
    final result = _levelIsRtlFfi(level);
    return result;
  }

  static final _levelIsRtlFfi =
      _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Uint8)>>(
              'ICU4XBidi_level_is_rtl')
          .asFunction<bool Function(int)>(isLeaf: true);

  /// Check if a Level returned by level_at is an LTR level.
  ///
  /// Invalid levels (numbers greater than 125) will be assumed LTR
  ///
  /// See the [Rust documentation for `is_ltr`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.Level.html#method.is_ltr) for more information.
  static bool levelIsLtr(int level) {
    final result = _levelIsLtrFfi(level);
    return result;
  }

  static final _levelIsLtrFfi =
      _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Uint8)>>(
              'ICU4XBidi_level_is_ltr')
          .asFunction<bool Function(int)>(isLeaf: true);

  /// Get a basic RTL Level value
  ///
  /// See the [Rust documentation for `rtl`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.Level.html#method.rtl) for more information.
  static final int levelRtl =
      _capi<ffi.NativeFunction<ffi.Uint8 Function()>>('ICU4XBidi_level_rtl')
          .asFunction<int Function()>(isLeaf: true)();

  /// Get a simple LTR Level value
  ///
  /// See the [Rust documentation for `ltr`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.Level.html#method.ltr) for more information.
  static final int levelLtr =
      _capi<ffi.NativeFunction<ffi.Uint8 Function()>>('ICU4XBidi_level_ltr')
          .asFunction<int Function()>(isLeaf: true)();
}

enum ICU4XBidiDirection {
  ltr.__(0),
  rtl.__(1),
  mixed.__(2);

  const ICU4XBidiDirection.__(this._id);

  // ignore: unused_element
  factory ICU4XBidiDirection._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// An object containing bidi information for a given string, produced by `for_text()` on `ICU4XBidi`
///
/// See the [Rust documentation for `BidiInfo`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.BidiInfo.html) for more information.
class ICU4XBidiInfo implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XBidiInfo._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XBidiInfo_destroy'));

  /// The number of paragraphs contained here
  int get paragraphCount {
    final result = _paragraphCountFfi(_underlying);
    return result;
  }

  static final _paragraphCountFfi =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XBidiInfo_paragraph_count')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Get the nth paragraph, returning `None` if out of bounds
  ICU4XBidiParagraph? paragraphAt(int n) {
    final result = _paragraphAtFfi(_underlying, n);
    return result.address == 0 ? null : ICU4XBidiParagraph._(result);
  }

  static final _paragraphAtFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Size)>>('ICU4XBidiInfo_paragraph_at')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// The number of bytes in this full text
  int get size {
    final result = _sizeFfi(_underlying);
    return result;
  }

  static final _sizeFfi =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XBidiInfo_size')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Get the BIDI level at a particular byte index in the full text.
  /// This integer is conceptually a `unicode_bidi::Level`,
  /// and can be further inspected using the static methods on ICU4XBidi.
  ///
  /// Returns 0 (equivalent to `Level::ltr()`) on error
  int levelAt(int pos) {
    final result = _levelAtFfi(_underlying, pos);
    return result;
  }

  static final _levelAtFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint8 Function(
                  ffi.Pointer<ffi.Opaque>, ffi.Size)>>('ICU4XBidiInfo_level_at')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);
}

/// Bidi information for a single processed paragraph
class ICU4XBidiParagraph implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XBidiParagraph._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XBidiParagraph_destroy'));

  /// Given a paragraph index `n` within the surrounding text, this sets this
  /// object to the paragraph at that index. Returns `ICU4XError::OutOfBoundsError` when out of bounds.
  ///
  /// This is equivalent to calling `paragraph_at()` on `ICU4XBidiInfo` but doesn't
  /// create a new object
  void setParagraphInText(int n) {
    final result = _setParagraphInTextFfi(_underlying, n);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _setParagraphInTextFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Size)>>('ICU4XBidiParagraph_set_paragraph_in_text')
      .asFunction<_ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>, int)>(
          isLeaf: true);

  /// The primary direction of this paragraph
  ///
  /// See the [Rust documentation for `level_at`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.Paragraph.html#method.level_at) for more information.
  ICU4XBidiDirection get direction {
    final result = _directionFfi(_underlying);
    return ICU4XBidiDirection._(result);
  }

  static final _directionFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XBidiParagraph_direction')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// The number of bytes in this paragraph
  ///
  /// See the [Rust documentation for `len`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.ParagraphInfo.html#method.len) for more information.
  int get size {
    final result = _sizeFfi(_underlying);
    return result;
  }

  static final _sizeFfi =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XBidiParagraph_size')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// The start index of this paragraph within the source text
  int get rangeStart {
    final result = _rangeStartFfi(_underlying);
    return result;
  }

  static final _rangeStartFfi =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XBidiParagraph_range_start')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// The end index of this paragraph within the source text
  int get rangeEnd {
    final result = _rangeEndFfi(_underlying);
    return result;
  }

  static final _rangeEndFfi =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XBidiParagraph_range_end')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Reorder a line based on display order. The ranges are specified relative to the source text and must be contained
  /// within this paragraph's range.
  ///
  /// See the [Rust documentation for `level_at`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.Paragraph.html#method.level_at) for more information.
  String reorderLine(int rangeStart, int rangeEnd) {
    final writeable = _Writeable();
    final result = _reorderLineFfi(
        _underlying, rangeStart, rangeEnd, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _reorderLineFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Size,
                  ffi.Size,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XBidiParagraph_reorder_line')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>, int, int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Get the BIDI level at a particular byte index in this paragraph.
  /// This integer is conceptually a `unicode_bidi::Level`,
  /// and can be further inspected using the static methods on ICU4XBidi.
  ///
  /// Returns 0 (equivalent to `Level::ltr()`) on error
  ///
  /// See the [Rust documentation for `level_at`](https://docs.rs/unicode_bidi/latest/unicode_bidi/struct.Paragraph.html#method.level_at) for more information.
  int levelAt(int pos) {
    final result = _levelAtFfi(_underlying, pos);
    return result;
  }

  static final _levelAtFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Size)>>('ICU4XBidiParagraph_level_at')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);
}

/// See the [Rust documentation for `AnyCalendar`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html) for more information.
class ICU4XCalendar implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XCalendar._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XCalendar_destroy'));

  /// Creates a new [`ICU4XCalendar`] from the specified date and time.
  ///
  /// See the [Rust documentation for `new_for_locale`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.new_for_locale) for more information.
  factory ICU4XCalendar.forLocale(
      ICU4XDataProvider provider, ICU4XLocale locale) {
    final result =
        _createForLocaleFfi(provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XCalendar._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createForLocaleFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCalendar_create_for_locale')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Creates a new [`ICU4XCalendar`] from the specified date and time.
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.new) for more information.
  factory ICU4XCalendar.forKind(
      ICU4XDataProvider provider, ICU4XAnyCalendarKind kind) {
    final result = _createForKindFfi(provider._underlying, kind._id);
    return result.isOk
        ? ICU4XCalendar._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createForKindFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCalendar_create_for_kind')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>, int)>(
          isLeaf: true);

  /// Returns the kind of this calendar
  ///
  /// See the [Rust documentation for `kind`](https://docs.rs/icu/latest/icu/calendar/enum.AnyCalendar.html#method.kind) for more information.
  ICU4XAnyCalendarKind get kind {
    final result = _kindFfi(_underlying);
    return ICU4XAnyCalendarKind._(result);
  }

  static final _kindFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XCalendar_kind')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// Lookup of the Canonical_Combining_Class Unicode property
///
/// See the [Rust documentation for `CanonicalCombiningClassMap`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalCombiningClassMap.html) for more information.
class ICU4XCanonicalCombiningClassMap implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XCanonicalCombiningClassMap._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XCanonicalCombiningClassMap_destroy'));

  /// Construct a new ICU4XCanonicalCombiningClassMap instance for NFC
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalCombiningClassMap.html#method.new) for more information.
  factory ICU4XCanonicalCombiningClassMap(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XCanonicalCombiningClassMap._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCanonicalCombiningClassMap_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `get`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalCombiningClassMap.html#method.get) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/properties/properties/struct.CanonicalCombiningClass.html)
  int get(int ch) {
    final result = _getFfi(_underlying, ch);
    return result;
  }

  static final _getFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCanonicalCombiningClassMap_get')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `get32`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalCombiningClassMap.html#method.get32) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/properties/properties/struct.CanonicalCombiningClass.html)
  int get32(int ch) {
    final result = _get32Ffi(_underlying, ch);
    return result;
  }

  static final _get32Ffi = _capi<
          ffi.NativeFunction<
              ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCanonicalCombiningClassMap_get32')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);
}

/// The raw canonical composition operation.
///
/// Callers should generally use ICU4XComposingNormalizer unless they specifically need raw composition operations
///
/// See the [Rust documentation for `CanonicalComposition`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html) for more information.
class ICU4XCanonicalComposition implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XCanonicalComposition._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XCanonicalComposition_destroy'));

  /// Construct a new ICU4XCanonicalComposition instance for NFC
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html#method.new) for more information.
  factory ICU4XCanonicalComposition(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XCanonicalComposition._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCanonicalComposition_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Performs canonical composition (including Hangul) on a pair of characters
  /// or returns NUL if these characters don’t compose. Composition exclusions are taken into account.
  ///
  /// See the [Rust documentation for `compose`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalComposition.html#method.compose) for more information.
  int compose(int starter, int second) {
    final result = _composeFfi(_underlying, starter, second);
    return result;
  }

  static final _composeFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32,
                  ffi.Uint32)>>('ICU4XCanonicalComposition_compose')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int, int)>(
          isLeaf: true);
}

/// The raw (non-recursive) canonical decomposition operation.
///
/// Callers should generally use ICU4XDecomposingNormalizer unless they specifically need raw composition operations
///
/// See the [Rust documentation for `CanonicalDecomposition`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalDecomposition.html) for more information.
class ICU4XCanonicalDecomposition implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XCanonicalDecomposition._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XCanonicalDecomposition_destroy'));

  /// Construct a new ICU4XCanonicalDecomposition instance for NFC
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalDecomposition.html#method.new) for more information.
  factory ICU4XCanonicalDecomposition(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XCanonicalDecomposition._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCanonicalDecomposition_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Performs non-recursive canonical decomposition (including for Hangul).
  ///
  /// See the [Rust documentation for `decompose`](https://docs.rs/icu/latest/icu/normalizer/properties/struct.CanonicalDecomposition.html#method.decompose) for more information.
  ICU4XDecomposed decompose(int c) {
    final result = _decomposeFfi(_underlying, c);
    return ICU4XDecomposed._(result);
  }

  static final _decomposeFfi = _capi<
          ffi.NativeFunction<
              _ICU4XDecomposedFfi Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCanonicalDecomposition_decompose')
      .asFunction<_ICU4XDecomposedFfi Function(ffi.Pointer<ffi.Opaque>, int)>(
          isLeaf: true);
}

/// See the [Rust documentation for `CaseMapCloser`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html) for more information.
class ICU4XCaseMapCloser implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XCaseMapCloser._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XCaseMapCloser_destroy'));

  /// Construct a new ICU4XCaseMapper instance
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.new) for more information.
  factory ICU4XCaseMapCloser(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XCaseMapCloser._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCaseMapCloser_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Adds all simple case mappings and the full case folding for `c` to `builder`.
  /// Also adds special case closure mappings.
  ///
  /// See the [Rust documentation for `add_case_closure_to`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.add_case_closure_to) for more information.
  void addCaseClosureTo(int c, ICU4XCodePointSetBuilder builder) {
    _addCaseClosureToFfi(_underlying, c, builder._underlying);
  }

  static final _addCaseClosureToFfi = _capi<
              ffi.NativeFunction<
                  ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCaseMapCloser_add_case_closure_to')
      .asFunction<
          void Function(ffi.Pointer<ffi.Opaque>, int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Finds all characters and strings which may casemap to `s` as their full case folding string
  /// and adds them to the set.
  ///
  /// Returns true if the string was found
  ///
  /// See the [Rust documentation for `add_string_case_closure_to`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapCloser.html#method.add_string_case_closure_to) for more information.
  bool addStringCaseClosureTo(String s, ICU4XCodePointSetBuilder builder) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final result = _addStringCaseClosureToFfi(
        _underlying, sSlice._bytes, sSlice._length, builder._underlying);
    alloc.releaseAll();
    return result;
  }

  static final _addStringCaseClosureToFfi = _capi<
              ffi.NativeFunction<
                  ffi.Bool Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi2.Utf8>,
                      ffi.Size,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCaseMapCloser_add_string_case_closure_to')
      .asFunction<
          bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>, int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `CaseMapper`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html) for more information.
class ICU4XCaseMapper implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XCaseMapper._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XCaseMapper_destroy'));

  /// Construct a new ICU4XCaseMapper instance
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.new) for more information.
  factory ICU4XCaseMapper(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XCaseMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCaseMapper_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Returns the full lowercase mapping of the given string
  ///
  /// See the [Rust documentation for `lowercase`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.lowercase) for more information.
  String lowercase(String s, ICU4XLocale locale) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final writeable = _Writeable();
    final result = _lowercaseFfi(_underlying, sSlice._bytes, sSlice._length,
        locale._underlying, writeable._underlying);
    alloc.releaseAll();
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _lowercaseFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCaseMapper_lowercase')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the full uppercase mapping of the given string
  ///
  /// See the [Rust documentation for `uppercase`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.uppercase) for more information.
  String uppercase(String s, ICU4XLocale locale) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final writeable = _Writeable();
    final result = _uppercaseFfi(_underlying, sSlice._bytes, sSlice._length,
        locale._underlying, writeable._underlying);
    alloc.releaseAll();
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _uppercaseFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCaseMapper_uppercase')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the full titlecase mapping of the given string, performing head adjustment without
  /// loading additional data.
  /// (if head adjustment is enabled in the options)
  ///
  /// The `v1` refers to the version of the options struct, which may change as we add more options
  ///
  /// See the [Rust documentation for `titlecase_segment_with_only_case_data`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.titlecase_segment_with_only_case_data) for more information.
  String titlecaseSegmentWithOnlyCaseDataV1(
      String s, ICU4XLocale locale, ICU4XTitlecaseOptionsV1 options) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final writeable = _Writeable();
    final result = _titlecaseSegmentWithOnlyCaseDataV1Ffi(
        _underlying,
        sSlice._bytes,
        sSlice._length,
        locale._underlying,
        options._underlying,
        writeable._underlying);
    alloc.releaseAll();
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _titlecaseSegmentWithOnlyCaseDataV1Ffi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi2.Utf8>,
                      ffi.Size,
                      ffi.Pointer<ffi.Opaque>,
                      _ICU4XTitlecaseOptionsV1Ffi,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCaseMapper_titlecase_segment_with_only_case_data_v1')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi.Opaque>,
              _ICU4XTitlecaseOptionsV1Ffi,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Case-folds the characters in the given string
  ///
  /// See the [Rust documentation for `fold`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.fold) for more information.
  String fold(String s) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final writeable = _Writeable();
    final result = _foldFfi(
        _underlying, sSlice._bytes, sSlice._length, writeable._underlying);
    alloc.releaseAll();
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _foldFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCaseMapper_fold')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Case-folds the characters in the given string
  /// using Turkic (T) mappings for dotted/dotless I.
  ///
  /// See the [Rust documentation for `fold_turkic`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.fold_turkic) for more information.
  String foldTurkic(String s) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final writeable = _Writeable();
    final result = _foldTurkicFfi(
        _underlying, sSlice._bytes, sSlice._length, writeable._underlying);
    alloc.releaseAll();
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _foldTurkicFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCaseMapper_fold_turkic')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Adds all simple case mappings and the full case folding for `c` to `builder`.
  /// Also adds special case closure mappings.
  ///
  /// In other words, this adds all characters that this casemaps to, as
  /// well as all characters that may casemap to this one.
  ///
  /// Note that since ICU4XCodePointSetBuilder does not contain strings, this will
  /// ignore string mappings.
  ///
  /// Identical to the similarly named method on `ICU4XCaseMapCloser`, use that if you
  /// plan on using string case closure mappings too.
  ///
  /// See the [Rust documentation for `add_case_closure_to`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.add_case_closure_to) for more information.
  void addCaseClosureTo(int c, ICU4XCodePointSetBuilder builder) {
    _addCaseClosureToFfi(_underlying, c, builder._underlying);
  }

  static final _addCaseClosureToFfi = _capi<
              ffi.NativeFunction<
                  ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCaseMapper_add_case_closure_to')
      .asFunction<
          void Function(ffi.Pointer<ffi.Opaque>, int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the simple lowercase mapping of the given character.
  ///
  /// This function only implements simple and common mappings.
  /// Full mappings, which can map one char to a string, are not included.
  /// For full mappings, use `ICU4XCaseMapper::lowercase`.
  ///
  /// See the [Rust documentation for `simple_lowercase`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_lowercase) for more information.
  int simpleLowercase(int ch) {
    final result = _simpleLowercaseFfi(_underlying, ch);
    return result;
  }

  static final _simpleLowercaseFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCaseMapper_simple_lowercase')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Returns the simple uppercase mapping of the given character.
  ///
  /// This function only implements simple and common mappings.
  /// Full mappings, which can map one char to a string, are not included.
  /// For full mappings, use `ICU4XCaseMapper::uppercase`.
  ///
  /// See the [Rust documentation for `simple_uppercase`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_uppercase) for more information.
  int simpleUppercase(int ch) {
    final result = _simpleUppercaseFfi(_underlying, ch);
    return result;
  }

  static final _simpleUppercaseFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCaseMapper_simple_uppercase')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Returns the simple titlecase mapping of the given character.
  ///
  /// This function only implements simple and common mappings.
  /// Full mappings, which can map one char to a string, are not included.
  /// For full mappings, use `ICU4XCaseMapper::titlecase_segment`.
  ///
  /// See the [Rust documentation for `simple_titlecase`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_titlecase) for more information.
  int simpleTitlecase(int ch) {
    final result = _simpleTitlecaseFfi(_underlying, ch);
    return result;
  }

  static final _simpleTitlecaseFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCaseMapper_simple_titlecase')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Returns the simple casefolding of the given character.
  ///
  /// This function only implements simple folding.
  /// For full folding, use `ICU4XCaseMapper::fold`.
  ///
  /// See the [Rust documentation for `simple_fold`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_fold) for more information.
  int simpleFold(int ch) {
    final result = _simpleFoldFfi(_underlying, ch);
    return result;
  }

  static final _simpleFoldFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCaseMapper_simple_fold')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Returns the simple casefolding of the given character in the Turkic locale
  ///
  /// This function only implements simple folding.
  /// For full folding, use `ICU4XCaseMapper::fold_turkic`.
  ///
  /// See the [Rust documentation for `simple_fold_turkic`](https://docs.rs/icu/latest/icu/casemap/struct.CaseMapper.html#method.simple_fold_turkic) for more information.
  int simpleFoldTurkic(int ch) {
    final result = _simpleFoldTurkicFfi(_underlying, ch);
    return result;
  }

  static final _simpleFoldTurkicFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCaseMapper_simple_fold_turkic')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);
}

/// An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.
///
/// For properties whose values fit into 16 bits.
///
/// See the [Rust documentation for `properties`](https://docs.rs/icu/latest/icu/properties/index.html) for more information.
///
/// See the [Rust documentation for `CodePointMapData`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapData.html) for more information.
///
/// See the [Rust documentation for `CodePointMapDataBorrowed`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html) for more information.
class ICU4XCodePointMapData16 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XCodePointMapData16._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XCodePointMapData16_destroy'));

  /// Gets the value for a code point.
  ///
  /// See the [Rust documentation for `get`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get) for more information.
  int get(int cp) {
    final result = _getFfi(_underlying, cp);
    return result;
  }

  static final _getFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint16 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCodePointMapData16_get')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Gets the value for a code point (specified as a 32 bit integer, in UTF-32)
  int get32(int cp) {
    final result = _get32Ffi(_underlying, cp);
    return result;
  }

  static final _get32Ffi = _capi<
          ffi.NativeFunction<
              ffi.Uint16 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCodePointMapData16_get32')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Produces an iterator over ranges of code points that map to `value`
  ///
  /// See the [Rust documentation for `iter_ranges_for_value`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value) for more information.
  CodePointRangeIterator iterRangesForValue(int value) {
    final result = _iterRangesForValueFfi(_underlying, value);
    return CodePointRangeIterator._(result);
  }

  static final _iterRangesForValueFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint16)>>('ICU4XCodePointMapData16_iter_ranges_for_value')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Produces an iterator over ranges of code points that do not map to `value`
  ///
  /// See the [Rust documentation for `iter_ranges_for_value_complemented`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value_complemented) for more information.
  CodePointRangeIterator iterRangesForValueComplemented(int value) {
    final result = _iterRangesForValueComplementedFfi(_underlying, value);
    return CodePointRangeIterator._(result);
  }

  static final _iterRangesForValueComplementedFfi = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Opaque> Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Uint16)>>(
          'ICU4XCodePointMapData16_iter_ranges_for_value_complemented')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Gets a [`ICU4XCodePointSetData`] representing all entries in this map that map to the given value
  ///
  /// See the [Rust documentation for `get_set_for_value`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get_set_for_value) for more information.
  ICU4XCodePointSetData getSetForValue(int value) {
    final result = _getSetForValueFfi(_underlying, value);
    return ICU4XCodePointSetData._(result);
  }

  static final _getSetForValueFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint16)>>('ICU4XCodePointMapData16_get_set_for_value')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `script`](https://docs.rs/icu/latest/icu/properties/maps/fn.script.html) for more information.
  factory ICU4XCodePointMapData16.loadScript(ICU4XDataProvider provider) {
    final result = _loadScriptFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointMapData16._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadScriptFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointMapData16_load_script')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
}

/// An ICU4X Unicode Map Property object, capable of querying whether a code point (key) to obtain the Unicode property value, for a specific Unicode property.
///
/// For properties whose values fit into 8 bits.
///
/// See the [Rust documentation for `properties`](https://docs.rs/icu/latest/icu/properties/index.html) for more information.
///
/// See the [Rust documentation for `CodePointMapData`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapData.html) for more information.
///
/// See the [Rust documentation for `CodePointMapDataBorrowed`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html) for more information.
class ICU4XCodePointMapData8 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XCodePointMapData8._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XCodePointMapData8_destroy'));

  /// Gets the value for a code point.
  ///
  /// See the [Rust documentation for `get`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get) for more information.
  int get(int cp) {
    final result = _getFfi(_underlying, cp);
    return result;
  }

  static final _getFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCodePointMapData8_get')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Gets the value for a code point (specified as a 32 bit integer, in UTF-32)
  int get32(int cp) {
    final result = _get32Ffi(_underlying, cp);
    return result;
  }

  static final _get32Ffi = _capi<
          ffi.NativeFunction<
              ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCodePointMapData8_get32')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Converts a general category to its corresponding mask value
  ///
  /// Nonexistant general categories will map to the empty mask
  ///
  /// See the [Rust documentation for `GeneralCategoryGroup`](https://docs.rs/icu/latest/icu/properties/struct.GeneralCategoryGroup.html) for more information.
  static int generalCategoryToMask(int gc) {
    final result = _generalCategoryToMaskFfi(gc);
    return result;
  }

  static final _generalCategoryToMaskFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Uint8)>>(
              'ICU4XCodePointMapData8_general_category_to_mask')
          .asFunction<int Function(int)>(isLeaf: true);

  /// Produces an iterator over ranges of code points that map to `value`
  ///
  /// See the [Rust documentation for `iter_ranges_for_value`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value) for more information.
  CodePointRangeIterator iterRangesForValue(int value) {
    final result = _iterRangesForValueFfi(_underlying, value);
    return CodePointRangeIterator._(result);
  }

  static final _iterRangesForValueFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint8)>>('ICU4XCodePointMapData8_iter_ranges_for_value')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Produces an iterator over ranges of code points that do not map to `value`
  ///
  /// See the [Rust documentation for `iter_ranges_for_value_complemented`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_value_complemented) for more information.
  CodePointRangeIterator iterRangesForValueComplemented(int value) {
    final result = _iterRangesForValueComplementedFfi(_underlying, value);
    return CodePointRangeIterator._(result);
  }

  static final _iterRangesForValueComplementedFfi = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Opaque> Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Uint8)>>(
          'ICU4XCodePointMapData8_iter_ranges_for_value_complemented')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Given a mask value (the nth bit marks property value = n), produce an iterator over ranges of code points
  /// whose property values are contained in the mask.
  ///
  /// The main mask property supported is that for General_Category, which can be obtained via `general_category_to_mask()` or
  /// by using `ICU4XGeneralCategoryNameToMaskMapper`
  ///
  /// Should only be used on maps for properties with values less than 32 (like Generak_Category),
  /// other maps will have unpredictable results
  ///
  /// See the [Rust documentation for `iter_ranges_for_group`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.iter_ranges_for_group) for more information.
  CodePointRangeIterator iterRangesForMask(int mask) {
    final result = _iterRangesForMaskFfi(_underlying, mask);
    return CodePointRangeIterator._(result);
  }

  static final _iterRangesForMaskFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCodePointMapData8_iter_ranges_for_mask')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Gets a [`ICU4XCodePointSetData`] representing all entries in this map that map to the given value
  ///
  /// See the [Rust documentation for `get_set_for_value`](https://docs.rs/icu/latest/icu/properties/maps/struct.CodePointMapDataBorrowed.html#method.get_set_for_value) for more information.
  ICU4XCodePointSetData getSetForValue(int value) {
    final result = _getSetForValueFfi(_underlying, value);
    return ICU4XCodePointSetData._(result);
  }

  static final _getSetForValueFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint8)>>('ICU4XCodePointMapData8_get_set_for_value')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `general_category`](https://docs.rs/icu/latest/icu/properties/maps/fn.general_category.html) for more information.
  factory ICU4XCodePointMapData8.loadGeneralCategory(
      ICU4XDataProvider provider) {
    final result = _loadGeneralCategoryFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointMapData8._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadGeneralCategoryFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointMapData8_load_general_category')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `bidi_class`](https://docs.rs/icu/latest/icu/properties/maps/fn.bidi_class.html) for more information.
  factory ICU4XCodePointMapData8.loadBidiClass(ICU4XDataProvider provider) {
    final result = _loadBidiClassFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointMapData8._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadBidiClassFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointMapData8_load_bidi_class')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `east_asian_width`](https://docs.rs/icu/latest/icu/properties/maps/fn.east_asian_width.html) for more information.
  factory ICU4XCodePointMapData8.loadEastAsianWidth(
      ICU4XDataProvider provider) {
    final result = _loadEastAsianWidthFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointMapData8._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadEastAsianWidthFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointMapData8_load_east_asian_width')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `indic_syllabic_category`](https://docs.rs/icu/latest/icu/properties/maps/fn.indic_syllabic_category.html) for more information.
  factory ICU4XCodePointMapData8.loadIndicSyllabicCategory(
      ICU4XDataProvider provider) {
    final result = _loadIndicSyllabicCategoryFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointMapData8._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadIndicSyllabicCategoryFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointMapData8_load_indic_syllabic_category')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `line_break`](https://docs.rs/icu/latest/icu/properties/maps/fn.line_break.html) for more information.
  factory ICU4XCodePointMapData8.loadLineBreak(ICU4XDataProvider provider) {
    final result = _loadLineBreakFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointMapData8._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadLineBreakFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointMapData8_load_line_break')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `grapheme_cluster_break`](https://docs.rs/icu/latest/icu/properties/maps/fn.grapheme_cluster_break.html) for more information.
  factory ICU4XCodePointMapData8.tryGraphemeClusterBreak(
      ICU4XDataProvider provider) {
    final result = _tryGraphemeClusterBreakFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointMapData8._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _tryGraphemeClusterBreakFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointMapData8_try_grapheme_cluster_break')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `word_break`](https://docs.rs/icu/latest/icu/properties/maps/fn.word_break.html) for more information.
  factory ICU4XCodePointMapData8.loadWordBreak(ICU4XDataProvider provider) {
    final result = _loadWordBreakFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointMapData8._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadWordBreakFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointMapData8_load_word_break')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `sentence_break`](https://docs.rs/icu/latest/icu/properties/maps/fn.sentence_break.html) for more information.
  factory ICU4XCodePointMapData8.loadSentenceBreak(ICU4XDataProvider provider) {
    final result = _loadSentenceBreakFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointMapData8._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadSentenceBreakFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointMapData8_load_sentence_break')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
}

/// See the [Rust documentation for `CodePointInversionListBuilder`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html) for more information.
class ICU4XCodePointSetBuilder implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XCodePointSetBuilder._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XCodePointSetBuilder_destroy'));

  /// Make a new set builder containing nothing
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.new) for more information.
  factory ICU4XCodePointSetBuilder() {
    final result = _createFfi();
    return ICU4XCodePointSetBuilder._(result);
  }
  static final _createFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XCodePointSetBuilder_create')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Build this into a set
  ///
  /// This object is repopulated with an empty builder
  ///
  /// See the [Rust documentation for `build`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.build) for more information.
  ICU4XCodePointSetData build() {
    final result = _buildFfi(_underlying);
    return ICU4XCodePointSetData._(result);
  }

  static final _buildFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCodePointSetBuilder_build')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Complements this set
  ///
  /// (Elements in this set are removed and vice versa)
  ///
  /// See the [Rust documentation for `complement`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.complement) for more information.
  void complement() {
    _complementFfi(_underlying);
  }

  static final _complementFfi =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XCodePointSetBuilder_complement')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns whether this set is empty
  ///
  /// See the [Rust documentation for `is_empty`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.is_empty) for more information.
  bool get isEmpty {
    final result = _isEmptyFfi(_underlying);
    return result;
  }

  static final _isEmptyFfi =
      _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XCodePointSetBuilder_is_empty')
          .asFunction<bool Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Add a single character to the set
  ///
  /// See the [Rust documentation for `add_char`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.add_char) for more information.
  void addChar(int ch) {
    _addCharFfi(_underlying, ch);
  }

  static final _addCharFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCodePointSetBuilder_add_char')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Add a single u32 value to the set
  ///
  /// See the [Rust documentation for `add_u32`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.add_u32) for more information.
  void addU32(int ch) {
    _addU32Ffi(_underlying, ch);
  }

  static final _addU32Ffi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCodePointSetBuilder_add_u32')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Add an inclusive range of characters to the set
  ///
  /// See the [Rust documentation for `add_range`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.add_range) for more information.
  void addInclusiveRange(int start, int end) {
    _addInclusiveRangeFfi(_underlying, start, end);
  }

  static final _addInclusiveRangeFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32,
                  ffi.Uint32)>>('ICU4XCodePointSetBuilder_add_inclusive_range')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int, int)>(
          isLeaf: true);

  /// Add an inclusive range of u32s to the set
  ///
  /// See the [Rust documentation for `add_range_u32`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.add_range_u32) for more information.
  void addInclusiveRangeU32(int start, int end) {
    _addInclusiveRangeU32Ffi(_underlying, start, end);
  }

  static final _addInclusiveRangeU32Ffi = _capi<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Uint32, ffi.Uint32)>>(
          'ICU4XCodePointSetBuilder_add_inclusive_range_u32')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int, int)>(
          isLeaf: true);

  /// Add all elements that belong to the provided set to the set
  ///
  /// See the [Rust documentation for `add_set`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.add_set) for more information.
  void addSet(ICU4XCodePointSetData data) {
    _addSetFfi(_underlying, data._underlying);
  }

  static final _addSetFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCodePointSetBuilder_add_set')
      .asFunction<
          void Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Remove a single character to the set
  ///
  /// See the [Rust documentation for `remove_char`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.remove_char) for more information.
  void removeChar(int ch) {
    _removeCharFfi(_underlying, ch);
  }

  static final _removeCharFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCodePointSetBuilder_remove_char')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Remove an inclusive range of characters from the set
  ///
  /// See the [Rust documentation for `remove_range`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.remove_range) for more information.
  void removeInclusiveRange(int start, int end) {
    _removeInclusiveRangeFfi(_underlying, start, end);
  }

  static final _removeInclusiveRangeFfi = _capi<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Uint32, ffi.Uint32)>>(
          'ICU4XCodePointSetBuilder_remove_inclusive_range')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int, int)>(
          isLeaf: true);

  /// Remove all elements that belong to the provided set from the set
  ///
  /// See the [Rust documentation for `remove_set`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.remove_set) for more information.
  void removeSet(ICU4XCodePointSetData data) {
    _removeSetFfi(_underlying, data._underlying);
  }

  static final _removeSetFfi = _capi<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetBuilder_remove_set')
      .asFunction<
          void Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Removes all elements from the set except a single character
  ///
  /// See the [Rust documentation for `retain_char`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.retain_char) for more information.
  void retainChar(int ch) {
    _retainCharFfi(_underlying, ch);
  }

  static final _retainCharFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCodePointSetBuilder_retain_char')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Removes all elements from the set except an inclusive range of characters f
  ///
  /// See the [Rust documentation for `retain_range`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.retain_range) for more information.
  void retainInclusiveRange(int start, int end) {
    _retainInclusiveRangeFfi(_underlying, start, end);
  }

  static final _retainInclusiveRangeFfi = _capi<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Uint32, ffi.Uint32)>>(
          'ICU4XCodePointSetBuilder_retain_inclusive_range')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int, int)>(
          isLeaf: true);

  /// Removes all elements from the set except all elements in the provided set
  ///
  /// See the [Rust documentation for `retain_set`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.retain_set) for more information.
  void retainSet(ICU4XCodePointSetData data) {
    _retainSetFfi(_underlying, data._underlying);
  }

  static final _retainSetFfi = _capi<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetBuilder_retain_set')
      .asFunction<
          void Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Complement a single character to the set
  ///
  /// (Characters which are in this set are removed and vice versa)
  ///
  /// See the [Rust documentation for `complement_char`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.complement_char) for more information.
  void complementChar(int ch) {
    _complementCharFfi(_underlying, ch);
  }

  static final _complementCharFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCodePointSetBuilder_complement_char')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Complement an inclusive range of characters from the set
  ///
  /// (Characters which are in this set are removed and vice versa)
  ///
  /// See the [Rust documentation for `complement_range`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.complement_range) for more information.
  void complementInclusiveRange(int start, int end) {
    _complementInclusiveRangeFfi(_underlying, start, end);
  }

  static final _complementInclusiveRangeFfi = _capi<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Uint32, ffi.Uint32)>>(
          'ICU4XCodePointSetBuilder_complement_inclusive_range')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int, int)>(
          isLeaf: true);

  /// Complement all elements that belong to the provided set from the set
  ///
  /// (Characters which are in this set are removed and vice versa)
  ///
  /// See the [Rust documentation for `complement_set`](https://docs.rs/icu/latest/icu/collections/codepointinvlist/struct.CodePointInversionListBuilder.html#method.complement_set) for more information.
  void complementSet(ICU4XCodePointSetData data) {
    _complementSetFfi(_underlying, data._underlying);
  }

  static final _complementSetFfi = _capi<
              ffi.NativeFunction<
                  ffi.Void Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetBuilder_complement_set')
      .asFunction<
          void Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
///
/// See the [Rust documentation for `properties`](https://docs.rs/icu/latest/icu/properties/index.html) for more information.
///
/// See the [Rust documentation for `CodePointSetData`](https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetData.html) for more information.
///
/// See the [Rust documentation for `CodePointSetDataBorrowed`](https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetDataBorrowed.html) for more information.
class ICU4XCodePointSetData implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XCodePointSetData._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XCodePointSetData_destroy'));

  /// Checks whether the code point is in the set.
  ///
  /// See the [Rust documentation for `contains`](https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.contains) for more information.
  bool contains(int cp) {
    final result = _containsFfi(_underlying, cp);
    return result;
  }

  static final _containsFfi = _capi<
          ffi.NativeFunction<
              ffi.Bool Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCodePointSetData_contains')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Checks whether the code point (specified as a 32 bit integer, in UTF-32) is in the set.
  bool contains32(int cp) {
    final result = _contains32Ffi(_underlying, cp);
    return result;
  }

  static final _contains32Ffi = _capi<
          ffi.NativeFunction<
              ffi.Bool Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XCodePointSetData_contains32')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Produces an iterator over ranges of code points contained in this set
  ///
  /// See the [Rust documentation for `iter_ranges`](https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.iter_ranges) for more information.
  CodePointRangeIterator get iterRanges {
    final result = _iterRangesFfi(_underlying);
    return CodePointRangeIterator._(result);
  }

  static final _iterRangesFfi = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_iter_ranges')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Produces an iterator over ranges of code points not contained in this set
  ///
  /// See the [Rust documentation for `iter_ranges_complemented`](https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetDataBorrowed.html#method.iter_ranges_complemented) for more information.
  CodePointRangeIterator get iterRangesComplemented {
    final result = _iterRangesComplementedFfi(_underlying);
    return CodePointRangeIterator._(result);
  }

  static final _iterRangesComplementedFfi = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_iter_ranges_complemented')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// which is a mask with the same format as the `U_GC_XX_MASK` mask in ICU4C
  ///
  /// See the [Rust documentation for `for_general_category_group`](https://docs.rs/icu/latest/icu/properties/sets/fn.for_general_category_group.html) for more information.
  factory ICU4XCodePointSetData.loadForGeneralCategoryGroup(
      ICU4XDataProvider provider, int group) {
    final result = _loadForGeneralCategoryGroupFfi(provider._underlying, group);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadForGeneralCategoryGroupFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Uint32)>>(
          'ICU4XCodePointSetData_load_for_general_category_group')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>, int)>(
          isLeaf: true);

  /// See the [Rust documentation for `ascii_hex_digit`](https://docs.rs/icu/latest/icu/properties/sets/fn.ascii_hex_digit.html) for more information.
  factory ICU4XCodePointSetData.loadAsciiHexDigit(ICU4XDataProvider provider) {
    final result = _loadAsciiHexDigitFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadAsciiHexDigitFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_ascii_hex_digit')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `alnum`](https://docs.rs/icu/latest/icu/properties/sets/fn.alnum.html) for more information.
  factory ICU4XCodePointSetData.loadAlnum(ICU4XDataProvider provider) {
    final result = _loadAlnumFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadAlnumFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCodePointSetData_load_alnum')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `alphabetic`](https://docs.rs/icu/latest/icu/properties/sets/fn.alphabetic.html) for more information.
  factory ICU4XCodePointSetData.loadAlphabetic(ICU4XDataProvider provider) {
    final result = _loadAlphabeticFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadAlphabeticFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_alphabetic')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `bidi_control`](https://docs.rs/icu/latest/icu/properties/sets/fn.bidi_control.html) for more information.
  factory ICU4XCodePointSetData.loadBidiControl(ICU4XDataProvider provider) {
    final result = _loadBidiControlFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadBidiControlFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_bidi_control')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `bidi_mirrored`](https://docs.rs/icu/latest/icu/properties/sets/fn.bidi_mirrored.html) for more information.
  factory ICU4XCodePointSetData.loadBidiMirrored(ICU4XDataProvider provider) {
    final result = _loadBidiMirroredFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadBidiMirroredFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_bidi_mirrored')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `blank`](https://docs.rs/icu/latest/icu/properties/sets/fn.blank.html) for more information.
  factory ICU4XCodePointSetData.loadBlank(ICU4XDataProvider provider) {
    final result = _loadBlankFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadBlankFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCodePointSetData_load_blank')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `cased`](https://docs.rs/icu/latest/icu/properties/sets/fn.cased.html) for more information.
  factory ICU4XCodePointSetData.loadCased(ICU4XDataProvider provider) {
    final result = _loadCasedFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadCasedFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCodePointSetData_load_cased')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `case_ignorable`](https://docs.rs/icu/latest/icu/properties/sets/fn.case_ignorable.html) for more information.
  factory ICU4XCodePointSetData.loadCaseIgnorable(ICU4XDataProvider provider) {
    final result = _loadCaseIgnorableFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadCaseIgnorableFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_case_ignorable')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `full_composition_exclusion`](https://docs.rs/icu/latest/icu/properties/sets/fn.full_composition_exclusion.html) for more information.
  factory ICU4XCodePointSetData.loadFullCompositionExclusion(
      ICU4XDataProvider provider) {
    final result = _loadFullCompositionExclusionFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadFullCompositionExclusionFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_full_composition_exclusion')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `changes_when_casefolded`](https://docs.rs/icu/latest/icu/properties/sets/fn.changes_when_casefolded.html) for more information.
  factory ICU4XCodePointSetData.loadChangesWhenCasefolded(
      ICU4XDataProvider provider) {
    final result = _loadChangesWhenCasefoldedFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadChangesWhenCasefoldedFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_changes_when_casefolded')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `changes_when_casemapped`](https://docs.rs/icu/latest/icu/properties/sets/fn.changes_when_casemapped.html) for more information.
  factory ICU4XCodePointSetData.loadChangesWhenCasemapped(
      ICU4XDataProvider provider) {
    final result = _loadChangesWhenCasemappedFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadChangesWhenCasemappedFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_changes_when_casemapped')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `changes_when_nfkc_casefolded`](https://docs.rs/icu/latest/icu/properties/sets/fn.changes_when_nfkc_casefolded.html) for more information.
  factory ICU4XCodePointSetData.loadChangesWhenNfkcCasefolded(
      ICU4XDataProvider provider) {
    final result = _loadChangesWhenNfkcCasefoldedFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadChangesWhenNfkcCasefoldedFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_changes_when_nfkc_casefolded')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `changes_when_lowercased`](https://docs.rs/icu/latest/icu/properties/sets/fn.changes_when_lowercased.html) for more information.
  factory ICU4XCodePointSetData.loadChangesWhenLowercased(
      ICU4XDataProvider provider) {
    final result = _loadChangesWhenLowercasedFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadChangesWhenLowercasedFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_changes_when_lowercased')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `changes_when_titlecased`](https://docs.rs/icu/latest/icu/properties/sets/fn.changes_when_titlecased.html) for more information.
  factory ICU4XCodePointSetData.loadChangesWhenTitlecased(
      ICU4XDataProvider provider) {
    final result = _loadChangesWhenTitlecasedFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadChangesWhenTitlecasedFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_changes_when_titlecased')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `changes_when_uppercased`](https://docs.rs/icu/latest/icu/properties/sets/fn.changes_when_uppercased.html) for more information.
  factory ICU4XCodePointSetData.loadChangesWhenUppercased(
      ICU4XDataProvider provider) {
    final result = _loadChangesWhenUppercasedFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadChangesWhenUppercasedFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_changes_when_uppercased')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `dash`](https://docs.rs/icu/latest/icu/properties/sets/fn.dash.html) for more information.
  factory ICU4XCodePointSetData.loadDash(ICU4XDataProvider provider) {
    final result = _loadDashFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadDashFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCodePointSetData_load_dash')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `deprecated`](https://docs.rs/icu/latest/icu/properties/sets/fn.deprecated.html) for more information.
  factory ICU4XCodePointSetData.loadDeprecated(ICU4XDataProvider provider) {
    final result = _loadDeprecatedFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadDeprecatedFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_deprecated')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `default_ignorable_code_point`](https://docs.rs/icu/latest/icu/properties/sets/fn.default_ignorable_code_point.html) for more information.
  factory ICU4XCodePointSetData.loadDefaultIgnorableCodePoint(
      ICU4XDataProvider provider) {
    final result = _loadDefaultIgnorableCodePointFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadDefaultIgnorableCodePointFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_default_ignorable_code_point')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `diacritic`](https://docs.rs/icu/latest/icu/properties/sets/fn.diacritic.html) for more information.
  factory ICU4XCodePointSetData.loadDiacritic(ICU4XDataProvider provider) {
    final result = _loadDiacriticFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadDiacriticFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_diacritic')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `emoji_modifier_base`](https://docs.rs/icu/latest/icu/properties/sets/fn.emoji_modifier_base.html) for more information.
  factory ICU4XCodePointSetData.loadEmojiModifierBase(
      ICU4XDataProvider provider) {
    final result = _loadEmojiModifierBaseFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadEmojiModifierBaseFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_emoji_modifier_base')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `emoji_component`](https://docs.rs/icu/latest/icu/properties/sets/fn.emoji_component.html) for more information.
  factory ICU4XCodePointSetData.loadEmojiComponent(ICU4XDataProvider provider) {
    final result = _loadEmojiComponentFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadEmojiComponentFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_emoji_component')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `emoji_modifier`](https://docs.rs/icu/latest/icu/properties/sets/fn.emoji_modifier.html) for more information.
  factory ICU4XCodePointSetData.loadEmojiModifier(ICU4XDataProvider provider) {
    final result = _loadEmojiModifierFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadEmojiModifierFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_emoji_modifier')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `emoji`](https://docs.rs/icu/latest/icu/properties/sets/fn.emoji.html) for more information.
  factory ICU4XCodePointSetData.loadEmoji(ICU4XDataProvider provider) {
    final result = _loadEmojiFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadEmojiFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCodePointSetData_load_emoji')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `emoji_presentation`](https://docs.rs/icu/latest/icu/properties/sets/fn.emoji_presentation.html) for more information.
  factory ICU4XCodePointSetData.loadEmojiPresentation(
      ICU4XDataProvider provider) {
    final result = _loadEmojiPresentationFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadEmojiPresentationFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_emoji_presentation')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `extender`](https://docs.rs/icu/latest/icu/properties/sets/fn.extender.html) for more information.
  factory ICU4XCodePointSetData.loadExtender(ICU4XDataProvider provider) {
    final result = _loadExtenderFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadExtenderFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_extender')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `extended_pictographic`](https://docs.rs/icu/latest/icu/properties/sets/fn.extended_pictographic.html) for more information.
  factory ICU4XCodePointSetData.loadExtendedPictographic(
      ICU4XDataProvider provider) {
    final result = _loadExtendedPictographicFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadExtendedPictographicFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_extended_pictographic')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `graph`](https://docs.rs/icu/latest/icu/properties/sets/fn.graph.html) for more information.
  factory ICU4XCodePointSetData.loadGraph(ICU4XDataProvider provider) {
    final result = _loadGraphFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadGraphFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCodePointSetData_load_graph')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `grapheme_base`](https://docs.rs/icu/latest/icu/properties/sets/fn.grapheme_base.html) for more information.
  factory ICU4XCodePointSetData.loadGraphemeBase(ICU4XDataProvider provider) {
    final result = _loadGraphemeBaseFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadGraphemeBaseFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_grapheme_base')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `grapheme_extend`](https://docs.rs/icu/latest/icu/properties/sets/fn.grapheme_extend.html) for more information.
  factory ICU4XCodePointSetData.loadGraphemeExtend(ICU4XDataProvider provider) {
    final result = _loadGraphemeExtendFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadGraphemeExtendFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_grapheme_extend')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `grapheme_link`](https://docs.rs/icu/latest/icu/properties/sets/fn.grapheme_link.html) for more information.
  factory ICU4XCodePointSetData.loadGraphemeLink(ICU4XDataProvider provider) {
    final result = _loadGraphemeLinkFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadGraphemeLinkFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_grapheme_link')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `hex_digit`](https://docs.rs/icu/latest/icu/properties/sets/fn.hex_digit.html) for more information.
  factory ICU4XCodePointSetData.loadHexDigit(ICU4XDataProvider provider) {
    final result = _loadHexDigitFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadHexDigitFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_hex_digit')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `hyphen`](https://docs.rs/icu/latest/icu/properties/sets/fn.hyphen.html) for more information.
  factory ICU4XCodePointSetData.loadHyphen(ICU4XDataProvider provider) {
    final result = _loadHyphenFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadHyphenFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_hyphen')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `id_continue`](https://docs.rs/icu/latest/icu/properties/sets/fn.id_continue.html) for more information.
  factory ICU4XCodePointSetData.loadIdContinue(ICU4XDataProvider provider) {
    final result = _loadIdContinueFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadIdContinueFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_id_continue')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `ideographic`](https://docs.rs/icu/latest/icu/properties/sets/fn.ideographic.html) for more information.
  factory ICU4XCodePointSetData.loadIdeographic(ICU4XDataProvider provider) {
    final result = _loadIdeographicFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadIdeographicFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_ideographic')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `id_start`](https://docs.rs/icu/latest/icu/properties/sets/fn.id_start.html) for more information.
  factory ICU4XCodePointSetData.loadIdStart(ICU4XDataProvider provider) {
    final result = _loadIdStartFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadIdStartFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_id_start')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `ids_binary_operator`](https://docs.rs/icu/latest/icu/properties/sets/fn.ids_binary_operator.html) for more information.
  factory ICU4XCodePointSetData.loadIdsBinaryOperator(
      ICU4XDataProvider provider) {
    final result = _loadIdsBinaryOperatorFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadIdsBinaryOperatorFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_ids_binary_operator')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `ids_trinary_operator`](https://docs.rs/icu/latest/icu/properties/sets/fn.ids_trinary_operator.html) for more information.
  factory ICU4XCodePointSetData.loadIdsTrinaryOperator(
      ICU4XDataProvider provider) {
    final result = _loadIdsTrinaryOperatorFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadIdsTrinaryOperatorFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_ids_trinary_operator')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `join_control`](https://docs.rs/icu/latest/icu/properties/sets/fn.join_control.html) for more information.
  factory ICU4XCodePointSetData.loadJoinControl(ICU4XDataProvider provider) {
    final result = _loadJoinControlFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadJoinControlFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_join_control')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `logical_order_exception`](https://docs.rs/icu/latest/icu/properties/sets/fn.logical_order_exception.html) for more information.
  factory ICU4XCodePointSetData.loadLogicalOrderException(
      ICU4XDataProvider provider) {
    final result = _loadLogicalOrderExceptionFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadLogicalOrderExceptionFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_logical_order_exception')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `lowercase`](https://docs.rs/icu/latest/icu/properties/sets/fn.lowercase.html) for more information.
  factory ICU4XCodePointSetData.loadLowercase(ICU4XDataProvider provider) {
    final result = _loadLowercaseFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadLowercaseFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_lowercase')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `math`](https://docs.rs/icu/latest/icu/properties/sets/fn.math.html) for more information.
  factory ICU4XCodePointSetData.loadMath(ICU4XDataProvider provider) {
    final result = _loadMathFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadMathFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCodePointSetData_load_math')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `noncharacter_code_point`](https://docs.rs/icu/latest/icu/properties/sets/fn.noncharacter_code_point.html) for more information.
  factory ICU4XCodePointSetData.loadNoncharacterCodePoint(
      ICU4XDataProvider provider) {
    final result = _loadNoncharacterCodePointFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadNoncharacterCodePointFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_noncharacter_code_point')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `nfc_inert`](https://docs.rs/icu/latest/icu/properties/sets/fn.nfc_inert.html) for more information.
  factory ICU4XCodePointSetData.loadNfcInert(ICU4XDataProvider provider) {
    final result = _loadNfcInertFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadNfcInertFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_nfc_inert')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `nfd_inert`](https://docs.rs/icu/latest/icu/properties/sets/fn.nfd_inert.html) for more information.
  factory ICU4XCodePointSetData.loadNfdInert(ICU4XDataProvider provider) {
    final result = _loadNfdInertFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadNfdInertFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_nfd_inert')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `nfkc_inert`](https://docs.rs/icu/latest/icu/properties/sets/fn.nfkc_inert.html) for more information.
  factory ICU4XCodePointSetData.loadNfkcInert(ICU4XDataProvider provider) {
    final result = _loadNfkcInertFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadNfkcInertFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_nfkc_inert')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `nfkd_inert`](https://docs.rs/icu/latest/icu/properties/sets/fn.nfkd_inert.html) for more information.
  factory ICU4XCodePointSetData.loadNfkdInert(ICU4XDataProvider provider) {
    final result = _loadNfkdInertFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadNfkdInertFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_nfkd_inert')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `pattern_syntax`](https://docs.rs/icu/latest/icu/properties/sets/fn.pattern_syntax.html) for more information.
  factory ICU4XCodePointSetData.loadPatternSyntax(ICU4XDataProvider provider) {
    final result = _loadPatternSyntaxFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadPatternSyntaxFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_pattern_syntax')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `pattern_white_space`](https://docs.rs/icu/latest/icu/properties/sets/fn.pattern_white_space.html) for more information.
  factory ICU4XCodePointSetData.loadPatternWhiteSpace(
      ICU4XDataProvider provider) {
    final result = _loadPatternWhiteSpaceFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadPatternWhiteSpaceFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_pattern_white_space')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `prepended_concatenation_mark`](https://docs.rs/icu/latest/icu/properties/sets/fn.prepended_concatenation_mark.html) for more information.
  factory ICU4XCodePointSetData.loadPrependedConcatenationMark(
      ICU4XDataProvider provider) {
    final result = _loadPrependedConcatenationMarkFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadPrependedConcatenationMarkFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_prepended_concatenation_mark')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `print`](https://docs.rs/icu/latest/icu/properties/sets/fn.print.html) for more information.
  factory ICU4XCodePointSetData.loadPrint(ICU4XDataProvider provider) {
    final result = _loadPrintFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadPrintFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCodePointSetData_load_print')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `quotation_mark`](https://docs.rs/icu/latest/icu/properties/sets/fn.quotation_mark.html) for more information.
  factory ICU4XCodePointSetData.loadQuotationMark(ICU4XDataProvider provider) {
    final result = _loadQuotationMarkFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadQuotationMarkFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_quotation_mark')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `radical`](https://docs.rs/icu/latest/icu/properties/sets/fn.radical.html) for more information.
  factory ICU4XCodePointSetData.loadRadical(ICU4XDataProvider provider) {
    final result = _loadRadicalFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadRadicalFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_radical')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `regional_indicator`](https://docs.rs/icu/latest/icu/properties/sets/fn.regional_indicator.html) for more information.
  factory ICU4XCodePointSetData.loadRegionalIndicator(
      ICU4XDataProvider provider) {
    final result = _loadRegionalIndicatorFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadRegionalIndicatorFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_regional_indicator')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `soft_dotted`](https://docs.rs/icu/latest/icu/properties/sets/fn.soft_dotted.html) for more information.
  factory ICU4XCodePointSetData.loadSoftDotted(ICU4XDataProvider provider) {
    final result = _loadSoftDottedFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadSoftDottedFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_soft_dotted')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `segment_starter`](https://docs.rs/icu/latest/icu/properties/sets/fn.segment_starter.html) for more information.
  factory ICU4XCodePointSetData.loadSegmentStarter(ICU4XDataProvider provider) {
    final result = _loadSegmentStarterFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadSegmentStarterFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_segment_starter')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `case_sensitive`](https://docs.rs/icu/latest/icu/properties/sets/fn.case_sensitive.html) for more information.
  factory ICU4XCodePointSetData.loadCaseSensitive(ICU4XDataProvider provider) {
    final result = _loadCaseSensitiveFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadCaseSensitiveFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_case_sensitive')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `sentence_terminal`](https://docs.rs/icu/latest/icu/properties/sets/fn.sentence_terminal.html) for more information.
  factory ICU4XCodePointSetData.loadSentenceTerminal(
      ICU4XDataProvider provider) {
    final result = _loadSentenceTerminalFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadSentenceTerminalFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_sentence_terminal')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `terminal_punctuation`](https://docs.rs/icu/latest/icu/properties/sets/fn.terminal_punctuation.html) for more information.
  factory ICU4XCodePointSetData.loadTerminalPunctuation(
      ICU4XDataProvider provider) {
    final result = _loadTerminalPunctuationFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadTerminalPunctuationFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_terminal_punctuation')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `unified_ideograph`](https://docs.rs/icu/latest/icu/properties/sets/fn.unified_ideograph.html) for more information.
  factory ICU4XCodePointSetData.loadUnifiedIdeograph(
      ICU4XDataProvider provider) {
    final result = _loadUnifiedIdeographFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadUnifiedIdeographFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_unified_ideograph')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `uppercase`](https://docs.rs/icu/latest/icu/properties/sets/fn.uppercase.html) for more information.
  factory ICU4XCodePointSetData.loadUppercase(ICU4XDataProvider provider) {
    final result = _loadUppercaseFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadUppercaseFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_uppercase')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `variation_selector`](https://docs.rs/icu/latest/icu/properties/sets/fn.variation_selector.html) for more information.
  factory ICU4XCodePointSetData.loadVariationSelector(
      ICU4XDataProvider provider) {
    final result = _loadVariationSelectorFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadVariationSelectorFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_variation_selector')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `white_space`](https://docs.rs/icu/latest/icu/properties/sets/fn.white_space.html) for more information.
  factory ICU4XCodePointSetData.loadWhiteSpace(ICU4XDataProvider provider) {
    final result = _loadWhiteSpaceFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadWhiteSpaceFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_white_space')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `xdigit`](https://docs.rs/icu/latest/icu/properties/sets/fn.xdigit.html) for more information.
  factory ICU4XCodePointSetData.loadXdigit(ICU4XDataProvider provider) {
    final result = _loadXdigitFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadXdigitFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_xdigit')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `xid_continue`](https://docs.rs/icu/latest/icu/properties/sets/fn.xid_continue.html) for more information.
  factory ICU4XCodePointSetData.loadXidContinue(ICU4XDataProvider provider) {
    final result = _loadXidContinueFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadXidContinueFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_xid_continue')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `xid_start`](https://docs.rs/icu/latest/icu/properties/sets/fn.xid_start.html) for more information.
  factory ICU4XCodePointSetData.loadXidStart(ICU4XDataProvider provider) {
    final result = _loadXidStartFfi(provider._underlying);
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadXidStartFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCodePointSetData_load_xid_start')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Loads data for a property specified as a string as long as it is one of the
  /// [ECMA-262 binary properties][ecma] (not including Any, ASCII, and Assigned pseudoproperties).
  ///
  /// Returns `ICU4XError::PropertyUnexpectedPropertyNameError` in case the string does not
  /// match any property in the list
  ///
  /// [ecma]: https://tc39.es/ecma262/#table-binary-unicode-properties
  ///
  /// See the [Rust documentation for `for_ecma262`](https://docs.rs/icu/latest/icu/properties/sets/fn.for_ecma262.html) for more information.
  factory ICU4XCodePointSetData.loadForEcma262(
      ICU4XDataProvider provider, String propertyName) {
    final alloc = ffi2.Arena();
    final propertyNameSlice = _SliceFfi2Utf8._fromDart(propertyName, alloc);

    final result = _loadForEcma262Ffi(provider._underlying,
        propertyNameSlice._bytes, propertyNameSlice._length);
    alloc.releaseAll();
    return result.isOk
        ? ICU4XCodePointSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadForEcma262Ffi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XCodePointSetData_load_for_ecma262')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);
}

/// See the [Rust documentation for `Collator`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html) for more information.
class ICU4XCollator implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XCollator._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XCollator_destroy'));

  /// Construct a new Collator instance.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.try_new) for more information.
  factory ICU4XCollator.v1(ICU4XDataProvider provider, ICU4XLocale locale,
      ICU4XCollatorOptionsV1 options) {
    final result = _createV1Ffi(
        provider._underlying, locale._underlying, options._underlying);
    return result.isOk
        ? ICU4XCollator._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createV1Ffi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  _ICU4XCollatorOptionsV1Ffi)>>('ICU4XCollator_create_v1')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              _ICU4XCollatorOptionsV1Ffi)>(isLeaf: true);

  /// Compare potentially ill-formed UTF-8 strings.
  ///
  /// Ill-formed input is compared
  /// as if errors had been replaced with REPLACEMENT CHARACTERs according
  /// to the WHATWG Encoding Standard.
  ///
  /// See the [Rust documentation for `compare_utf8`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.compare_utf8) for more information.
  ICU4XOrdering compare(String left, String right) {
    final alloc = ffi2.Arena();
    final leftSlice = _SliceFfi2Utf8._fromDart(left, alloc);
    final rightSlice = _SliceFfi2Utf8._fromDart(right, alloc);

    final result = _compareFfi(_underlying, leftSlice._bytes, leftSlice._length,
        rightSlice._bytes, rightSlice._length);
    alloc.releaseAll();
    return ICU4XOrdering._(result);
  }

  static final _compareFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XCollator_compare')
      .asFunction<
          int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>, int,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Compare guaranteed well-formed UTF-8 strings.
  ///
  /// Note: In C++, passing ill-formed UTF-8 strings is undefined behavior
  /// (and may be memory-unsafe to do so, too).
  ///
  /// See the [Rust documentation for `compare`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.compare) for more information.
  ICU4XOrdering compareValidUtf8(String left, String right) {
    final alloc = ffi2.Arena();
    final leftSlice = _SliceFfi2Utf8._fromDart(left, alloc);
    final rightSlice = _SliceFfi2Utf8._fromDart(right, alloc);

    final result = _compareValidUtf8Ffi(_underlying, leftSlice._bytes,
        leftSlice._length, rightSlice._bytes, rightSlice._length);
    alloc.releaseAll();
    return ICU4XOrdering._(result);
  }

  static final _compareValidUtf8Ffi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XCollator_compare_valid_utf8')
      .asFunction<
          int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>, int,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Compare potentially ill-formed UTF-16 strings, with unpaired surrogates
  /// compared as REPLACEMENT CHARACTER.
  ///
  /// See the [Rust documentation for `compare_utf16`](https://docs.rs/icu/latest/icu/collator/struct.Collator.html#method.compare_utf16) for more information.
  ICU4XOrdering compareUtf16(Uint16List left, Uint16List right) {
    final alloc = ffi2.Arena();
    final leftSlice = _SliceFfiUint16._fromDart(left, alloc);
    final rightSlice = _SliceFfiUint16._fromDart(right, alloc);

    final result = _compareUtf16Ffi(_underlying, leftSlice._bytes,
        leftSlice._length, rightSlice._bytes, rightSlice._length);
    alloc.releaseAll();
    return ICU4XOrdering._(result);
  }

  static final _compareUtf16Ffi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Uint16>,
                  ffi.Size,
                  ffi.Pointer<ffi.Uint16>,
                  ffi.Size)>>('ICU4XCollator_compare_utf16')
      .asFunction<
          int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint16>, int,
              ffi.Pointer<ffi.Uint16>, int)>(isLeaf: true);
}

/// See the [Rust documentation for `AlternateHandling`](https://docs.rs/icu/latest/icu/collator/enum.AlternateHandling.html) for more information.
enum ICU4XCollatorAlternateHandling {
  auto.__(0),
  nonIgnorable.__(1),
  shifted.__(2);

  const ICU4XCollatorAlternateHandling.__(this._id);

  // ignore: unused_element
  factory ICU4XCollatorAlternateHandling._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `BackwardSecondLevel`](https://docs.rs/icu/latest/icu/collator/enum.BackwardSecondLevel.html) for more information.
enum ICU4XCollatorBackwardSecondLevel {
  auto.__(0),
  off.__(1),
  on.__(2);

  const ICU4XCollatorBackwardSecondLevel.__(this._id);

  // ignore: unused_element
  factory ICU4XCollatorBackwardSecondLevel._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `CaseFirst`](https://docs.rs/icu/latest/icu/collator/enum.CaseFirst.html) for more information.
enum ICU4XCollatorCaseFirst {
  auto.__(0),
  off.__(1),
  lowerFirst.__(2),
  upperFirst.__(3);

  const ICU4XCollatorCaseFirst.__(this._id);

  // ignore: unused_element
  factory ICU4XCollatorCaseFirst._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `CaseLevel`](https://docs.rs/icu/latest/icu/collator/enum.CaseLevel.html) for more information.
enum ICU4XCollatorCaseLevel {
  auto.__(0),
  off.__(1),
  on.__(2);

  const ICU4XCollatorCaseLevel.__(this._id);

  // ignore: unused_element
  factory ICU4XCollatorCaseLevel._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `MaxVariable`](https://docs.rs/icu/latest/icu/collator/enum.MaxVariable.html) for more information.
enum ICU4XCollatorMaxVariable {
  auto.__(0),
  space.__(1),
  punctuation.__(2),
  symbol.__(3),
  currency.__(4);

  const ICU4XCollatorMaxVariable.__(this._id);

  // ignore: unused_element
  factory ICU4XCollatorMaxVariable._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `Numeric`](https://docs.rs/icu/latest/icu/collator/enum.Numeric.html) for more information.
enum ICU4XCollatorNumeric {
  auto.__(0),
  off.__(1),
  on.__(2);

  const ICU4XCollatorNumeric.__(this._id);

  // ignore: unused_element
  factory ICU4XCollatorNumeric._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `CollatorOptions`](https://docs.rs/icu/latest/icu/collator/struct.CollatorOptions.html) for more information.
class _ICU4XCollatorOptionsV1Ffi extends ffi.Struct {
  @ffi.Uint32()
  external int strength;
  @ffi.Uint32()
  external int alternateHandling;
  @ffi.Uint32()
  external int caseFirst;
  @ffi.Uint32()
  external int maxVariable;
  @ffi.Uint32()
  external int caseLevel;
  @ffi.Uint32()
  external int numeric;
  @ffi.Uint32()
  external int backwardSecondLevel;
}

class ICU4XCollatorOptionsV1 {
  final _ICU4XCollatorOptionsV1Ffi _underlying;

  // ignore: unused_element
  ICU4XCollatorOptionsV1._(this._underlying);

  factory ICU4XCollatorOptionsV1() {
    final pointer = ffi2.calloc<_ICU4XCollatorOptionsV1Ffi>();
    final result = ICU4XCollatorOptionsV1._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  ICU4XCollatorStrength get strength =>
      ICU4XCollatorStrength._(_underlying.strength);
  set strength(ICU4XCollatorStrength strength) {
    _underlying.strength = strength._id;
  }

  ICU4XCollatorAlternateHandling get alternateHandling =>
      ICU4XCollatorAlternateHandling._(_underlying.alternateHandling);
  set alternateHandling(ICU4XCollatorAlternateHandling alternateHandling) {
    _underlying.alternateHandling = alternateHandling._id;
  }

  ICU4XCollatorCaseFirst get caseFirst =>
      ICU4XCollatorCaseFirst._(_underlying.caseFirst);
  set caseFirst(ICU4XCollatorCaseFirst caseFirst) {
    _underlying.caseFirst = caseFirst._id;
  }

  ICU4XCollatorMaxVariable get maxVariable =>
      ICU4XCollatorMaxVariable._(_underlying.maxVariable);
  set maxVariable(ICU4XCollatorMaxVariable maxVariable) {
    _underlying.maxVariable = maxVariable._id;
  }

  ICU4XCollatorCaseLevel get caseLevel =>
      ICU4XCollatorCaseLevel._(_underlying.caseLevel);
  set caseLevel(ICU4XCollatorCaseLevel caseLevel) {
    _underlying.caseLevel = caseLevel._id;
  }

  ICU4XCollatorNumeric get numeric =>
      ICU4XCollatorNumeric._(_underlying.numeric);
  set numeric(ICU4XCollatorNumeric numeric) {
    _underlying.numeric = numeric._id;
  }

  ICU4XCollatorBackwardSecondLevel get backwardSecondLevel =>
      ICU4XCollatorBackwardSecondLevel._(_underlying.backwardSecondLevel);
  set backwardSecondLevel(
      ICU4XCollatorBackwardSecondLevel backwardSecondLevel) {
    _underlying.backwardSecondLevel = backwardSecondLevel._id;
  }

  @override
  bool operator ==(Object other) =>
      other is ICU4XCollatorOptionsV1 &&
      other._underlying.strength == _underlying.strength &&
      other._underlying.alternateHandling == _underlying.alternateHandling &&
      other._underlying.caseFirst == _underlying.caseFirst &&
      other._underlying.maxVariable == _underlying.maxVariable &&
      other._underlying.caseLevel == _underlying.caseLevel &&
      other._underlying.numeric == _underlying.numeric &&
      other._underlying.backwardSecondLevel == _underlying.backwardSecondLevel;

  @override
  int get hashCode => Object.hashAll([
        _underlying.strength,
        _underlying.alternateHandling,
        _underlying.caseFirst,
        _underlying.maxVariable,
        _underlying.caseLevel,
        _underlying.numeric,
        _underlying.backwardSecondLevel,
      ]);
}

/// See the [Rust documentation for `Strength`](https://docs.rs/icu/latest/icu/collator/enum.Strength.html) for more information.
enum ICU4XCollatorStrength {
  auto.__(0),
  primary.__(1),
  secondary.__(2),
  tertiary.__(3),
  quaternary.__(4),
  identical.__(5);

  const ICU4XCollatorStrength.__(this._id);

  // ignore: unused_element
  factory ICU4XCollatorStrength._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `ComposingNormalizer`](https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html) for more information.
class ICU4XComposingNormalizer implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XComposingNormalizer._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XComposingNormalizer_destroy'));

  /// Construct a new ICU4XComposingNormalizer instance for NFC
  ///
  /// See the [Rust documentation for `new_nfc`](https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.new_nfc) for more information.
  factory ICU4XComposingNormalizer.nfc(ICU4XDataProvider provider) {
    final result = _createNfcFfi(provider._underlying);
    return result.isOk
        ? ICU4XComposingNormalizer._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createNfcFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XComposingNormalizer_create_nfc')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Construct a new ICU4XComposingNormalizer instance for NFKC
  ///
  /// See the [Rust documentation for `new_nfkc`](https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.new_nfkc) for more information.
  factory ICU4XComposingNormalizer.nfkc(ICU4XDataProvider provider) {
    final result = _createNfkcFfi(provider._underlying);
    return result.isOk
        ? ICU4XComposingNormalizer._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createNfkcFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XComposingNormalizer_create_nfkc')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Normalize a (potentially ill-formed) UTF8 string
  ///
  /// Errors are mapped to REPLACEMENT CHARACTER
  ///
  /// See the [Rust documentation for `normalize_utf8`](https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.normalize_utf8) for more information.
  String normalize(String s) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final writeable = _Writeable();
    final result = _normalizeFfi(
        _underlying, sSlice._bytes, sSlice._length, writeable._underlying);
    alloc.releaseAll();
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _normalizeFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi2.Utf8>,
                      ffi.Size,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XComposingNormalizer_normalize')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Check if a (potentially ill-formed) UTF8 string is normalized
  ///
  /// Errors are mapped to REPLACEMENT CHARACTER
  ///
  /// See the [Rust documentation for `is_normalized_utf8`](https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html#method.is_normalized_utf8) for more information.
  bool isNormalized(String s) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final result = _isNormalizedFfi(_underlying, sSlice._bytes, sSlice._length);
    alloc.releaseAll();
    return result;
  }

  static final _isNormalizedFfi = _capi<
          ffi.NativeFunction<
              ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XComposingNormalizer_is_normalized')
      .asFunction<
          bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
              int)>(isLeaf: true);
}

/// See the [Rust documentation for `CustomTimeZone`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html) for more information.
class ICU4XCustomTimeZone implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XCustomTimeZone._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XCustomTimeZone_destroy'));

  /// Creates a time zone from an offset string.
  ///
  /// See the [Rust documentation for `from_str`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.from_str) for more information.
  factory ICU4XCustomTimeZone.fromString(String s) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final result = _createFromStringFfi(sSlice._bytes, sSlice._length);
    alloc.releaseAll();
    return result.isOk
        ? ICU4XCustomTimeZone._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromStringFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XCustomTimeZone_create_from_string')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi2.Utf8>, int)>(
          isLeaf: true);

  /// Creates a time zone with no information.
  ///
  /// See the [Rust documentation for `new_empty`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.new_empty) for more information.
  factory ICU4XCustomTimeZone.empty() {
    final result = _createEmptyFfi();
    return ICU4XCustomTimeZone._(result);
  }
  static final _createEmptyFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XCustomTimeZone_create_empty')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Creates a time zone for UTC.
  ///
  /// See the [Rust documentation for `utc`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.utc) for more information.
  factory ICU4XCustomTimeZone.utc() {
    final result = _createUtcFfi();
    return ICU4XCustomTimeZone._(result);
  }
  static final _createUtcFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XCustomTimeZone_create_utc')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Sets the `gmt_offset` field from offset seconds.
  ///
  /// Errors if the offset seconds are out of range.
  ///
  /// See the [Rust documentation for `try_from_offset_seconds`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.try_from_offset_seconds) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html)
  void trySetGmtOffsetSeconds(int offsetSeconds) {
    final result = _trySetGmtOffsetSecondsFfi(_underlying, offsetSeconds);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _trySetGmtOffsetSecondsFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int32)>>('ICU4XCustomTimeZone_try_set_gmt_offset_seconds')
      .asFunction<_ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>, int)>(
          isLeaf: true);

  /// Clears the `gmt_offset` field.
  ///
  /// See the [Rust documentation for `offset_seconds`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.offset_seconds) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html)
  void clearGmtOffset() {
    _clearGmtOffsetFfi(_underlying);
  }

  static final _clearGmtOffsetFfi =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XCustomTimeZone_clear_gmt_offset')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the value of the `gmt_offset` field as offset seconds.
  ///
  /// Errors if the `gmt_offset` field is empty.
  ///
  /// See the [Rust documentation for `offset_seconds`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.offset_seconds) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html)
  int get gmtOffsetSeconds {
    final result = _gmtOffsetSecondsFfi(_underlying);
    return result.isOk ? result.union.ok : throw ICU4XError._(result.union.err);
  }

  static final _gmtOffsetSecondsFfi = _capi<
              ffi.NativeFunction<
                  _ResultInt32Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCustomTimeZone_gmt_offset_seconds')
      .asFunction<_ResultInt32Uint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Returns whether the `gmt_offset` field is positive.
  ///
  /// Errors if the `gmt_offset` field is empty.
  ///
  /// See the [Rust documentation for `is_positive`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.is_positive) for more information.
  bool get isGmtOffsetPositive {
    final result = _isGmtOffsetPositiveFfi(_underlying);
    return result.isOk ? result.union.ok : throw ICU4XError._(result.union.err);
  }

  static final _isGmtOffsetPositiveFfi = _capi<
              ffi.NativeFunction<
                  _ResultBoolUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCustomTimeZone_is_gmt_offset_positive')
      .asFunction<_ResultBoolUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Returns whether the `gmt_offset` field is zero.
  ///
  /// Errors if the `gmt_offset` field is empty (which is not the same as zero).
  ///
  /// See the [Rust documentation for `is_zero`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.is_zero) for more information.
  bool get isGmtOffsetZero {
    final result = _isGmtOffsetZeroFfi(_underlying);
    return result.isOk ? result.union.ok : throw ICU4XError._(result.union.err);
  }

  static final _isGmtOffsetZeroFfi = _capi<
              ffi.NativeFunction<
                  _ResultBoolUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCustomTimeZone_is_gmt_offset_zero')
      .asFunction<_ResultBoolUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Returns whether the `gmt_offset` field has nonzero minutes.
  ///
  /// Errors if the `gmt_offset` field is empty.
  ///
  /// See the [Rust documentation for `has_minutes`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.has_minutes) for more information.
  bool gmtOffsetHasMinutes() {
    final result = _gmtOffsetHasMinutesFfi(_underlying);
    return result.isOk ? result.union.ok : throw ICU4XError._(result.union.err);
  }

  static final _gmtOffsetHasMinutesFfi = _capi<
              ffi.NativeFunction<
                  _ResultBoolUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCustomTimeZone_gmt_offset_has_minutes')
      .asFunction<_ResultBoolUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Returns whether the `gmt_offset` field has nonzero seconds.
  ///
  /// Errors if the `gmt_offset` field is empty.
  ///
  /// See the [Rust documentation for `has_seconds`](https://docs.rs/icu/latest/icu/timezone/struct.GmtOffset.html#method.has_seconds) for more information.
  bool gmtOffsetHasSeconds() {
    final result = _gmtOffsetHasSecondsFfi(_underlying);
    return result.isOk ? result.union.ok : throw ICU4XError._(result.union.err);
  }

  static final _gmtOffsetHasSecondsFfi = _capi<
              ffi.NativeFunction<
                  _ResultBoolUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCustomTimeZone_gmt_offset_has_seconds')
      .asFunction<_ResultBoolUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Sets the `time_zone_id` field from a BCP-47 string.
  ///
  /// Errors if the string is not a valid BCP-47 time zone ID.
  ///
  /// See the [Rust documentation for `time_zone_id`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneBcp47Id.html)
  void trySetTimeZoneId(String id) {
    final alloc = ffi2.Arena();
    final idSlice = _SliceFfi2Utf8._fromDart(id, alloc);

    final result =
        _trySetTimeZoneIdFfi(_underlying, idSlice._bytes, idSlice._length);
    alloc.releaseAll();
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _trySetTimeZoneIdFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XCustomTimeZone_try_set_time_zone_id')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Sets the `time_zone_id` field from an IANA string by looking up
  /// the corresponding BCP-47 string.
  ///
  /// Errors if the string is not a valid BCP-47 time zone ID.
  ///
  /// See the [Rust documentation for `get`](https://docs.rs/icu/latest/icu/timezone/struct.IanaToBcp47MapperBorrowed.html#method.get) for more information.
  void trySetIanaTimeZoneId(ICU4XIanaToBcp47Mapper mapper, String id) {
    final alloc = ffi2.Arena();
    final idSlice = _SliceFfi2Utf8._fromDart(id, alloc);

    final result = _trySetIanaTimeZoneIdFfi(
        _underlying, mapper._underlying, idSlice._bytes, idSlice._length);
    alloc.releaseAll();
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _trySetIanaTimeZoneIdFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XCustomTimeZone_try_set_iana_time_zone_id')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>,
              int)>(isLeaf: true);

  /// Clears the `time_zone_id` field.
  ///
  /// See the [Rust documentation for `time_zone_id`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneBcp47Id.html)
  void clearTimeZoneId() {
    _clearTimeZoneIdFfi(_underlying);
  }

  static final _clearTimeZoneIdFfi =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XCustomTimeZone_clear_time_zone_id')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Writes the value of the `time_zone_id` field as a string.
  ///
  /// Errors if the `time_zone_id` field is empty.
  ///
  /// See the [Rust documentation for `time_zone_id`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.time_zone_id) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.TimeZoneBcp47Id.html)
  String get timeZoneId {
    final writeable = _Writeable();
    final result = _timeZoneIdFfi(_underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _timeZoneIdFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCustomTimeZone_time_zone_id')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Sets the `metazone_id` field from a string.
  ///
  /// Errors if the string is not a valid BCP-47 metazone ID.
  ///
  /// See the [Rust documentation for `metazone_id`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.metazone_id) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.MetazoneId.html)
  void trySetMetazoneId(String id) {
    final alloc = ffi2.Arena();
    final idSlice = _SliceFfi2Utf8._fromDart(id, alloc);

    final result =
        _trySetMetazoneIdFfi(_underlying, idSlice._bytes, idSlice._length);
    alloc.releaseAll();
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _trySetMetazoneIdFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XCustomTimeZone_try_set_metazone_id')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Clears the `metazone_id` field.
  ///
  /// See the [Rust documentation for `metazone_id`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.metazone_id) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.MetazoneId.html)
  void clearMetazoneId() {
    _clearMetazoneIdFfi(_underlying);
  }

  static final _clearMetazoneIdFfi =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XCustomTimeZone_clear_metazone_id')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Writes the value of the `metazone_id` field as a string.
  ///
  /// Errors if the `metazone_id` field is empty.
  ///
  /// See the [Rust documentation for `metazone_id`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.metazone_id) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.MetazoneId.html)
  String get metazoneId {
    final writeable = _Writeable();
    final result = _metazoneIdFfi(_underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _metazoneIdFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCustomTimeZone_metazone_id')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Sets the `zone_variant` field from a string.
  ///
  /// Errors if the string is not a valid zone variant.
  ///
  /// See the [Rust documentation for `zone_variant`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html)
  void trySetZoneVariant(String id) {
    final alloc = ffi2.Arena();
    final idSlice = _SliceFfi2Utf8._fromDart(id, alloc);

    final result =
        _trySetZoneVariantFfi(_underlying, idSlice._bytes, idSlice._length);
    alloc.releaseAll();
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _trySetZoneVariantFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XCustomTimeZone_try_set_zone_variant')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Clears the `zone_variant` field.
  ///
  /// See the [Rust documentation for `zone_variant`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html)
  void clearZoneVariant() {
    _clearZoneVariantFfi(_underlying);
  }

  static final _clearZoneVariantFfi =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XCustomTimeZone_clear_zone_variant')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Writes the value of the `zone_variant` field as a string.
  ///
  /// Errors if the `zone_variant` field is empty.
  ///
  /// See the [Rust documentation for `zone_variant`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html)
  String get zoneVariant {
    final writeable = _Writeable();
    final result = _zoneVariantFfi(_underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _zoneVariantFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XCustomTimeZone_zone_variant')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Sets the `zone_variant` field to standard time.
  ///
  /// See the [Rust documentation for `standard`](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html#method.standard) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant)
  void setStandardTime() {
    _setStandardTimeFfi(_underlying);
  }

  static final _setStandardTimeFfi =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XCustomTimeZone_set_standard_time')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Sets the `zone_variant` field to daylight time.
  ///
  /// See the [Rust documentation for `daylight`](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html#method.daylight) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant)
  void setDaylightTime() {
    _setDaylightTimeFfi(_underlying);
  }

  static final _setDaylightTimeFfi =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XCustomTimeZone_set_daylight_time')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns whether the `zone_variant` field is standard time.
  ///
  /// Errors if the `zone_variant` field is empty.
  ///
  /// See the [Rust documentation for `standard`](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html#method.standard) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant)
  bool get isStandardTime {
    final result = _isStandardTimeFfi(_underlying);
    return result.isOk ? result.union.ok : throw ICU4XError._(result.union.err);
  }

  static final _isStandardTimeFfi = _capi<
              ffi.NativeFunction<
                  _ResultBoolUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCustomTimeZone_is_standard_time')
      .asFunction<_ResultBoolUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Returns whether the `zone_variant` field is daylight time.
  ///
  /// Errors if the `zone_variant` field is empty.
  ///
  /// See the [Rust documentation for `daylight`](https://docs.rs/icu/latest/icu/timezone/struct.ZoneVariant.html#method.daylight) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#structfield.zone_variant)
  bool get isDaylightTime {
    final result = _isDaylightTimeFfi(_underlying);
    return result.isOk ? result.union.ok : throw ICU4XError._(result.union.err);
  }

  static final _isDaylightTimeFfi = _capi<
              ffi.NativeFunction<
                  _ResultBoolUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCustomTimeZone_is_daylight_time')
      .asFunction<_ResultBoolUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Sets the metazone based on the time zone and the local timestamp.
  ///
  /// See the [Rust documentation for `maybe_calculate_metazone`](https://docs.rs/icu/latest/icu/timezone/struct.CustomTimeZone.html#method.maybe_calculate_metazone) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/timezone/struct.MetazoneCalculator.html#method.compute_metazone_from_time_zone)
  void maybeCalculateMetazone(ICU4XMetazoneCalculator metazoneCalculator,
      ICU4XIsoDateTime localDatetime) {
    _maybeCalculateMetazoneFfi(
        _underlying, metazoneCalculator._underlying, localDatetime._underlying);
  }

  static final _maybeCalculateMetazoneFfi = _capi<
              ffi.NativeFunction<
                  ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XCustomTimeZone_maybe_calculate_metazone')
      .asFunction<
          void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An ICU4X data provider, capable of loading ICU4X data keys from some source.
///
/// See the [Rust documentation for `icu_provider`](https://docs.rs/icu_provider/latest/icu_provider/index.html) for more information.
class ICU4XDataProvider implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XDataProvider._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XDataProvider_destroy'));

  /// Constructs an [`ICU4XDataProvider`] that uses compiled data.
  ///
  /// Requires the `compiled_data` feature.
  ///
  /// This provider cannot be modified or combined with other providers, so `enable_fallback`,
  /// `enabled_fallback_with`, `fork_by_locale`, and `fork_by_key` will return `Err`s.
  factory ICU4XDataProvider.compiled() {
    final result = _createCompiledFfi();
    return ICU4XDataProvider._(result);
  }
  static final _createCompiledFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XDataProvider_create_compiled')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Constructs an `FsDataProvider` and returns it as an [`ICU4XDataProvider`].
  /// Requires the `provider_fs` Cargo feature.
  /// Not supported in WASM.
  ///
  /// See the [Rust documentation for `FsDataProvider`](https://docs.rs/icu_provider_fs/latest/icu_provider_fs/struct.FsDataProvider.html) for more information.
  factory ICU4XDataProvider.fs(String path) {
    final alloc = ffi2.Arena();
    final pathSlice = _SliceFfi2Utf8._fromDart(path, alloc);

    final result = _createFsFfi(pathSlice._bytes, pathSlice._length);
    alloc.releaseAll();
    return result.isOk
        ? ICU4XDataProvider._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFsFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XDataProvider_create_fs')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi2.Utf8>, int)>(
          isLeaf: true);

  /// Deprecated
  ///
  /// Use `create_compiled()`.
  factory ICU4XDataProvider.test() {
    final result = _createTestFfi();
    return ICU4XDataProvider._(result);
  }
  static final _createTestFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XDataProvider_create_test')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Constructs a `BlobDataProvider` and returns it as an [`ICU4XDataProvider`].
  ///
  /// See the [Rust documentation for `BlobDataProvider`](https://docs.rs/icu_provider_blob/latest/icu_provider_blob/struct.BlobDataProvider.html) for more information.
  factory ICU4XDataProvider.fromByteSlice(Uint8List blob) {
    final alloc = ffi2.Arena();
    final blobSlice = _SliceFfiUint8._fromDart(blob, alloc);

    final result = _createFromByteSliceFfi(blobSlice._bytes, blobSlice._length);
    alloc.releaseAll();
    return result.isOk
        ? ICU4XDataProvider._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromByteSliceFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Uint8>,
                  ffi.Size)>>('ICU4XDataProvider_create_from_byte_slice')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Uint8>, int)>(
          isLeaf: true);

  /// Constructs an empty [`ICU4XDataProvider`].
  ///
  /// See the [Rust documentation for `EmptyDataProvider`](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/empty/struct.EmptyDataProvider.html) for more information.
  factory ICU4XDataProvider.empty() {
    final result = _createEmptyFfi();
    return ICU4XDataProvider._(result);
  }
  static final _createEmptyFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XDataProvider_create_empty')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Creates a provider that tries the current provider and then, if the current provider
  /// doesn't support the data key, another provider `other`.
  ///
  /// This takes ownership of the `other` provider, leaving an empty provider in its place.
  ///
  /// The providers must be the same type (Any or Buffer). This condition is satisfied if
  /// both providers originate from the same constructor, such as `create_from_byte_slice`
  /// or `create_fs`. If the condition is not upheld, a runtime error occurs.
  ///
  /// See the [Rust documentation for `ForkByKeyProvider`](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fork/type.ForkByKeyProvider.html) for more information.
  void forkByKey(ICU4XDataProvider other) {
    final result = _forkByKeyFfi(_underlying, other._underlying);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _forkByKeyFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDataProvider_fork_by_key')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Same as `fork_by_key` but forks by locale instead of key.
  ///
  /// See the [Rust documentation for `MissingLocalePredicate`](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fork/predicates/struct.MissingLocalePredicate.html) for more information.
  void forkByLocale(ICU4XDataProvider other) {
    final result = _forkByLocaleFfi(_underlying, other._underlying);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _forkByLocaleFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDataProvider_fork_by_locale')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Enables locale fallbacking for data requests made to this provider.
  ///
  /// Note that the test provider (from `create_test`) already has fallbacking enabled.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html#method.try_new) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html)
  void enableLocaleFallback() {
    final result = _enableLocaleFallbackFfi(_underlying);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _enableLocaleFallbackFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDataProvider_enable_locale_fallback')
      .asFunction<_ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `new_with_fallbacker`](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html#method.new_with_fallbacker) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu_provider_adapters/latest/icu_provider_adapters/fallback/struct.LocaleFallbackProvider.html)
  void enableLocaleFallbackWith(ICU4XLocaleFallbacker fallbacker) {
    final result =
        _enableLocaleFallbackWithFfi(_underlying, fallbacker._underlying);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _enableLocaleFallbackWithFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDataProvider_enable_locale_fallback_with')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// A generic data struct to be used by ICU4X
///
/// This can be used to construct a StructDataProvider.
class ICU4XDataStruct implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XDataStruct._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XDataStruct_destroy'));

  /// Construct a new DecimalSymbolsV1 data struct.
  ///
  /// C++ users: All string arguments must be valid UTF8
  ///
  /// See the [Rust documentation for `DecimalSymbolsV1`](https://docs.rs/icu/latest/icu/decimal/provider/struct.DecimalSymbolsV1.html) for more information.
  factory ICU4XDataStruct.decimalSymbolsV1(
      String plusSignPrefix,
      String plusSignSuffix,
      String minusSignPrefix,
      String minusSignSuffix,
      String decimalSeparator,
      String groupingSeparator,
      int primaryGroupSize,
      int secondaryGroupSize,
      int minGroupSize,
      Uint32List digits) {
    final alloc = ffi2.Arena();
    final plusSignPrefixSlice = _SliceFfi2Utf8._fromDart(plusSignPrefix, alloc);
    final plusSignSuffixSlice = _SliceFfi2Utf8._fromDart(plusSignSuffix, alloc);
    final minusSignPrefixSlice =
        _SliceFfi2Utf8._fromDart(minusSignPrefix, alloc);
    final minusSignSuffixSlice =
        _SliceFfi2Utf8._fromDart(minusSignSuffix, alloc);
    final decimalSeparatorSlice =
        _SliceFfi2Utf8._fromDart(decimalSeparator, alloc);
    final groupingSeparatorSlice =
        _SliceFfi2Utf8._fromDart(groupingSeparator, alloc);
    final digitsSlice = _SliceFfiUint32._fromDart(digits, alloc);

    final result = _createDecimalSymbolsV1Ffi(
        plusSignPrefixSlice._bytes,
        plusSignPrefixSlice._length,
        plusSignSuffixSlice._bytes,
        plusSignSuffixSlice._length,
        minusSignPrefixSlice._bytes,
        minusSignPrefixSlice._length,
        minusSignSuffixSlice._bytes,
        minusSignSuffixSlice._length,
        decimalSeparatorSlice._bytes,
        decimalSeparatorSlice._length,
        groupingSeparatorSlice._bytes,
        groupingSeparatorSlice._length,
        primaryGroupSize,
        secondaryGroupSize,
        minGroupSize,
        digitsSlice._bytes,
        digitsSlice._length);
    alloc.releaseAll();
    return result.isOk
        ? ICU4XDataStruct._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createDecimalSymbolsV1Ffi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Uint8,
                  ffi.Uint8,
                  ffi.Uint8,
                  ffi.Pointer<ffi.Uint32>,
                  ffi.Size)>>('ICU4XDataStruct_create_decimal_symbols_v1')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi2.Utf8>,
              int,
              int,
              int,
              int,
              ffi.Pointer<ffi.Uint32>,
              int)>(isLeaf: true);
}

/// An ICU4X Date object capable of containing a date and time for any calendar.
///
/// See the [Rust documentation for `Date`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html) for more information.
class ICU4XDate implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XDate._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XDate_destroy'));

  /// Creates a new [`ICU4XDate`] representing the ISO date and time
  /// given but in a given calendar
  ///
  /// See the [Rust documentation for `new_from_iso`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.new_from_iso) for more information.
  factory ICU4XDate.fromIsoInCalendar(
      int year, int month, int day, ICU4XCalendar calendar) {
    final result =
        _createFromIsoInCalendarFfi(year, month, day, calendar._underlying);
    return result.isOk
        ? ICU4XDate._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromIsoInCalendarFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Int32, ffi.Uint8, ffi.Uint8,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDate_create_from_iso_in_calendar')
      .asFunction<
          _ResultOpaqueUint32 Function(
              int, int, int, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Creates a new [`ICU4XDate`] from the given codes, which are interpreted in the given calendar system
  ///
  /// See the [Rust documentation for `try_new_from_codes`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.try_new_from_codes) for more information.
  factory ICU4XDate.fromCodesInCalendar(String eraCode, int year,
      String monthCode, int day, ICU4XCalendar calendar) {
    final alloc = ffi2.Arena();
    final eraCodeSlice = _SliceFfi2Utf8._fromDart(eraCode, alloc);
    final monthCodeSlice = _SliceFfi2Utf8._fromDart(monthCode, alloc);

    final result = _createFromCodesInCalendarFfi(
        eraCodeSlice._bytes,
        eraCodeSlice._length,
        year,
        monthCodeSlice._bytes,
        monthCodeSlice._length,
        day,
        calendar._underlying);
    alloc.releaseAll();
    return result.isOk
        ? ICU4XDate._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromCodesInCalendarFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi2.Utf8>,
                      ffi.Size,
                      ffi.Int32,
                      ffi.Pointer<ffi2.Utf8>,
                      ffi.Size,
                      ffi.Uint8,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDate_create_from_codes_in_calendar')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi2.Utf8>,
              int,
              int,
              ffi.Pointer<ffi2.Utf8>,
              int,
              int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Convert this date to one in a different calendar
  ///
  /// See the [Rust documentation for `to_calendar`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.to_calendar) for more information.
  ICU4XDate toCalendar(ICU4XCalendar calendar) {
    final result = _toCalendarFfi(_underlying, calendar._underlying);
    return ICU4XDate._(result);
  }

  static final _toCalendarFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDate_to_calendar')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Converts this date to ISO
  ///
  /// See the [Rust documentation for `to_iso`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.to_iso) for more information.
  ICU4XIsoDate toIso() {
    final result = _toIsoFfi(_underlying);
    return ICU4XIsoDate._(result);
  }

  static final _toIsoFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDate_to_iso')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Returns the 1-indexed day in the month for this date
  ///
  /// See the [Rust documentation for `day_of_month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_month) for more information.
  int get dayOfMonth {
    final result = _dayOfMonthFfi(_underlying);
    return result;
  }

  static final _dayOfMonthFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDate_day_of_month')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the day in the week for this day
  ///
  /// See the [Rust documentation for `day_of_week`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_week) for more information.
  ICU4XIsoWeekday get dayOfWeek {
    final result = _dayOfWeekFfi(_underlying);
    return ICU4XIsoWeekday._(result);
  }

  static final _dayOfWeekFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDate_day_of_week')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the week number in this month, 1-indexed, based on what
  /// is considered the first day of the week (often a locale preference).
  ///
  /// `first_weekday` can be obtained via `first_weekday()` on [`ICU4XWeekCalculator`]
  ///
  /// See the [Rust documentation for `week_of_month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.week_of_month) for more information.
  int weekOfMonth(ICU4XIsoWeekday firstWeekday) {
    final result = _weekOfMonthFfi(_underlying, firstWeekday._id);
    return result;
  }

  static final _weekOfMonthFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XDate_week_of_month')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Returns the week number in this year, using week data
  ///
  /// See the [Rust documentation for `week_of_year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.week_of_year) for more information.
  ICU4XWeekOf weekOfYear(ICU4XWeekCalculator calculator) {
    final result = _weekOfYearFfi(_underlying, calculator._underlying);
    return result.isOk
        ? ICU4XWeekOf._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }

  static final _weekOfYearFfi = _capi<
          ffi.NativeFunction<
              _ResultICU4XWeekOfFfiUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDate_week_of_year')
      .asFunction<
          _ResultICU4XWeekOfFfiUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns 1-indexed number of the month of this date in its year
  ///
  /// Note that for lunar calendars this may not lead to the same month
  /// having the same ordinal month across years; use month_code if you care
  /// about month identity.
  ///
  /// See the [Rust documentation for `month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.month) for more information.
  int get ordinalMonth {
    final result = _ordinalMonthFfi(_underlying);
    return result;
  }

  static final _ordinalMonthFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDate_ordinal_month')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the month code for this date. Typically something
  /// like "M01", "M02", but can be more complicated for lunar calendars.
  ///
  /// See the [Rust documentation for `month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.month) for more information.
  String get monthCode {
    final writeable = _Writeable();
    final result = _monthCodeFfi(_underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _monthCodeFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDate_month_code')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the year number in the current era for this date
  ///
  /// See the [Rust documentation for `year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.year) for more information.
  int get yearInEra {
    final result = _yearInEraFfi(_underlying);
    return result;
  }

  static final _yearInEraFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDate_year_in_era')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the era for this date,
  ///
  /// See the [Rust documentation for `year`](https://docs.rs/icu/latest/icu/struct.Date.html#method.year) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/types/struct.Era.html)
  String get era {
    final writeable = _Writeable();
    final result = _eraFfi(_underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _eraFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDate_era')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the number of months in the year represented by this date
  ///
  /// See the [Rust documentation for `months_in_year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.months_in_year) for more information.
  int get monthsInYear {
    final result = _monthsInYearFfi(_underlying);
    return result;
  }

  static final _monthsInYearFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDate_months_in_year')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the number of days in the month represented by this date
  ///
  /// See the [Rust documentation for `days_in_month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.days_in_month) for more information.
  int get daysInMonth {
    final result = _daysInMonthFfi(_underlying);
    return result;
  }

  static final _daysInMonthFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDate_days_in_month')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the number of days in the year represented by this date
  ///
  /// See the [Rust documentation for `days_in_year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.days_in_year) for more information.
  int get daysInYear {
    final result = _daysInYearFfi(_underlying);
    return result;
  }

  static final _daysInYearFfi =
      _capi<ffi.NativeFunction<ffi.Uint16 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDate_days_in_year')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the [`ICU4XCalendar`] object backing this date
  ///
  /// See the [Rust documentation for `calendar`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.calendar) for more information.
  ICU4XCalendar get calendar {
    final result = _calendarFfi(_underlying);
    return ICU4XCalendar._(result);
  }

  static final _calendarFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDate_calendar')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
}

/// An ICU4X DateFormatter object capable of formatting a [`ICU4XDate`] as a string,
/// using some calendar specified at runtime in the locale.
///
/// See the [Rust documentation for `DateFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html) for more information.
class ICU4XDateFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XDateFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XDateFormatter_destroy'));

  /// Creates a new [`ICU4XDateFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new_with_length`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.try_new_with_length) for more information.
  factory ICU4XDateFormatter.withLength(ICU4XDataProvider provider,
      ICU4XLocale locale, ICU4XDateLength dateLength) {
    final result = _createWithLengthFfi(
        provider._underlying, locale._underlying, dateLength._id);
    return result.isOk
        ? ICU4XDateFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithLengthFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XDateFormatter_create_with_length')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Formats a [`ICU4XDate`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.format) for more information.
  String formatDate(ICU4XDate value) {
    final writeable = _Writeable();
    final result =
        _formatDateFfi(_underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatDateFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDateFormatter_format_date')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Formats a [`ICU4XIsoDate`] to a string.
  ///
  /// Will convert to this formatter's calendar first
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.format) for more information.
  String formatIsoDate(ICU4XIsoDate value) {
    final writeable = _Writeable();
    final result = _formatIsoDateFfi(
        _underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatIsoDateFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDateFormatter_format_iso_date')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Formats a [`ICU4XDateTime`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.format) for more information.
  String formatDatetime(ICU4XDateTime value) {
    final writeable = _Writeable();
    final result = _formatDatetimeFfi(
        _underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatDatetimeFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDateFormatter_format_datetime')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Formats a [`ICU4XIsoDateTime`] to a string.
  ///
  /// Will convert to this formatter's calendar first
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateFormatter.html#method.format) for more information.
  String formatIsoDatetime(ICU4XIsoDateTime value) {
    final writeable = _Writeable();
    final result = _formatIsoDatetimeFfi(
        _underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatIsoDatetimeFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDateFormatter_format_iso_datetime')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `Date`](https://docs.rs/icu/latest/icu/datetime/options/length/enum.Date.html) for more information.
enum ICU4XDateLength {
  full.__(0),
  long.__(1),
  medium.__(2),
  short.__(3);

  const ICU4XDateLength.__(this._id);

  // ignore: unused_element
  factory ICU4XDateLength._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// An ICU4X DateTime object capable of containing a date and time for any calendar.
///
/// See the [Rust documentation for `DateTime`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html) for more information.
class ICU4XDateTime implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XDateTime._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XDateTime_destroy'));

  /// Creates a new [`ICU4XDateTime`] representing the ISO date and time
  /// given but in a given calendar
  ///
  /// See the [Rust documentation for `new_from_iso`](https://docs.rs/icu/latest/icu/struct.DateTime.html#method.new_from_iso) for more information.
  factory ICU4XDateTime.fromIsoInCalendar(
      int year,
      int month,
      int day,
      int hour,
      int minute,
      int second,
      int nanosecond,
      ICU4XCalendar calendar) {
    final result = _createFromIsoInCalendarFfi(year, month, day, hour, minute,
        second, nanosecond, calendar._underlying);
    return result.isOk
        ? ICU4XDateTime._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromIsoInCalendarFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Int32,
                      ffi.Uint8,
                      ffi.Uint8,
                      ffi.Uint8,
                      ffi.Uint8,
                      ffi.Uint8,
                      ffi.Uint32,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDateTime_create_from_iso_in_calendar')
      .asFunction<
          _ResultOpaqueUint32 Function(int, int, int, int, int, int, int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Creates a new [`ICU4XDateTime`] from the given codes, which are interpreted in the given calendar system
  ///
  /// See the [Rust documentation for `try_new_from_codes`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.try_new_from_codes) for more information.
  factory ICU4XDateTime.fromCodesInCalendar(
      String eraCode,
      int year,
      String monthCode,
      int day,
      int hour,
      int minute,
      int second,
      int nanosecond,
      ICU4XCalendar calendar) {
    final alloc = ffi2.Arena();
    final eraCodeSlice = _SliceFfi2Utf8._fromDart(eraCode, alloc);
    final monthCodeSlice = _SliceFfi2Utf8._fromDart(monthCode, alloc);

    final result = _createFromCodesInCalendarFfi(
        eraCodeSlice._bytes,
        eraCodeSlice._length,
        year,
        monthCodeSlice._bytes,
        monthCodeSlice._length,
        day,
        hour,
        minute,
        second,
        nanosecond,
        calendar._underlying);
    alloc.releaseAll();
    return result.isOk
        ? ICU4XDateTime._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromCodesInCalendarFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi2.Utf8>,
                      ffi.Size,
                      ffi.Int32,
                      ffi.Pointer<ffi2.Utf8>,
                      ffi.Size,
                      ffi.Uint8,
                      ffi.Uint8,
                      ffi.Uint8,
                      ffi.Uint8,
                      ffi.Uint32,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDateTime_create_from_codes_in_calendar')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi2.Utf8>,
              int,
              int,
              ffi.Pointer<ffi2.Utf8>,
              int,
              int,
              int,
              int,
              int,
              int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Creates a new [`ICU4XDateTime`] from an [`ICU4XDate`] and [`ICU4XTime`] object
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.new) for more information.
  factory ICU4XDateTime.fromDateAndTime(ICU4XDate date, ICU4XTime time) {
    final result =
        _createFromDateAndTimeFfi(date._underlying, time._underlying);
    return ICU4XDateTime._(result);
  }
  static final _createFromDateAndTimeFfi = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Opaque> Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDateTime_create_from_date_and_time')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Gets a copy of the date contained in this object
  ///
  /// See the [Rust documentation for `date`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#structfield.date) for more information.
  ICU4XDate get date {
    final result = _dateFfi(_underlying);
    return ICU4XDate._(result);
  }

  static final _dateFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDateTime_date')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Gets the time contained in this object
  ///
  /// See the [Rust documentation for `time`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#structfield.time) for more information.
  ICU4XTime get time {
    final result = _timeFfi(_underlying);
    return ICU4XTime._(result);
  }

  static final _timeFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDateTime_time')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Converts this date to ISO
  ///
  /// See the [Rust documentation for `to_iso`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.to_iso) for more information.
  ICU4XIsoDateTime toIso() {
    final result = _toIsoFfi(_underlying);
    return ICU4XIsoDateTime._(result);
  }

  static final _toIsoFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDateTime_to_iso')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Convert this datetime to one in a different calendar
  ///
  /// See the [Rust documentation for `to_calendar`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.to_calendar) for more information.
  ICU4XDateTime toCalendar(ICU4XCalendar calendar) {
    final result = _toCalendarFfi(_underlying, calendar._underlying);
    return ICU4XDateTime._(result);
  }

  static final _toCalendarFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDateTime_to_calendar')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the hour in this time
  ///
  /// See the [Rust documentation for `hour`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.hour) for more information.
  int get hour {
    final result = _hourFfi(_underlying);
    return result;
  }

  static final _hourFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDateTime_hour')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the minute in this time
  ///
  /// See the [Rust documentation for `minute`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.minute) for more information.
  int get minute {
    final result = _minuteFfi(_underlying);
    return result;
  }

  static final _minuteFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDateTime_minute')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the second in this time
  ///
  /// See the [Rust documentation for `second`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.second) for more information.
  int get second {
    final result = _secondFfi(_underlying);
    return result;
  }

  static final _secondFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDateTime_second')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the nanosecond in this time
  ///
  /// See the [Rust documentation for `nanosecond`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.nanosecond) for more information.
  int get nanosecond {
    final result = _nanosecondFfi(_underlying);
    return result;
  }

  static final _nanosecondFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDateTime_nanosecond')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the 1-indexed day in the month for this date
  ///
  /// See the [Rust documentation for `day_of_month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_month) for more information.
  int get dayOfMonth {
    final result = _dayOfMonthFfi(_underlying);
    return result;
  }

  static final _dayOfMonthFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDateTime_day_of_month')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the day in the week for this day
  ///
  /// See the [Rust documentation for `day_of_week`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_week) for more information.
  ICU4XIsoWeekday get dayOfWeek {
    final result = _dayOfWeekFfi(_underlying);
    return ICU4XIsoWeekday._(result);
  }

  static final _dayOfWeekFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDateTime_day_of_week')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the week number in this month, 1-indexed, based on what
  /// is considered the first day of the week (often a locale preference).
  ///
  /// `first_weekday` can be obtained via `first_weekday()` on [`ICU4XWeekCalculator`]
  ///
  /// See the [Rust documentation for `week_of_month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.week_of_month) for more information.
  int weekOfMonth(ICU4XIsoWeekday firstWeekday) {
    final result = _weekOfMonthFfi(_underlying, firstWeekday._id);
    return result;
  }

  static final _weekOfMonthFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XDateTime_week_of_month')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Returns the week number in this year, using week data
  ///
  /// See the [Rust documentation for `week_of_year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.week_of_year) for more information.
  ICU4XWeekOf weekOfYear(ICU4XWeekCalculator calculator) {
    final result = _weekOfYearFfi(_underlying, calculator._underlying);
    return result.isOk
        ? ICU4XWeekOf._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }

  static final _weekOfYearFfi = _capi<
          ffi.NativeFunction<
              _ResultICU4XWeekOfFfiUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDateTime_week_of_year')
      .asFunction<
          _ResultICU4XWeekOfFfiUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns 1-indexed number of the month of this date in its year
  ///
  /// Note that for lunar calendars this may not lead to the same month
  /// having the same ordinal month across years; use month_code if you care
  /// about month identity.
  ///
  /// See the [Rust documentation for `month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.month) for more information.
  int get ordinalMonth {
    final result = _ordinalMonthFfi(_underlying);
    return result;
  }

  static final _ordinalMonthFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDateTime_ordinal_month')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the month code for this date. Typically something
  /// like "M01", "M02", but can be more complicated for lunar calendars.
  ///
  /// See the [Rust documentation for `month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.month) for more information.
  String get monthCode {
    final writeable = _Writeable();
    final result = _monthCodeFfi(_underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _monthCodeFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDateTime_month_code')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the year number in the current era for this date
  ///
  /// See the [Rust documentation for `year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.year) for more information.
  int get yearInEra {
    final result = _yearInEraFfi(_underlying);
    return result;
  }

  static final _yearInEraFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDateTime_year_in_era')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the era for this date,
  ///
  /// See the [Rust documentation for `year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.year) for more information.
  String get era {
    final writeable = _Writeable();
    final result = _eraFfi(_underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _eraFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDateTime_era')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the number of months in the year represented by this date
  ///
  /// See the [Rust documentation for `months_in_year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.months_in_year) for more information.
  int get monthsInYear {
    final result = _monthsInYearFfi(_underlying);
    return result;
  }

  static final _monthsInYearFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDateTime_months_in_year')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the number of days in the month represented by this date
  ///
  /// See the [Rust documentation for `days_in_month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.days_in_month) for more information.
  int get daysInMonth {
    final result = _daysInMonthFfi(_underlying);
    return result;
  }

  static final _daysInMonthFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDateTime_days_in_month')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the number of days in the year represented by this date
  ///
  /// See the [Rust documentation for `days_in_year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.days_in_year) for more information.
  int get daysInYear {
    final result = _daysInYearFfi(_underlying);
    return result;
  }

  static final _daysInYearFfi =
      _capi<ffi.NativeFunction<ffi.Uint16 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XDateTime_days_in_year')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the [`ICU4XCalendar`] object backing this date
  ///
  /// See the [Rust documentation for `calendar`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.calendar) for more information.
  ICU4XCalendar get calendar {
    final result = _calendarFfi(_underlying);
    return ICU4XCalendar._(result);
  }

  static final _calendarFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XDateTime_calendar')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
}

/// An ICU4X DateFormatter object capable of formatting a [`ICU4XDateTime`] as a string,
/// using some calendar specified at runtime in the locale.
///
/// See the [Rust documentation for `DateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html) for more information.
class ICU4XDateTimeFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XDateTimeFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XDateTimeFormatter_destroy'));

  /// Creates a new [`ICU4XDateTimeFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.try_new) for more information.
  factory ICU4XDateTimeFormatter.withLengths(
      ICU4XDataProvider provider,
      ICU4XLocale locale,
      ICU4XDateLength dateLength,
      ICU4XTimeLength timeLength) {
    final result = _createWithLengthsFfi(provider._underlying,
        locale._underlying, dateLength._id, timeLength._id);
    return result.isOk
        ? ICU4XDateTimeFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithLengthsFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32,
                  ffi.Uint32)>>('ICU4XDateTimeFormatter_create_with_lengths')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, int, int)>(isLeaf: true);

  /// Formats a [`ICU4XDateTime`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.format) for more information.
  String formatDatetime(ICU4XDateTime value) {
    final writeable = _Writeable();
    final result = _formatDatetimeFfi(
        _underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatDatetimeFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDateTimeFormatter_format_datetime')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Formats a [`ICU4XIsoDateTime`] to a string.
  ///
  /// Will convert to this formatter's calendar first
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.DateTimeFormatter.html#method.format) for more information.
  String formatIsoDatetime(ICU4XIsoDateTime value) {
    final writeable = _Writeable();
    final result = _formatIsoDatetimeFfi(
        _underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatIsoDatetimeFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDateTimeFormatter_format_iso_datetime')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// The outcome of non-recursive canonical decomposition of a character.
/// `second` will be NUL when the decomposition expands to a single character
/// (which may or may not be the original one)
///
/// See the [Rust documentation for `Decomposed`](https://docs.rs/icu/latest/icu/normalizer/properties/enum.Decomposed.html) for more information.
class _ICU4XDecomposedFfi extends ffi.Struct {
  @ffi.Uint32()
  external int first;
  @ffi.Uint32()
  external int second;
}

class ICU4XDecomposed {
  final _ICU4XDecomposedFfi _underlying;

  // ignore: unused_element
  ICU4XDecomposed._(this._underlying);

  factory ICU4XDecomposed() {
    final pointer = ffi2.calloc<_ICU4XDecomposedFfi>();
    final result = ICU4XDecomposed._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  int get first => _underlying.first;
  set first(int first) {
    _underlying.first = first;
  }

  int get second => _underlying.second;
  set second(int second) {
    _underlying.second = second;
  }

  @override
  bool operator ==(Object other) =>
      other is ICU4XDecomposed &&
      other._underlying.first == _underlying.first &&
      other._underlying.second == _underlying.second;

  @override
  int get hashCode => Object.hashAll([
        _underlying.first,
        _underlying.second,
      ]);
}

/// See the [Rust documentation for `DecomposingNormalizer`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html) for more information.
class ICU4XDecomposingNormalizer implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XDecomposingNormalizer._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XDecomposingNormalizer_destroy'));

  /// Construct a new ICU4XDecomposingNormalizer instance for NFC
  ///
  /// See the [Rust documentation for `new_nfd`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.new_nfd) for more information.
  factory ICU4XDecomposingNormalizer.nfd(ICU4XDataProvider provider) {
    final result = _createNfdFfi(provider._underlying);
    return result.isOk
        ? ICU4XDecomposingNormalizer._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createNfdFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDecomposingNormalizer_create_nfd')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Construct a new ICU4XDecomposingNormalizer instance for NFKC
  ///
  /// See the [Rust documentation for `new_nfkd`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.new_nfkd) for more information.
  factory ICU4XDecomposingNormalizer.nfkd(ICU4XDataProvider provider) {
    final result = _createNfkdFfi(provider._underlying);
    return result.isOk
        ? ICU4XDecomposingNormalizer._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createNfkdFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDecomposingNormalizer_create_nfkd')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Normalize a (potentially ill-formed) UTF8 string
  ///
  /// Errors are mapped to REPLACEMENT CHARACTER
  ///
  /// See the [Rust documentation for `normalize_utf8`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.normalize_utf8) for more information.
  String normalize(String s) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final writeable = _Writeable();
    final result = _normalizeFfi(
        _underlying, sSlice._bytes, sSlice._length, writeable._underlying);
    alloc.releaseAll();
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _normalizeFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi2.Utf8>,
                      ffi.Size,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XDecomposingNormalizer_normalize')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Check if a (potentially ill-formed) UTF8 string is normalized
  ///
  /// Errors are mapped to REPLACEMENT CHARACTER
  ///
  /// See the [Rust documentation for `is_normalized_utf8`](https://docs.rs/icu/latest/icu/normalizer/struct.DecomposingNormalizer.html#method.is_normalized_utf8) for more information.
  bool isNormalized(String s) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final result = _isNormalizedFfi(_underlying, sSlice._bytes, sSlice._length);
    alloc.releaseAll();
    return result;
  }

  static final _isNormalizedFfi = _capi<
          ffi.NativeFunction<
              ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XDecomposingNormalizer_is_normalized')
      .asFunction<
          bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
              int)>(isLeaf: true);
}

/// See the [Rust documentation for `Fallback`](https://docs.rs/icu/latest/icu/displaynames/options/enum.Fallback.html) for more information.
enum ICU4XDisplayNamesFallback {
  code.__(0),
  none.__(1);

  const ICU4XDisplayNamesFallback.__(this._id);

  // ignore: unused_element
  factory ICU4XDisplayNamesFallback._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `DisplayNamesOptions`](https://docs.rs/icu/latest/icu/displaynames/options/struct.DisplayNamesOptions.html) for more information.
class _ICU4XDisplayNamesOptionsV1Ffi extends ffi.Struct {
  @ffi.Uint32()
  external int style;
  @ffi.Uint32()
  external int fallback;
  @ffi.Uint32()
  external int languageDisplay;
}

class ICU4XDisplayNamesOptionsV1 {
  final _ICU4XDisplayNamesOptionsV1Ffi _underlying;

  // ignore: unused_element
  ICU4XDisplayNamesOptionsV1._(this._underlying);

  factory ICU4XDisplayNamesOptionsV1() {
    final pointer = ffi2.calloc<_ICU4XDisplayNamesOptionsV1Ffi>();
    final result = ICU4XDisplayNamesOptionsV1._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  ICU4XDisplayNamesStyle get style =>
      ICU4XDisplayNamesStyle._(_underlying.style);
  set style(ICU4XDisplayNamesStyle style) {
    _underlying.style = style._id;
  }

  ICU4XDisplayNamesFallback get fallback =>
      ICU4XDisplayNamesFallback._(_underlying.fallback);
  set fallback(ICU4XDisplayNamesFallback fallback) {
    _underlying.fallback = fallback._id;
  }

  ICU4XLanguageDisplay get languageDisplay =>
      ICU4XLanguageDisplay._(_underlying.languageDisplay);
  set languageDisplay(ICU4XLanguageDisplay languageDisplay) {
    _underlying.languageDisplay = languageDisplay._id;
  }

  @override
  bool operator ==(Object other) =>
      other is ICU4XDisplayNamesOptionsV1 &&
      other._underlying.style == _underlying.style &&
      other._underlying.fallback == _underlying.fallback &&
      other._underlying.languageDisplay == _underlying.languageDisplay;

  @override
  int get hashCode => Object.hashAll([
        _underlying.style,
        _underlying.fallback,
        _underlying.languageDisplay,
      ]);
}

/// See the [Rust documentation for `Style`](https://docs.rs/icu/latest/icu/displaynames/options/enum.Style.html) for more information.
enum ICU4XDisplayNamesStyle {
  auto.__(0),
  narrow.__(1),
  short.__(2),
  long.__(3),
  menu.__(4);

  const ICU4XDisplayNamesStyle.__(this._id);

  // ignore: unused_element
  factory ICU4XDisplayNamesStyle._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// A common enum for errors that ICU4X may return, organized by API
///
/// The error names are stable and can be checked against as strings in the JS API
///
/// Additional information: [1](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.FixedDecimalError.html), [2](https://docs.rs/icu/latest/icu/calendar/enum.CalendarError.html), [3](https://docs.rs/icu/latest/icu/collator/enum.CollatorError.html), [4](https://docs.rs/icu/latest/icu/datetime/enum.DateTimeError.html), [5](https://docs.rs/icu/latest/icu/decimal/enum.DecimalError.html), [6](https://docs.rs/icu/latest/icu/list/enum.ListError.html), [7](https://docs.rs/icu/latest/icu/locid/enum.ParserError.html), [8](https://docs.rs/icu/latest/icu/locid_transform/enum.LocaleTransformError.html), [9](https://docs.rs/icu/latest/icu/normalizer/enum.NormalizerError.html), [10](https://docs.rs/icu/latest/icu/plurals/enum.PluralsError.html), [11](https://docs.rs/icu/latest/icu/properties/enum.PropertiesError.html), [12](https://docs.rs/icu/latest/icu/provider/struct.DataError.html), [13](https://docs.rs/icu/latest/icu/provider/enum.DataErrorKind.html), [14](https://docs.rs/icu/latest/icu/segmenter/enum.SegmenterError.html), [15](https://docs.rs/icu/latest/icu/timezone/enum.TimeZoneError.html)
enum ICU4XError {
  /// The error is not currently categorized as ICU4XError.
  /// Please file a bug
  unknownError.__(0),

  /// An error arising from writing to a string
  /// Typically found when not enough space is allocated
  /// Most APIs that return a string may return this error
  writeableError.__(1),
  outOfBoundsError.__(2),
  dataMissingDataKeyError.__(256),
  dataMissingVariantError.__(257),
  dataMissingLocaleError.__(258),
  dataNeedsVariantError.__(259),
  dataNeedsLocaleError.__(260),
  dataExtraneousLocaleError.__(261),
  dataFilteredResourceError.__(262),
  dataMismatchedTypeError.__(263),
  dataMissingPayloadError.__(264),
  dataInvalidStateError.__(265),
  dataCustomError.__(266),
  dataIoError.__(267),
  dataUnavailableBufferFormatError.__(268),
  dataMismatchedAnyBufferError.__(269),

  /// The subtag being requested was not set
  localeUndefinedSubtagError.__(512),

  /// The locale or subtag string failed to parse
  localeParserLanguageError.__(513),
  localeParserSubtagError.__(514),
  localeParserExtensionError.__(515),

  /// Attempted to construct an invalid data struct
  dataStructValidityError.__(768),
  propertyUnknownScriptIdError.__(1024),
  propertyUnknownGeneralCategoryGroupError.__(1025),
  propertyUnexpectedPropertyNameError.__(1026),
  fixedDecimalLimitError.__(1280),
  fixedDecimalSyntaxError.__(1281),
  pluralsParserError.__(1536),
  calendarParseError.__(1792),
  calendarOverflowError.__(1793),
  calendarUnderflowError.__(1794),
  calendarOutOfRangeError.__(1795),
  calendarUnknownEraError.__(1796),
  calendarUnknownMonthCodeError.__(1797),
  calendarMissingInputError.__(1798),
  calendarUnknownKindError.__(1799),
  calendarMissingError.__(1800),
  dateTimePatternError.__(2048),
  dateTimeMissingInputFieldError.__(2049),
  dateTimeSkeletonError.__(2050),
  dateTimeUnsupportedFieldError.__(2051),
  dateTimeUnsupportedOptionsError.__(2052),
  dateTimeMissingWeekdaySymbolError.__(2053),
  dateTimeMissingMonthSymbolError.__(2054),
  dateTimeFixedDecimalError.__(2055),
  dateTimeMismatchedCalendarError.__(2056),
  tinyStrTooLargeError.__(2304),
  tinyStrContainsNullError.__(2305),
  tinyStrNonAsciiError.__(2306),
  timeZoneOffsetOutOfBoundsError.__(2560),
  timeZoneInvalidOffsetError.__(2561),
  timeZoneMissingInputError.__(2562),
  timeZoneInvalidIdError.__(2563),
  normalizerFutureExtensionError.__(2816),
  normalizerValidationError.__(2817);

  const ICU4XError.__(this._id);

  // ignore: unused_element
  factory ICU4XError._(int id) => values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
class ICU4XFixedDecimal implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XFixedDecimal._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XFixedDecimal_destroy'));

  /// Construct an [`ICU4XFixedDecimal`] from an integer.
  ///
  /// See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
  factory ICU4XFixedDecimal.fromI32(int v) {
    final result = _createFromI32Ffi(v);
    return ICU4XFixedDecimal._(result);
  }
  static final _createFromI32Ffi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Int32)>>(
              'ICU4XFixedDecimal_create_from_i32')
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>(isLeaf: true);

  /// Construct an [`ICU4XFixedDecimal`] from an integer.
  ///
  /// See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
  factory ICU4XFixedDecimal.fromU32(int v) {
    final result = _createFromU32Ffi(v);
    return ICU4XFixedDecimal._(result);
  }
  static final _createFromU32Ffi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Uint32)>>(
              'ICU4XFixedDecimal_create_from_u32')
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>(isLeaf: true);

  /// Construct an [`ICU4XFixedDecimal`] from an integer.
  ///
  /// See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
  factory ICU4XFixedDecimal.fromI64(int v) {
    final result = _createFromI64Ffi(v);
    return ICU4XFixedDecimal._(result);
  }
  static final _createFromI64Ffi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Int64)>>(
              'ICU4XFixedDecimal_create_from_i64')
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>(isLeaf: true);

  /// Construct an [`ICU4XFixedDecimal`] from an integer.
  ///
  /// See the [Rust documentation for `FixedDecimal`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html) for more information.
  factory ICU4XFixedDecimal.fromU64(int v) {
    final result = _createFromU64Ffi(v);
    return ICU4XFixedDecimal._(result);
  }
  static final _createFromU64Ffi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Uint64)>>(
              'ICU4XFixedDecimal_create_from_u64')
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>(isLeaf: true);

  /// Construct an [`ICU4XFixedDecimal`] from an integer-valued float
  ///
  /// See the [Rust documentation for `try_from_f64`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.try_from_f64) for more information.
  ///
  /// See the [Rust documentation for `FloatPrecision`](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.FloatPrecision.html) for more information.
  factory ICU4XFixedDecimal.fromF64WithIntegerPrecision(double f) {
    final result = _createFromF64WithIntegerPrecisionFfi(f);
    return result.isOk
        ? ICU4XFixedDecimal._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromF64WithIntegerPrecisionFfi =
      _capi<ffi.NativeFunction<_ResultOpaqueUint32 Function(ffi.Double)>>(
              'ICU4XFixedDecimal_create_from_f64_with_integer_precision')
          .asFunction<_ResultOpaqueUint32 Function(double)>(isLeaf: true);

  /// Construct an [`ICU4XFixedDecimal`] from an float, with a given power of 10 for the lower magnitude
  ///
  /// See the [Rust documentation for `try_from_f64`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.try_from_f64) for more information.
  ///
  /// See the [Rust documentation for `FloatPrecision`](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.FloatPrecision.html) for more information.
  factory ICU4XFixedDecimal.fromF64WithLowerMagnitude(double f, int magnitude) {
    final result = _createFromF64WithLowerMagnitudeFfi(f, magnitude);
    return result.isOk
        ? ICU4XFixedDecimal._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromF64WithLowerMagnitudeFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Double, ffi.Int16)>>(
          'ICU4XFixedDecimal_create_from_f64_with_lower_magnitude')
      .asFunction<_ResultOpaqueUint32 Function(double, int)>(isLeaf: true);

  /// Construct an [`ICU4XFixedDecimal`] from an float, for a given number of significant digits
  ///
  /// See the [Rust documentation for `try_from_f64`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.try_from_f64) for more information.
  ///
  /// See the [Rust documentation for `FloatPrecision`](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.FloatPrecision.html) for more information.
  factory ICU4XFixedDecimal.fromF64WithSignificantDigits(double f, int digits) {
    final result = _createFromF64WithSignificantDigitsFfi(f, digits);
    return result.isOk
        ? ICU4XFixedDecimal._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromF64WithSignificantDigitsFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Double, ffi.Uint8)>>(
          'ICU4XFixedDecimal_create_from_f64_with_significant_digits')
      .asFunction<_ResultOpaqueUint32 Function(double, int)>(isLeaf: true);

  /// Construct an [`ICU4XFixedDecimal`] from an float, with enough digits to recover
  /// the original floating point in IEEE 754 without needing trailing zeros
  ///
  /// See the [Rust documentation for `try_from_f64`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.try_from_f64) for more information.
  ///
  /// See the [Rust documentation for `FloatPrecision`](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.FloatPrecision.html) for more information.
  factory ICU4XFixedDecimal.fromF64WithFloatingPrecision(double f) {
    final result = _createFromF64WithFloatingPrecisionFfi(f);
    return result.isOk
        ? ICU4XFixedDecimal._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromF64WithFloatingPrecisionFfi =
      _capi<ffi.NativeFunction<_ResultOpaqueUint32 Function(ffi.Double)>>(
              'ICU4XFixedDecimal_create_from_f64_with_floating_precision')
          .asFunction<_ResultOpaqueUint32 Function(double)>(isLeaf: true);

  /// Construct an [`ICU4XFixedDecimal`] from a string.
  ///
  /// See the [Rust documentation for `from_str`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.from_str) for more information.
  factory ICU4XFixedDecimal.fromString(String v) {
    final alloc = ffi2.Arena();
    final vSlice = _SliceFfi2Utf8._fromDart(v, alloc);

    final result = _createFromStringFfi(vSlice._bytes, vSlice._length);
    alloc.releaseAll();
    return result.isOk
        ? ICU4XFixedDecimal._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromStringFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XFixedDecimal_create_from_string')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi2.Utf8>, int)>(
          isLeaf: true);

  /// See the [Rust documentation for `digit_at`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.digit_at) for more information.
  int digitAt(int magnitude) {
    final result = _digitAtFfi(_underlying, magnitude);
    return result;
  }

  static final _digitAtFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_digit_at')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `magnitude_range`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.magnitude_range) for more information.
  int get magnitudeStart {
    final result = _magnitudeStartFfi(_underlying);
    return result;
  }

  static final _magnitudeStartFfi =
      _capi<ffi.NativeFunction<ffi.Int16 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XFixedDecimal_magnitude_start')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `magnitude_range`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.magnitude_range) for more information.
  int get magnitudeEnd {
    final result = _magnitudeEndFfi(_underlying);
    return result;
  }

  static final _magnitudeEndFfi =
      _capi<ffi.NativeFunction<ffi.Int16 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XFixedDecimal_magnitude_end')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `nonzero_magnitude_start`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.nonzero_magnitude_start) for more information.
  int get nonzeroMagnitudeStart {
    final result = _nonzeroMagnitudeStartFfi(_underlying);
    return result;
  }

  static final _nonzeroMagnitudeStartFfi =
      _capi<ffi.NativeFunction<ffi.Int16 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XFixedDecimal_nonzero_magnitude_start')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `nonzero_magnitude_end`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.nonzero_magnitude_end) for more information.
  int get nonzeroMagnitudeEnd {
    final result = _nonzeroMagnitudeEndFfi(_underlying);
    return result;
  }

  static final _nonzeroMagnitudeEndFfi =
      _capi<ffi.NativeFunction<ffi.Int16 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XFixedDecimal_nonzero_magnitude_end')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `is_zero`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.is_zero) for more information.
  bool get isZero {
    final result = _isZeroFfi(_underlying);
    return result;
  }

  static final _isZeroFfi =
      _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XFixedDecimal_is_zero')
          .asFunction<bool Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Multiply the [`ICU4XFixedDecimal`] by a given power of ten.
  ///
  /// See the [Rust documentation for `multiply_pow10`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.multiply_pow10) for more information.
  void multiplyPow10(int power) {
    _multiplyPow10Ffi(_underlying, power);
  }

  static final _multiplyPow10Ffi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_multiply_pow10')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `sign`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.sign) for more information.
  ICU4XFixedDecimalSign get sign {
    final result = _signFfi(_underlying);
    return ICU4XFixedDecimalSign._(result);
  }

  static final _signFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XFixedDecimal_sign')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Set the sign of the [`ICU4XFixedDecimal`].
  ///
  /// See the [Rust documentation for `set_sign`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.set_sign) for more information.
  set sign(ICU4XFixedDecimalSign sign) {
    _setSignFfi(_underlying, sign._id);
  }

  static final _setSignFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XFixedDecimal_set_sign')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `apply_sign_display`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.apply_sign_display) for more information.
  void applySignDisplay(ICU4XFixedDecimalSignDisplay signDisplay) {
    _applySignDisplayFfi(_underlying, signDisplay._id);
  }

  static final _applySignDisplayFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XFixedDecimal_apply_sign_display')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `trim_start`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.trim_start) for more information.
  void trimStart() {
    _trimStartFfi(_underlying);
  }

  static final _trimStartFfi =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XFixedDecimal_trim_start')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `trim_end`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.trim_end) for more information.
  void trimEnd() {
    _trimEndFfi(_underlying);
  }

  static final _trimEndFfi =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XFixedDecimal_trim_end')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Zero-pad the [`ICU4XFixedDecimal`] on the left to a particular position
  ///
  /// See the [Rust documentation for `pad_start`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.pad_start) for more information.
  void padStart(int position) {
    _padStartFfi(_underlying, position);
  }

  static final _padStartFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_pad_start')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Zero-pad the [`ICU4XFixedDecimal`] on the right to a particular position
  ///
  /// See the [Rust documentation for `pad_end`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.pad_end) for more information.
  void padEnd(int position) {
    _padEndFfi(_underlying, position);
  }

  static final _padEndFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_pad_end')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Truncate the [`ICU4XFixedDecimal`] on the left to a particular position, deleting digits if necessary. This is useful for, e.g. abbreviating years
  /// ("2022" -> "22")
  ///
  /// See the [Rust documentation for `set_max_position`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.set_max_position) for more information.
  void setMaxPosition(int position) {
    _setMaxPositionFfi(_underlying, position);
  }

  static final _setMaxPositionFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_set_max_position')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `trunc`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.trunc) for more information.
  void trunc(int position) {
    _truncFfi(_underlying, position);
  }

  static final _truncFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_trunc')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `trunc_to_increment`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.trunc_to_increment) for more information.
  void truncToIncrement(int position, ICU4XRoundingIncrement increment) {
    _truncToIncrementFfi(_underlying, position, increment._id);
  }

  static final _truncToIncrementFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Int16,
                  ffi.Uint32)>>('ICU4XFixedDecimal_trunc_to_increment')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int, int)>(
          isLeaf: true);

  /// See the [Rust documentation for `half_trunc`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.half_trunc) for more information.
  void halfTrunc(int position) {
    _halfTruncFfi(_underlying, position);
  }

  static final _halfTruncFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_half_trunc')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `expand`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.expand) for more information.
  void expand(int position) {
    _expandFfi(_underlying, position);
  }

  static final _expandFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_expand')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `expand_to_increment`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.expand_to_increment) for more information.
  void expandToIncrement(int position, ICU4XRoundingIncrement increment) {
    _expandToIncrementFfi(_underlying, position, increment._id);
  }

  static final _expandToIncrementFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Int16,
                  ffi.Uint32)>>('ICU4XFixedDecimal_expand_to_increment')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int, int)>(
          isLeaf: true);

  /// See the [Rust documentation for `half_expand`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.half_expand) for more information.
  void halfExpand(int position) {
    _halfExpandFfi(_underlying, position);
  }

  static final _halfExpandFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_half_expand')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `ceil`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.ceil) for more information.
  void ceil(int position) {
    _ceilFfi(_underlying, position);
  }

  static final _ceilFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_ceil')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `half_ceil`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.half_ceil) for more information.
  void halfCeil(int position) {
    _halfCeilFfi(_underlying, position);
  }

  static final _halfCeilFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_half_ceil')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `floor`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.floor) for more information.
  void floor(int position) {
    _floorFfi(_underlying, position);
  }

  static final _floorFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_floor')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `half_floor`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.half_floor) for more information.
  void halfFloor(int position) {
    _halfFloorFfi(_underlying, position);
  }

  static final _halfFloorFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_half_floor')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `half_even`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.half_even) for more information.
  void halfEven(int position) {
    _halfEvenFfi(_underlying, position);
  }

  static final _halfEvenFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Int16)>>('ICU4XFixedDecimal_half_even')
      .asFunction<void Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Concatenates `other` to the end of `self`.
  ///
  /// If successful, `other` will be set to 0 and a successful status is returned.
  ///
  /// If not successful, `other` will be unchanged and an error is returned.
  ///
  /// See the [Rust documentation for `concatenate_end`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.concatenate_end) for more information.
  void concatenateEnd(ICU4XFixedDecimal other) {
    final result = _concatenateEndFfi(_underlying, other._underlying);
    if (!result.isOk) {
      throw VoidError();
    }
  }

  static final _concatenateEndFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidVoid Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XFixedDecimal_concatenate_end')
      .asFunction<
          _ResultVoidVoid Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Format the [`ICU4XFixedDecimal`] as a string.
  ///
  /// See the [Rust documentation for `write_to`](https://docs.rs/fixed_decimal/latest/fixed_decimal/struct.FixedDecimal.html#method.write_to) for more information.
  @override
  String toString() {
    final writeable = _Writeable();
    _toStringFfi(_underlying, writeable._underlying);
    return writeable.finalize();
  }

  static final _toStringFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XFixedDecimal_to_string')
      .asFunction<
          void Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An ICU4X Fixed Decimal Format object, capable of formatting a [`ICU4XFixedDecimal`] as a string.
///
/// See the [Rust documentation for `FixedDecimalFormatter`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html) for more information.
class ICU4XFixedDecimalFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XFixedDecimalFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XFixedDecimalFormatter_destroy'));

  /// Creates a new [`ICU4XFixedDecimalFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.try_new) for more information.
  factory ICU4XFixedDecimalFormatter.withGroupingStrategy(
      ICU4XDataProvider provider,
      ICU4XLocale locale,
      ICU4XFixedDecimalGroupingStrategy groupingStrategy) {
    final result = _createWithGroupingStrategyFfi(
        provider._underlying, locale._underlying, groupingStrategy._id);
    return result.isOk
        ? ICU4XFixedDecimalFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithGroupingStrategyFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Uint32)>>(
          'ICU4XFixedDecimalFormatter_create_with_grouping_strategy')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Creates a new [`ICU4XFixedDecimalFormatter`] from preconstructed locale data in the form of an [`ICU4XDataStruct`]
  /// constructed from `ICU4XDataStruct::create_decimal_symbols()`.
  ///
  /// The contents of the data struct will be consumed: if you wish to use the struct again it will have to be reconstructed.
  /// Passing a consumed struct to this method will return an error.
  factory ICU4XFixedDecimalFormatter.withDecimalSymbolsV1(
      ICU4XDataStruct dataStruct,
      ICU4XFixedDecimalGroupingStrategy groupingStrategy) {
    final result = _createWithDecimalSymbolsV1Ffi(
        dataStruct._underlying, groupingStrategy._id);
    return result.isOk
        ? ICU4XFixedDecimalFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithDecimalSymbolsV1Ffi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Uint32)>>(
          'ICU4XFixedDecimalFormatter_create_with_decimal_symbols_v1')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>, int)>(
          isLeaf: true);

  /// Formats a [`ICU4XFixedDecimal`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/decimal/struct.FixedDecimalFormatter.html#method.format) for more information.
  String format(ICU4XFixedDecimal value) {
    final writeable = _Writeable();
    final result =
        _formatFfi(_underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XFixedDecimalFormatter_format')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `GroupingStrategy`](https://docs.rs/icu/latest/icu/decimal/options/enum.GroupingStrategy.html) for more information.
enum ICU4XFixedDecimalGroupingStrategy {
  auto.__(0),
  never.__(1),
  always.__(2),
  min2.__(3);

  const ICU4XFixedDecimalGroupingStrategy.__(this._id);

  // ignore: unused_element
  factory ICU4XFixedDecimalGroupingStrategy._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// The sign of a FixedDecimal, as shown in formatting.
///
/// See the [Rust documentation for `Sign`](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.Sign.html) for more information.
enum ICU4XFixedDecimalSign {
  /// No sign (implicitly positive, e.g., 1729).
  none.__(0),

  /// A negative sign, e.g., -1729.
  negative.__(1),

  /// An explicit positive sign, e.g., +1729.
  positive.__(2);

  const ICU4XFixedDecimalSign.__(this._id);

  // ignore: unused_element
  factory ICU4XFixedDecimalSign._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// ECMA-402 compatible sign display preference.
///
/// See the [Rust documentation for `SignDisplay`](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.SignDisplay.html) for more information.
enum ICU4XFixedDecimalSignDisplay {
  auto.__(0),
  never.__(1),
  always.__(2),
  exceptZero.__(3),
  negative.__(4);

  const ICU4XFixedDecimalSignDisplay.__(this._id);

  // ignore: unused_element
  factory ICU4XFixedDecimalSignDisplay._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// A type capable of looking up General Category mask values from a string name.
///
/// See the [Rust documentation for `get_name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.GeneralCategoryGroup.html#method.get_name_to_enum_mapper) for more information.
///
/// See the [Rust documentation for `PropertyValueNameToEnumMapper`](https://docs.rs/icu/latest/icu/properties/names/struct.PropertyValueNameToEnumMapper.html) for more information.
class ICU4XGeneralCategoryNameToMaskMapper implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XGeneralCategoryNameToMaskMapper._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(
      _capi('ICU4XGeneralCategoryNameToMaskMapper_destroy'));

  /// Get the mask value matching the given name, using strict matching
  ///
  /// Returns 0 if the name is unknown for this property
  int getStrict(String name) {
    final alloc = ffi2.Arena();
    final nameSlice = _SliceFfi2Utf8._fromDart(name, alloc);

    final result =
        _getStrictFfi(_underlying, nameSlice._bytes, nameSlice._length);
    alloc.releaseAll();
    return result;
  }

  static final _getStrictFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XGeneralCategoryNameToMaskMapper_get_strict')
      .asFunction<
          int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
              int)>(isLeaf: true);

  /// Get the mask value matching the given name, using loose matching
  ///
  /// Returns 0 if the name is unknown for this property
  int getLoose(String name) {
    final alloc = ffi2.Arena();
    final nameSlice = _SliceFfi2Utf8._fromDart(name, alloc);

    final result =
        _getLooseFfi(_underlying, nameSlice._bytes, nameSlice._length);
    alloc.releaseAll();
    return result;
  }

  static final _getLooseFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XGeneralCategoryNameToMaskMapper_get_loose')
      .asFunction<
          int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
              int)>(isLeaf: true);

  /// See the [Rust documentation for `get_name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.GeneralCategoryGroup.html#method.get_name_to_enum_mapper) for more information.
  factory ICU4XGeneralCategoryNameToMaskMapper.load(
      ICU4XDataProvider provider) {
    final result = _loadFfi(provider._underlying);
    return result.isOk
        ? ICU4XGeneralCategoryNameToMaskMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XGeneralCategoryNameToMaskMapper_load')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
}

/// See the [Rust documentation for `GraphemeClusterBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterBreakIterator.html) for more information.
class ICU4XGraphemeClusterBreakIteratorLatin1 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XGraphemeClusterBreakIteratorLatin1._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(
      _capi('ICU4XGraphemeClusterBreakIteratorLatin1_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterBreakIterator.html#method.next) for more information.
  int next() {
    final result = _nextFfi(_underlying);
    return result;
  }

  static final _nextFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XGraphemeClusterBreakIteratorLatin1_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `GraphemeClusterBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterBreakIterator.html) for more information.
class ICU4XGraphemeClusterBreakIteratorUtf16 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XGraphemeClusterBreakIteratorUtf16._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(
      _capi('ICU4XGraphemeClusterBreakIteratorUtf16_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterBreakIterator.html#method.next) for more information.
  int next() {
    final result = _nextFfi(_underlying);
    return result;
  }

  static final _nextFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XGraphemeClusterBreakIteratorUtf16_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `GraphemeClusterBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterBreakIterator.html) for more information.
class ICU4XGraphemeClusterBreakIteratorUtf8 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XGraphemeClusterBreakIteratorUtf8._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(
      _capi('ICU4XGraphemeClusterBreakIteratorUtf8_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterBreakIterator.html#method.next) for more information.
  int next() {
    final result = _nextFfi(_underlying);
    return result;
  }

  static final _nextFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XGraphemeClusterBreakIteratorUtf8_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An ICU4X grapheme-cluster-break segmenter, capable of finding grapheme cluster breakpoints
/// in strings.
///
/// See the [Rust documentation for `GraphemeClusterSegmenter`](https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterSegmenter.html) for more information.
class ICU4XGraphemeClusterSegmenter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XGraphemeClusterSegmenter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XGraphemeClusterSegmenter_destroy'));

  /// Construct an [`ICU4XGraphemeClusterSegmenter`].
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterSegmenter.html#method.new) for more information.
  factory ICU4XGraphemeClusterSegmenter(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XGraphemeClusterSegmenter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XGraphemeClusterSegmenter_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Segments a (potentially ill-formed) UTF-8 string.
  ///
  /// See the [Rust documentation for `segment_utf8`](https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterSegmenter.html#method.segment_utf8) for more information.
  ICU4XGraphemeClusterBreakIteratorUtf8 segmentUtf8(String input) {
    final alloc = ffi2.Arena();
    final inputSlice = _SliceFfi2Utf8._fromDart(input, alloc);

    final result =
        _segmentUtf8Ffi(_underlying, inputSlice._bytes, inputSlice._length);
    alloc.releaseAll();
    return ICU4XGraphemeClusterBreakIteratorUtf8._(result);
  }

  static final _segmentUtf8Ffi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XGraphemeClusterSegmenter_segment_utf8')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Segments a UTF-16 string.
  ///
  /// See the [Rust documentation for `segment_utf16`](https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterSegmenter.html#method.segment_utf16) for more information.
  ICU4XGraphemeClusterBreakIteratorUtf16 segmentUtf16(Uint16List input) {
    final alloc = ffi2.Arena();
    final inputSlice = _SliceFfiUint16._fromDart(input, alloc);

    final result =
        _segmentUtf16Ffi(_underlying, inputSlice._bytes, inputSlice._length);
    alloc.releaseAll();
    return ICU4XGraphemeClusterBreakIteratorUtf16._(result);
  }

  static final _segmentUtf16Ffi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Uint16>,
                  ffi.Size)>>('ICU4XGraphemeClusterSegmenter_segment_utf16')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Uint16>, int)>(isLeaf: true);

  /// Segments a Latin-1 string.
  ///
  /// See the [Rust documentation for `segment_latin1`](https://docs.rs/icu/latest/icu/segmenter/struct.GraphemeClusterSegmenter.html#method.segment_latin1) for more information.
  ICU4XGraphemeClusterBreakIteratorLatin1 segmentLatin1(Uint8List input) {
    final alloc = ffi2.Arena();
    final inputSlice = _SliceFfiUint8._fromDart(input, alloc);

    final result =
        _segmentLatin1Ffi(_underlying, inputSlice._bytes, inputSlice._length);
    alloc.releaseAll();
    return ICU4XGraphemeClusterBreakIteratorLatin1._(result);
  }

  static final _segmentLatin1Ffi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Uint8>,
                  ffi.Size)>>('ICU4XGraphemeClusterSegmenter_segment_latin1')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);
}

/// An ICU4X TypedDateFormatter object capable of formatting a [`ICU4XIsoDateTime`] as a string,
/// using the Gregorian Calendar.
///
/// See the [Rust documentation for `TypedDateFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.TypedDateFormatter.html) for more information.
class ICU4XGregorianDateFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XGregorianDateFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XGregorianDateFormatter_destroy'));

  /// Creates a new [`ICU4XGregorianDateFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new_with_length`](https://docs.rs/icu/latest/icu/datetime/struct.TypedDateFormatter.html#method.try_new_with_length) for more information.
  factory ICU4XGregorianDateFormatter.withLength(
      ICU4XDataProvider provider, ICU4XLocale locale, ICU4XDateLength length) {
    final result = _createWithLengthFfi(
        provider._underlying, locale._underlying, length._id);
    return result.isOk
        ? ICU4XGregorianDateFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithLengthFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Uint32)>>(
          'ICU4XGregorianDateFormatter_create_with_length')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Formats a [`ICU4XIsoDate`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.TypedDateFormatter.html#method.format) for more information.
  String formatIsoDate(ICU4XIsoDate value) {
    final writeable = _Writeable();
    final result = _formatIsoDateFfi(
        _underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatIsoDateFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XGregorianDateFormatter_format_iso_date')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Formats a [`ICU4XIsoDateTime`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.TypedDateFormatter.html#method.format) for more information.
  String formatIsoDatetime(ICU4XIsoDateTime value) {
    final writeable = _Writeable();
    final result = _formatIsoDatetimeFfi(
        _underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatIsoDatetimeFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XGregorianDateFormatter_format_iso_datetime')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An ICU4X TypedDateTimeFormatter object capable of formatting a [`ICU4XIsoDateTime`] as a string,
/// using the Gregorian Calendar.
///
/// See the [Rust documentation for `TypedDateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.TypedDateTimeFormatter.html) for more information.
class ICU4XGregorianDateTimeFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XGregorianDateTimeFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XGregorianDateTimeFormatter_destroy'));

  /// Creates a new [`ICU4XGregorianDateFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.TypedDateTimeFormatter.html#method.try_new) for more information.
  factory ICU4XGregorianDateTimeFormatter.withLengths(
      ICU4XDataProvider provider,
      ICU4XLocale locale,
      ICU4XDateLength dateLength,
      ICU4XTimeLength timeLength) {
    final result = _createWithLengthsFfi(provider._underlying,
        locale._underlying, dateLength._id, timeLength._id);
    return result.isOk
        ? ICU4XGregorianDateTimeFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithLengthsFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Uint32, ffi.Uint32)>>(
          'ICU4XGregorianDateTimeFormatter_create_with_lengths')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, int, int)>(isLeaf: true);

  /// Formats a [`ICU4XIsoDateTime`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.TypedDateTimeFormatter.html#method.format) for more information.
  String formatIsoDatetime(ICU4XIsoDateTime value) {
    final writeable = _Writeable();
    final result = _formatIsoDatetimeFfi(
        _underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatIsoDatetimeFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XGregorianDateTimeFormatter_format_iso_datetime')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An object capable of formatting a date time with time zone to a string.
///
/// See the [Rust documentation for `TypedZonedDateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.TypedZonedDateTimeFormatter.html) for more information.
class ICU4XGregorianZonedDateTimeFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XGregorianZonedDateTimeFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(
      _capi('ICU4XGregorianZonedDateTimeFormatter_destroy'));

  /// Creates a new [`ICU4XGregorianZonedDateTimeFormatter`] from locale data.
  ///
  /// This function has `date_length` and `time_length` arguments and uses default options
  /// for the time zone.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.TypedZonedDateTimeFormatter.html#method.try_new) for more information.
  factory ICU4XGregorianZonedDateTimeFormatter.withLengths(
      ICU4XDataProvider provider,
      ICU4XLocale locale,
      ICU4XDateLength dateLength,
      ICU4XTimeLength timeLength) {
    final result = _createWithLengthsFfi(provider._underlying,
        locale._underlying, dateLength._id, timeLength._id);
    return result.isOk
        ? ICU4XGregorianZonedDateTimeFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithLengthsFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Uint32, ffi.Uint32)>>(
          'ICU4XGregorianZonedDateTimeFormatter_create_with_lengths')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, int, int)>(isLeaf: true);

  /// Creates a new [`ICU4XGregorianZonedDateTimeFormatter`] from locale data.
  ///
  /// This function has `date_length` and `time_length` arguments and uses an ISO-8601 style
  /// fallback for the time zone with the given configurations.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.TypedZonedDateTimeFormatter.html#method.try_new) for more information.
  factory ICU4XGregorianZonedDateTimeFormatter.withLengthsAndIso8601TimeZoneFallback(
      ICU4XDataProvider provider,
      ICU4XLocale locale,
      ICU4XDateLength dateLength,
      ICU4XTimeLength timeLength,
      ICU4XIsoTimeZoneOptions zoneOptions) {
    final result = _createWithLengthsAndIso8601TimeZoneFallbackFfi(
        provider._underlying,
        locale._underlying,
        dateLength._id,
        timeLength._id,
        zoneOptions._underlying);
    return result.isOk
        ? ICU4XGregorianZonedDateTimeFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithLengthsAndIso8601TimeZoneFallbackFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Uint32,
                      ffi.Uint32,
                      _ICU4XIsoTimeZoneOptionsFfi)>>(
          'ICU4XGregorianZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              int,
              int,
              _ICU4XIsoTimeZoneOptionsFfi)>(isLeaf: true);

  /// Formats a [`ICU4XIsoDateTime`] and [`ICU4XCustomTimeZone`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.TypedZonedDateTimeFormatter.html#method.format) for more information.
  String formatIsoDatetimeWithCustomTimeZone(
      ICU4XIsoDateTime datetime, ICU4XCustomTimeZone timeZone) {
    final writeable = _Writeable();
    final result = _formatIsoDatetimeWithCustomTimeZoneFfi(_underlying,
        datetime._underlying, timeZone._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatIsoDatetimeWithCustomTimeZoneFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XGregorianZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An object capable of mapping from an IANA time zone ID to a BCP-47 ID.
///
/// This can be used via `try_set_iana_time_zone_id()` on [`ICU4XCustomTimeZone`].
///
/// [`ICU4XCustomTimeZone`]: crate::timezone::ffi::ICU4XCustomTimeZone
///
/// See the [Rust documentation for `IanaToBcp47Mapper`](https://docs.rs/icu/latest/icu/timezone/struct.IanaToBcp47Mapper.html) for more information.
class ICU4XIanaToBcp47Mapper implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XIanaToBcp47Mapper._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XIanaToBcp47Mapper_destroy'));

  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/timezone/struct.IanaToBcp47Mapper.html#method.new) for more information.
  factory ICU4XIanaToBcp47Mapper(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XIanaToBcp47Mapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XIanaToBcp47Mapper_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
}

/// An ICU4X Date object capable of containing a ISO-8601 date
///
/// See the [Rust documentation for `Date`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html) for more information.
class ICU4XIsoDate implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XIsoDate._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XIsoDate_destroy'));

  /// Creates a new [`ICU4XIsoDate`] from the specified date and time.
  ///
  /// See the [Rust documentation for `try_new_iso_date`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.try_new_iso_date) for more information.
  factory ICU4XIsoDate(int year, int month, int day) {
    final result = _createFfi(year, month, day);
    return result.isOk
        ? ICU4XIsoDate._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Int32, ffi.Uint8, ffi.Uint8)>>('ICU4XIsoDate_create')
      .asFunction<_ResultOpaqueUint32 Function(int, int, int)>(isLeaf: true);

  /// Creates a new [`ICU4XIsoDate`] representing January 1, 1970.
  ///
  /// See the [Rust documentation for `unix_epoch`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.unix_epoch) for more information.
  factory ICU4XIsoDate.forUnixEpoch() {
    final result = _createForUnixEpochFfi();
    return ICU4XIsoDate._(result);
  }
  static final _createForUnixEpochFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XIsoDate_create_for_unix_epoch')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Convert this date to one in a different calendar
  ///
  /// See the [Rust documentation for `to_calendar`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.to_calendar) for more information.
  ICU4XDate toCalendar(ICU4XCalendar calendar) {
    final result = _toCalendarFfi(_underlying, calendar._underlying);
    return ICU4XDate._(result);
  }

  static final _toCalendarFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XIsoDate_to_calendar')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `to_any`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.to_any) for more information.
  ICU4XDate toAny() {
    final result = _toAnyFfi(_underlying);
    return ICU4XDate._(result);
  }

  static final _toAnyFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XIsoDate_to_any')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Returns the 1-indexed day in the month for this date
  ///
  /// See the [Rust documentation for `day_of_month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_month) for more information.
  int get dayOfMonth {
    final result = _dayOfMonthFfi(_underlying);
    return result;
  }

  static final _dayOfMonthFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDate_day_of_month')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the day in the week for this day
  ///
  /// See the [Rust documentation for `day_of_week`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_week) for more information.
  ICU4XIsoWeekday get dayOfWeek {
    final result = _dayOfWeekFfi(_underlying);
    return ICU4XIsoWeekday._(result);
  }

  static final _dayOfWeekFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDate_day_of_week')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the week number in this month, 1-indexed, based on what
  /// is considered the first day of the week (often a locale preference).
  ///
  /// `first_weekday` can be obtained via `first_weekday()` on [`ICU4XWeekCalculator`]
  ///
  /// See the [Rust documentation for `week_of_month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.week_of_month) for more information.
  int weekOfMonth(ICU4XIsoWeekday firstWeekday) {
    final result = _weekOfMonthFfi(_underlying, firstWeekday._id);
    return result;
  }

  static final _weekOfMonthFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XIsoDate_week_of_month')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Returns the week number in this year, using week data
  ///
  /// See the [Rust documentation for `week_of_year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.week_of_year) for more information.
  ICU4XWeekOf weekOfYear(ICU4XWeekCalculator calculator) {
    final result = _weekOfYearFfi(_underlying, calculator._underlying);
    return result.isOk
        ? ICU4XWeekOf._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }

  static final _weekOfYearFfi = _capi<
          ffi.NativeFunction<
              _ResultICU4XWeekOfFfiUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XIsoDate_week_of_year')
      .asFunction<
          _ResultICU4XWeekOfFfiUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns 1-indexed number of the month of this date in its year
  ///
  /// See the [Rust documentation for `month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.month) for more information.
  int get month {
    final result = _monthFfi(_underlying);
    return result;
  }

  static final _monthFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDate_month')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the year number for this date
  ///
  /// See the [Rust documentation for `year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.year) for more information.
  int get year {
    final result = _yearFfi(_underlying);
    return result;
  }

  static final _yearFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDate_year')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the number of months in the year represented by this date
  ///
  /// See the [Rust documentation for `months_in_year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.months_in_year) for more information.
  int get monthsInYear {
    final result = _monthsInYearFfi(_underlying);
    return result;
  }

  static final _monthsInYearFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDate_months_in_year')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the number of days in the month represented by this date
  ///
  /// See the [Rust documentation for `days_in_month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.days_in_month) for more information.
  int get daysInMonth {
    final result = _daysInMonthFfi(_underlying);
    return result;
  }

  static final _daysInMonthFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDate_days_in_month')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the number of days in the year represented by this date
  ///
  /// See the [Rust documentation for `days_in_year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.days_in_year) for more information.
  int get daysInYear {
    final result = _daysInYearFfi(_underlying);
    return result;
  }

  static final _daysInYearFfi =
      _capi<ffi.NativeFunction<ffi.Uint16 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDate_days_in_year')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An ICU4X DateTime object capable of containing a ISO-8601 date and time.
///
/// See the [Rust documentation for `DateTime`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html) for more information.
class ICU4XIsoDateTime implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XIsoDateTime._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XIsoDateTime_destroy'));

  /// Creates a new [`ICU4XIsoDateTime`] from the specified date and time.
  ///
  /// See the [Rust documentation for `try_new_iso_datetime`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.try_new_iso_datetime) for more information.
  factory ICU4XIsoDateTime(int year, int month, int day, int hour, int minute,
      int second, int nanosecond) {
    final result =
        _createFfi(year, month, day, hour, minute, second, nanosecond);
    return result.isOk
        ? ICU4XIsoDateTime._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Int32,
                  ffi.Uint8,
                  ffi.Uint8,
                  ffi.Uint8,
                  ffi.Uint8,
                  ffi.Uint8,
                  ffi.Uint32)>>('ICU4XIsoDateTime_create')
      .asFunction<
          _ResultOpaqueUint32 Function(
              int, int, int, int, int, int, int)>(isLeaf: true);

  /// Creates a new [`ICU4XIsoDateTime`] from an [`ICU4XIsoDate`] and [`ICU4XTime`] object
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.new) for more information.
  factory ICU4XIsoDateTime.crateFromDateAndTime(
      ICU4XIsoDate date, ICU4XTime time) {
    final result = _crateFromDateAndTimeFfi(date._underlying, time._underlying);
    return ICU4XIsoDateTime._(result);
  }
  static final _crateFromDateAndTimeFfi = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Opaque> Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XIsoDateTime_crate_from_date_and_time')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Construct from the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)
  ///
  /// See the [Rust documentation for `from_minutes_since_local_unix_epoch`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.from_minutes_since_local_unix_epoch) for more information.
  factory ICU4XIsoDateTime.fromMinutesSinceLocalUnixEpoch(int minutes) {
    final result = _createFromMinutesSinceLocalUnixEpochFfi(minutes);
    return ICU4XIsoDateTime._(result);
  }
  static final _createFromMinutesSinceLocalUnixEpochFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Int32)>>(
              'ICU4XIsoDateTime_create_from_minutes_since_local_unix_epoch')
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>(isLeaf: true);

  /// Gets the date contained in this object
  ///
  /// See the [Rust documentation for `date`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#structfield.date) for more information.
  ICU4XIsoDate get date {
    final result = _dateFfi(_underlying);
    return ICU4XIsoDate._(result);
  }

  static final _dateFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XIsoDateTime_date')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Gets the time contained in this object
  ///
  /// See the [Rust documentation for `time`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#structfield.time) for more information.
  ICU4XTime get time {
    final result = _timeFfi(_underlying);
    return ICU4XTime._(result);
  }

  static final _timeFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XIsoDateTime_time')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Converts this to an [`ICU4XDateTime`] capable of being mixed with dates of
  /// other calendars
  ///
  /// See the [Rust documentation for `to_any`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.to_any) for more information.
  ICU4XDateTime toAny() {
    final result = _toAnyFfi(_underlying);
    return ICU4XDateTime._(result);
  }

  static final _toAnyFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XIsoDateTime_to_any')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Gets the minutes since the local unix epoch for this date (Jan 1 1970, 00:00)
  ///
  /// See the [Rust documentation for `minutes_since_local_unix_epoch`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.minutes_since_local_unix_epoch) for more information.
  int get minutesSinceLocalUnixEpoch {
    final result = _minutesSinceLocalUnixEpochFfi(_underlying);
    return result;
  }

  static final _minutesSinceLocalUnixEpochFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDateTime_minutes_since_local_unix_epoch')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Convert this datetime to one in a different calendar
  ///
  /// See the [Rust documentation for `to_calendar`](https://docs.rs/icu/latest/icu/calendar/struct.DateTime.html#method.to_calendar) for more information.
  ICU4XDateTime toCalendar(ICU4XCalendar calendar) {
    final result = _toCalendarFfi(_underlying, calendar._underlying);
    return ICU4XDateTime._(result);
  }

  static final _toCalendarFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XIsoDateTime_to_calendar')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the hour in this time
  ///
  /// See the [Rust documentation for `hour`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.hour) for more information.
  int get hour {
    final result = _hourFfi(_underlying);
    return result;
  }

  static final _hourFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDateTime_hour')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the minute in this time
  ///
  /// See the [Rust documentation for `minute`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.minute) for more information.
  int get minute {
    final result = _minuteFfi(_underlying);
    return result;
  }

  static final _minuteFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDateTime_minute')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the second in this time
  ///
  /// See the [Rust documentation for `second`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.second) for more information.
  int get second {
    final result = _secondFfi(_underlying);
    return result;
  }

  static final _secondFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDateTime_second')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the nanosecond in this time
  ///
  /// See the [Rust documentation for `nanosecond`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.nanosecond) for more information.
  int get nanosecond {
    final result = _nanosecondFfi(_underlying);
    return result;
  }

  static final _nanosecondFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDateTime_nanosecond')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the 1-indexed day in the month for this date
  ///
  /// See the [Rust documentation for `day_of_month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_month) for more information.
  int get dayOfMonth {
    final result = _dayOfMonthFfi(_underlying);
    return result;
  }

  static final _dayOfMonthFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDateTime_day_of_month')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the day in the week for this day
  ///
  /// See the [Rust documentation for `day_of_week`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.day_of_week) for more information.
  ICU4XIsoWeekday get dayOfWeek {
    final result = _dayOfWeekFfi(_underlying);
    return ICU4XIsoWeekday._(result);
  }

  static final _dayOfWeekFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDateTime_day_of_week')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the week number in this month, 1-indexed, based on what
  /// is considered the first day of the week (often a locale preference).
  ///
  /// `first_weekday` can be obtained via `first_weekday()` on [`ICU4XWeekCalculator`]
  ///
  /// See the [Rust documentation for `week_of_month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.week_of_month) for more information.
  int weekOfMonth(ICU4XIsoWeekday firstWeekday) {
    final result = _weekOfMonthFfi(_underlying, firstWeekday._id);
    return result;
  }

  static final _weekOfMonthFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XIsoDateTime_week_of_month')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Returns the week number in this year, using week data
  ///
  /// See the [Rust documentation for `week_of_year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.week_of_year) for more information.
  ICU4XWeekOf weekOfYear(ICU4XWeekCalculator calculator) {
    final result = _weekOfYearFfi(_underlying, calculator._underlying);
    return result.isOk
        ? ICU4XWeekOf._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }

  static final _weekOfYearFfi = _capi<
          ffi.NativeFunction<
              _ResultICU4XWeekOfFfiUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XIsoDateTime_week_of_year')
      .asFunction<
          _ResultICU4XWeekOfFfiUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns 1-indexed number of the month of this date in its year
  ///
  /// See the [Rust documentation for `month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.month) for more information.
  int get month {
    final result = _monthFfi(_underlying);
    return result;
  }

  static final _monthFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDateTime_month')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the year number for this date
  ///
  /// See the [Rust documentation for `year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.year) for more information.
  int get year {
    final result = _yearFfi(_underlying);
    return result;
  }

  static final _yearFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDateTime_year')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the number of months in the year represented by this date
  ///
  /// See the [Rust documentation for `months_in_year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.months_in_year) for more information.
  int get monthsInYear {
    final result = _monthsInYearFfi(_underlying);
    return result;
  }

  static final _monthsInYearFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDateTime_months_in_year')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the number of days in the month represented by this date
  ///
  /// See the [Rust documentation for `days_in_month`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.days_in_month) for more information.
  int get daysInMonth {
    final result = _daysInMonthFfi(_underlying);
    return result;
  }

  static final _daysInMonthFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDateTime_days_in_month')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the number of days in the year represented by this date
  ///
  /// See the [Rust documentation for `days_in_year`](https://docs.rs/icu/latest/icu/calendar/struct.Date.html#method.days_in_year) for more information.
  int get daysInYear {
    final result = _daysInYearFfi(_underlying);
    return result;
  }

  static final _daysInYearFfi =
      _capi<ffi.NativeFunction<ffi.Uint16 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XIsoDateTime_days_in_year')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `IsoFormat`](https://docs.rs/icu/latest/icu/datetime/time_zone/enum.IsoFormat.html) for more information.
enum ICU4XIsoTimeZoneFormat {
  basic.__(0),
  extended.__(1),
  utcBasic.__(2),
  utcExtended.__(3);

  const ICU4XIsoTimeZoneFormat.__(this._id);

  // ignore: unused_element
  factory ICU4XIsoTimeZoneFormat._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `IsoMinutes`](https://docs.rs/icu/latest/icu/datetime/time_zone/enum.IsoMinutes.html) for more information.
enum ICU4XIsoTimeZoneMinuteDisplay {
  required.__(0),
  optional.__(1);

  const ICU4XIsoTimeZoneMinuteDisplay.__(this._id);

  // ignore: unused_element
  factory ICU4XIsoTimeZoneMinuteDisplay._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

class _ICU4XIsoTimeZoneOptionsFfi extends ffi.Struct {
  @ffi.Uint32()
  external int format;
  @ffi.Uint32()
  external int minutes;
  @ffi.Uint32()
  external int seconds;
}

class ICU4XIsoTimeZoneOptions {
  final _ICU4XIsoTimeZoneOptionsFfi _underlying;

  // ignore: unused_element
  ICU4XIsoTimeZoneOptions._(this._underlying);

  factory ICU4XIsoTimeZoneOptions() {
    final pointer = ffi2.calloc<_ICU4XIsoTimeZoneOptionsFfi>();
    final result = ICU4XIsoTimeZoneOptions._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  ICU4XIsoTimeZoneFormat get format =>
      ICU4XIsoTimeZoneFormat._(_underlying.format);
  set format(ICU4XIsoTimeZoneFormat format) {
    _underlying.format = format._id;
  }

  ICU4XIsoTimeZoneMinuteDisplay get minutes =>
      ICU4XIsoTimeZoneMinuteDisplay._(_underlying.minutes);
  set minutes(ICU4XIsoTimeZoneMinuteDisplay minutes) {
    _underlying.minutes = minutes._id;
  }

  ICU4XIsoTimeZoneSecondDisplay get seconds =>
      ICU4XIsoTimeZoneSecondDisplay._(_underlying.seconds);
  set seconds(ICU4XIsoTimeZoneSecondDisplay seconds) {
    _underlying.seconds = seconds._id;
  }

  @override
  bool operator ==(Object other) =>
      other is ICU4XIsoTimeZoneOptions &&
      other._underlying.format == _underlying.format &&
      other._underlying.minutes == _underlying.minutes &&
      other._underlying.seconds == _underlying.seconds;

  @override
  int get hashCode => Object.hashAll([
        _underlying.format,
        _underlying.minutes,
        _underlying.seconds,
      ]);
}

/// See the [Rust documentation for `IsoSeconds`](https://docs.rs/icu/latest/icu/datetime/time_zone/enum.IsoSeconds.html) for more information.
enum ICU4XIsoTimeZoneSecondDisplay {
  optional.__(0),
  never.__(1);

  const ICU4XIsoTimeZoneSecondDisplay.__(this._id);

  // ignore: unused_element
  factory ICU4XIsoTimeZoneSecondDisplay._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

enum ICU4XIsoWeekday {
  monday.__(1),
  tuesday.__(2),
  wednesday.__(3),
  thursday.__(4),
  friday.__(5),
  saturday.__(6),
  sunday.__(7);

  const ICU4XIsoWeekday.__(this._id);

  // ignore: unused_element
  factory ICU4XIsoWeekday._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `LanguageDisplay`](https://docs.rs/icu/latest/icu/displaynames/options/enum.LanguageDisplay.html) for more information.
enum ICU4XLanguageDisplay {
  dialect.__(0),
  standard.__(1);

  const ICU4XLanguageDisplay.__(this._id);

  // ignore: unused_element
  factory ICU4XLanguageDisplay._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `LeadingAdjustment`](https://docs.rs/icu/latest/icu/casemap/titlecase/enum.LeadingAdjustment.html) for more information.
enum ICU4XLeadingAdjustment {
  auto.__(0),
  none.__(1),
  toCased.__(2);

  const ICU4XLeadingAdjustment.__(this._id);

  // ignore: unused_element
  factory ICU4XLeadingAdjustment._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `LineBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.LineBreakIterator.html) for more information.
///
/// Additional information: [1](https://docs.rs/icu/latest/icu/segmenter/type.LineBreakIteratorLatin1.html)
class ICU4XLineBreakIteratorLatin1 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLineBreakIteratorLatin1._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XLineBreakIteratorLatin1_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.LineBreakIterator.html#method.next) for more information.
  int next() {
    final result = _nextFfi(_underlying);
    return result;
  }

  static final _nextFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XLineBreakIteratorLatin1_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `LineBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.LineBreakIterator.html) for more information.
///
/// Additional information: [1](https://docs.rs/icu/latest/icu/segmenter/type.LineBreakIteratorUtf16.html)
class ICU4XLineBreakIteratorUtf16 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLineBreakIteratorUtf16._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XLineBreakIteratorUtf16_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.LineBreakIterator.html#method.next) for more information.
  int next() {
    final result = _nextFfi(_underlying);
    return result;
  }

  static final _nextFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XLineBreakIteratorUtf16_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `LineBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.LineBreakIterator.html) for more information.
///
/// Additional information: [1](https://docs.rs/icu/latest/icu/segmenter/type.LineBreakIteratorPotentiallyIllFormedUtf8.html)
class ICU4XLineBreakIteratorUtf8 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLineBreakIteratorUtf8._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XLineBreakIteratorUtf8_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.LineBreakIterator.html#method.next) for more information.
  int next() {
    final result = _nextFfi(_underlying);
    return result;
  }

  static final _nextFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XLineBreakIteratorUtf8_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `LineBreakOptions`](https://docs.rs/icu/latest/icu/segmenter/struct.LineBreakOptions.html) for more information.
class _ICU4XLineBreakOptionsV1Ffi extends ffi.Struct {
  @ffi.Uint32()
  external int strictness;
  @ffi.Uint32()
  external int wordOption;
  @ffi.Bool()
  external bool jaZh;
}

class ICU4XLineBreakOptionsV1 {
  final _ICU4XLineBreakOptionsV1Ffi _underlying;

  // ignore: unused_element
  ICU4XLineBreakOptionsV1._(this._underlying);

  factory ICU4XLineBreakOptionsV1() {
    final pointer = ffi2.calloc<_ICU4XLineBreakOptionsV1Ffi>();
    final result = ICU4XLineBreakOptionsV1._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  ICU4XLineBreakStrictness get strictness =>
      ICU4XLineBreakStrictness._(_underlying.strictness);
  set strictness(ICU4XLineBreakStrictness strictness) {
    _underlying.strictness = strictness._id;
  }

  ICU4XLineBreakWordOption get wordOption =>
      ICU4XLineBreakWordOption._(_underlying.wordOption);
  set wordOption(ICU4XLineBreakWordOption wordOption) {
    _underlying.wordOption = wordOption._id;
  }

  bool get jaZh => _underlying.jaZh;
  set jaZh(bool jaZh) {
    _underlying.jaZh = jaZh;
  }

  @override
  bool operator ==(Object other) =>
      other is ICU4XLineBreakOptionsV1 &&
      other._underlying.strictness == _underlying.strictness &&
      other._underlying.wordOption == _underlying.wordOption &&
      other._underlying.jaZh == _underlying.jaZh;

  @override
  int get hashCode => Object.hashAll([
        _underlying.strictness,
        _underlying.wordOption,
        _underlying.jaZh,
      ]);
}

/// See the [Rust documentation for `LineBreakStrictness`](https://docs.rs/icu/latest/icu/segmenter/enum.LineBreakStrictness.html) for more information.
enum ICU4XLineBreakStrictness {
  loose.__(0),
  normal.__(1),
  strict.__(2),
  anywhere.__(3);

  const ICU4XLineBreakStrictness.__(this._id);

  // ignore: unused_element
  factory ICU4XLineBreakStrictness._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `LineBreakWordOption`](https://docs.rs/icu/latest/icu/segmenter/enum.LineBreakWordOption.html) for more information.
enum ICU4XLineBreakWordOption {
  normal.__(0),
  breakAll.__(1),
  keepAll.__(2);

  const ICU4XLineBreakWordOption.__(this._id);

  // ignore: unused_element
  factory ICU4XLineBreakWordOption._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// An ICU4X line-break segmenter, capable of finding breakpoints in strings.
///
/// See the [Rust documentation for `LineSegmenter`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html) for more information.
class ICU4XLineSegmenter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLineSegmenter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XLineSegmenter_destroy'));

  /// Construct a [`ICU4XLineSegmenter`] with default options. It automatically loads the best
  /// available payload data for Burmese, Khmer, Lao, and Thai.
  ///
  /// See the [Rust documentation for `new_auto`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_auto) for more information.
  factory ICU4XLineSegmenter.auto(ICU4XDataProvider provider) {
    final result = _createAutoFfi(provider._underlying);
    return result.isOk
        ? ICU4XLineSegmenter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createAutoFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLineSegmenter_create_auto')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Construct a [`ICU4XLineSegmenter`] with default options and LSTM payload data for
  /// Burmese, Khmer, Lao, and Thai.
  ///
  /// See the [Rust documentation for `new_lstm`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_lstm) for more information.
  factory ICU4XLineSegmenter.lstm(ICU4XDataProvider provider) {
    final result = _createLstmFfi(provider._underlying);
    return result.isOk
        ? ICU4XLineSegmenter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createLstmFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLineSegmenter_create_lstm')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Construct a [`ICU4XLineSegmenter`] with default options and dictionary payload data for
  /// Burmese, Khmer, Lao, and Thai..
  ///
  /// See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary) for more information.
  factory ICU4XLineSegmenter.dictionary(ICU4XDataProvider provider) {
    final result = _createDictionaryFfi(provider._underlying);
    return result.isOk
        ? ICU4XLineSegmenter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createDictionaryFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XLineSegmenter_create_dictionary')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Construct a [`ICU4XLineSegmenter`] with custom options. It automatically loads the best
  /// available payload data for Burmese, Khmer, Lao, and Thai.
  ///
  /// See the [Rust documentation for `new_auto_with_options`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_auto_with_options) for more information.
  factory ICU4XLineSegmenter.autoWithOptionsV1(
      ICU4XDataProvider provider, ICU4XLineBreakOptionsV1 options) {
    final result =
        _createAutoWithOptionsV1Ffi(provider._underlying, options._underlying);
    return result.isOk
        ? ICU4XLineSegmenter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createAutoWithOptionsV1Ffi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, _ICU4XLineBreakOptionsV1Ffi)>>(
          'ICU4XLineSegmenter_create_auto_with_options_v1')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              _ICU4XLineBreakOptionsV1Ffi)>(isLeaf: true);

  /// Construct a [`ICU4XLineSegmenter`] with custom options and LSTM payload data for
  /// Burmese, Khmer, Lao, and Thai.
  ///
  /// See the [Rust documentation for `new_lstm_with_options`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_lstm_with_options) for more information.
  factory ICU4XLineSegmenter.lstmWithOptionsV1(
      ICU4XDataProvider provider, ICU4XLineBreakOptionsV1 options) {
    final result =
        _createLstmWithOptionsV1Ffi(provider._underlying, options._underlying);
    return result.isOk
        ? ICU4XLineSegmenter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createLstmWithOptionsV1Ffi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, _ICU4XLineBreakOptionsV1Ffi)>>(
          'ICU4XLineSegmenter_create_lstm_with_options_v1')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              _ICU4XLineBreakOptionsV1Ffi)>(isLeaf: true);

  /// Construct a [`ICU4XLineSegmenter`] with custom options and dictionary payload data for
  /// Burmese, Khmer, Lao, and Thai.
  ///
  /// See the [Rust documentation for `new_dictionary_with_options`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.new_dictionary_with_options) for more information.
  factory ICU4XLineSegmenter.dictionaryWithOptionsV1(
      ICU4XDataProvider provider, ICU4XLineBreakOptionsV1 options) {
    final result = _createDictionaryWithOptionsV1Ffi(
        provider._underlying, options._underlying);
    return result.isOk
        ? ICU4XLineSegmenter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createDictionaryWithOptionsV1Ffi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, _ICU4XLineBreakOptionsV1Ffi)>>(
          'ICU4XLineSegmenter_create_dictionary_with_options_v1')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              _ICU4XLineBreakOptionsV1Ffi)>(isLeaf: true);

  /// Segments a (potentially ill-formed) UTF-8 string.
  ///
  /// See the [Rust documentation for `segment_utf8`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.segment_utf8) for more information.
  ICU4XLineBreakIteratorUtf8 segmentUtf8(String input) {
    final alloc = ffi2.Arena();
    final inputSlice = _SliceFfi2Utf8._fromDart(input, alloc);

    final result =
        _segmentUtf8Ffi(_underlying, inputSlice._bytes, inputSlice._length);
    alloc.releaseAll();
    return ICU4XLineBreakIteratorUtf8._(result);
  }

  static final _segmentUtf8Ffi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XLineSegmenter_segment_utf8')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Segments a UTF-16 string.
  ///
  /// See the [Rust documentation for `segment_utf16`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.segment_utf16) for more information.
  ICU4XLineBreakIteratorUtf16 segmentUtf16(Uint16List input) {
    final alloc = ffi2.Arena();
    final inputSlice = _SliceFfiUint16._fromDart(input, alloc);

    final result =
        _segmentUtf16Ffi(_underlying, inputSlice._bytes, inputSlice._length);
    alloc.releaseAll();
    return ICU4XLineBreakIteratorUtf16._(result);
  }

  static final _segmentUtf16Ffi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Uint16>,
                  ffi.Size)>>('ICU4XLineSegmenter_segment_utf16')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Uint16>, int)>(isLeaf: true);

  /// Segments a Latin-1 string.
  ///
  /// See the [Rust documentation for `segment_latin1`](https://docs.rs/icu/latest/icu/segmenter/struct.LineSegmenter.html#method.segment_latin1) for more information.
  ICU4XLineBreakIteratorLatin1 segmentLatin1(Uint8List input) {
    final alloc = ffi2.Arena();
    final inputSlice = _SliceFfiUint8._fromDart(input, alloc);

    final result =
        _segmentLatin1Ffi(_underlying, inputSlice._bytes, inputSlice._length);
    alloc.releaseAll();
    return ICU4XLineBreakIteratorLatin1._(result);
  }

  static final _segmentLatin1Ffi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Uint8>,
                  ffi.Size)>>('ICU4XLineSegmenter_segment_latin1')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);
}

/// A list of strings
class ICU4XList implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XList._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XList_destroy'));

  /// Create a new list of strings
  factory ICU4XList() {
    final result = _createFfi();
    return ICU4XList._(result);
  }
  static final _createFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XList_create')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Create a new list of strings with preallocated space to hold
  /// at least `capacity` elements
  factory ICU4XList.withCapacity(int capacity) {
    final result = _createWithCapacityFfi(capacity);
    return ICU4XList._(result);
  }
  static final _createWithCapacityFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>>(
              'ICU4XList_create_with_capacity')
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>(isLeaf: true);

  /// Push a string to the list
  ///
  /// For C++ users, potentially invalid UTF8 will be handled via
  /// REPLACEMENT CHARACTERs
  void push(String val) {
    final alloc = ffi2.Arena();
    final valSlice = _SliceFfi2Utf8._fromDart(val, alloc);

    _pushFfi(_underlying, valSlice._bytes, valSlice._length);
    alloc.releaseAll();
  }

  static final _pushFfi = _capi<
          ffi.NativeFunction<
              ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XList_push')
      .asFunction<
          void Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
              int)>(isLeaf: true);

  /// The number of elements in this list
  int get len {
    final result = _lenFfi(_underlying);
    return result;
  }

  static final _lenFfi =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XList_len')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `ListFormatter`](https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html) for more information.
class ICU4XListFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XListFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XListFormatter_destroy'));

  /// Construct a new ICU4XListFormatter instance for And patterns
  ///
  /// See the [Rust documentation for `try_new_and_with_length`](https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.try_new_and_with_length) for more information.
  factory ICU4XListFormatter.andWithLength(
      ICU4XDataProvider provider, ICU4XLocale locale, ICU4XListLength length) {
    final result = _createAndWithLengthFfi(
        provider._underlying, locale._underlying, length._id);
    return result.isOk
        ? ICU4XListFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createAndWithLengthFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XListFormatter_create_and_with_length')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Construct a new ICU4XListFormatter instance for And patterns
  ///
  /// See the [Rust documentation for `try_new_or_with_length`](https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.try_new_or_with_length) for more information.
  factory ICU4XListFormatter.orWithLength(
      ICU4XDataProvider provider, ICU4XLocale locale, ICU4XListLength length) {
    final result = _createOrWithLengthFfi(
        provider._underlying, locale._underlying, length._id);
    return result.isOk
        ? ICU4XListFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createOrWithLengthFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XListFormatter_create_or_with_length')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Construct a new ICU4XListFormatter instance for And patterns
  ///
  /// See the [Rust documentation for `try_new_unit_with_length`](https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.try_new_unit_with_length) for more information.
  factory ICU4XListFormatter.unitWithLength(
      ICU4XDataProvider provider, ICU4XLocale locale, ICU4XListLength length) {
    final result = _createUnitWithLengthFfi(
        provider._underlying, locale._underlying, length._id);
    return result.isOk
        ? ICU4XListFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createUnitWithLengthFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XListFormatter_create_unit_with_length')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/list/struct.ListFormatter.html#method.format) for more information.
  String format(ICU4XList list) {
    final writeable = _Writeable();
    final result =
        _formatFfi(_underlying, list._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XListFormatter_format')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `ListLength`](https://docs.rs/icu/latest/icu/list/enum.ListLength.html) for more information.
enum ICU4XListLength {
  wide.__(0),
  short.__(1),
  narrow.__(2);

  const ICU4XListLength.__(this._id);

  // ignore: unused_element
  factory ICU4XListLength._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// An ICU4X Locale, capable of representing strings like `"en-US"`.
///
/// See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
class ICU4XLocale implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLocale._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XLocale_destroy'));

  /// Construct an [`ICU4XLocale`] from an locale identifier.
  ///
  /// This will run the complete locale parsing algorithm. If code size and
  /// performance are critical and the locale is of a known shape (such as
  /// `aa-BB`) use `create_und`, `set_language`, `set_script`, and `set_region`.
  ///
  /// See the [Rust documentation for `try_from_bytes`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.try_from_bytes) for more information.
  factory ICU4XLocale.fromString(String name) {
    final alloc = ffi2.Arena();
    final nameSlice = _SliceFfi2Utf8._fromDart(name, alloc);

    final result = _createFromStringFfi(nameSlice._bytes, nameSlice._length);
    alloc.releaseAll();
    return result.isOk
        ? ICU4XLocale._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromStringFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XLocale_create_from_string')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi2.Utf8>, int)>(
          isLeaf: true);

  /// Construct a default undefined [`ICU4XLocale`] "und".
  ///
  /// See the [Rust documentation for `UND`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#associatedconstant.UND) for more information.
  factory ICU4XLocale.und() {
    final result = _createUndFfi();
    return ICU4XLocale._(result);
  }
  static final _createUndFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XLocale_create_und')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Clones the [`ICU4XLocale`].
  ///
  /// See the [Rust documentation for `Locale`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html) for more information.
  ICU4XLocale clone() {
    final result = _cloneFfi(_underlying);
    return ICU4XLocale._(result);
  }

  static final _cloneFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_clone')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Write a string representation of the `LanguageIdentifier` part of
  /// [`ICU4XLocale`] to `write`.
  ///
  /// See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#structfield.id) for more information.
  String get basename {
    final writeable = _Writeable();
    final result = _basenameFfi(_underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _basenameFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_basename')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Write a string representation of the unicode extension to `write`
  ///
  /// See the [Rust documentation for `extensions`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#structfield.extensions) for more information.
  String getUnicodeExtension(String bytes) {
    final alloc = ffi2.Arena();
    final bytesSlice = _SliceFfi2Utf8._fromDart(bytes, alloc);

    final writeable = _Writeable();
    final result = _getUnicodeExtensionFfi(_underlying, bytesSlice._bytes,
        bytesSlice._length, writeable._underlying);
    alloc.releaseAll();
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _getUnicodeExtensionFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi2.Utf8>,
                      ffi.Size,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XLocale_get_unicode_extension')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Write a string representation of [`ICU4XLocale`] language to `write`
  ///
  /// See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#structfield.id) for more information.
  String get language {
    final writeable = _Writeable();
    final result = _languageFfi(_underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _languageFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_language')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Set the language part of the [`ICU4XLocale`].
  ///
  /// See the [Rust documentation for `try_from_bytes`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.try_from_bytes) for more information.
  set language(String bytes) {
    final alloc = ffi2.Arena();
    final bytesSlice = _SliceFfi2Utf8._fromDart(bytes, alloc);

    final result =
        _setLanguageFfi(_underlying, bytesSlice._bytes, bytesSlice._length);
    alloc.releaseAll();
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _setLanguageFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XLocale_set_language')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Write a string representation of [`ICU4XLocale`] region to `write`
  ///
  /// See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#structfield.id) for more information.
  String get region {
    final writeable = _Writeable();
    final result = _regionFfi(_underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _regionFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_region')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Set the region part of the [`ICU4XLocale`].
  ///
  /// See the [Rust documentation for `try_from_bytes`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.try_from_bytes) for more information.
  set region(String bytes) {
    final alloc = ffi2.Arena();
    final bytesSlice = _SliceFfi2Utf8._fromDart(bytes, alloc);

    final result =
        _setRegionFfi(_underlying, bytesSlice._bytes, bytesSlice._length);
    alloc.releaseAll();
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _setRegionFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>, ffi.Size)>>('ICU4XLocale_set_region')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Write a string representation of [`ICU4XLocale`] script to `write`
  ///
  /// See the [Rust documentation for `id`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#structfield.id) for more information.
  String get script {
    final writeable = _Writeable();
    final result = _scriptFfi(_underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _scriptFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_script')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Set the script part of the [`ICU4XLocale`]. Pass an empty string to remove the script.
  ///
  /// See the [Rust documentation for `try_from_bytes`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.try_from_bytes) for more information.
  set script(String bytes) {
    final alloc = ffi2.Arena();
    final bytesSlice = _SliceFfi2Utf8._fromDart(bytes, alloc);

    final result =
        _setScriptFfi(_underlying, bytesSlice._bytes, bytesSlice._length);
    alloc.releaseAll();
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _setScriptFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>, ffi.Size)>>('ICU4XLocale_set_script')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Best effort locale canonicalizer that doesn't need any data
  ///
  /// Use ICU4XLocaleCanonicalizer for better control and functionality
  ///
  /// See the [Rust documentation for `canonicalize`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.canonicalize) for more information.
  static String canonicalize(String bytes) {
    final alloc = ffi2.Arena();
    final bytesSlice = _SliceFfi2Utf8._fromDart(bytes, alloc);

    final writeable = _Writeable();
    final result = _canonicalizeFfi(
        bytesSlice._bytes, bytesSlice._length, writeable._underlying);
    alloc.releaseAll();
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _canonicalizeFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi2.Utf8>, ffi.Size,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_canonicalize')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi2.Utf8>, int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Write a string representation of [`ICU4XLocale`] to `write`
  ///
  /// See the [Rust documentation for `write_to`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.write_to) for more information.
  @override
  String toString() {
    final writeable = _Writeable();
    final result = _toStringFfi(_underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _toStringFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocale_to_string')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `normalizing_eq`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.normalizing_eq) for more information.
  bool normalizingEq(String other) {
    final alloc = ffi2.Arena();
    final otherSlice = _SliceFfi2Utf8._fromDart(other, alloc);

    final result =
        _normalizingEqFfi(_underlying, otherSlice._bytes, otherSlice._length);
    alloc.releaseAll();
    return result;
  }

  static final _normalizingEqFfi = _capi<
          ffi.NativeFunction<
              ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XLocale_normalizing_eq')
      .asFunction<
          bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
              int)>(isLeaf: true);

  /// See the [Rust documentation for `strict_cmp`](https://docs.rs/icu/latest/icu/locid/struct.Locale.html#method.strict_cmp) for more information.
  ICU4XOrdering strictCmp(String other) {
    final alloc = ffi2.Arena();
    final otherSlice = _SliceFfi2Utf8._fromDart(other, alloc);

    final result =
        _strictCmpFfi(_underlying, otherSlice._bytes, otherSlice._length);
    alloc.releaseAll();
    return ICU4XOrdering._(result);
  }

  static final _strictCmpFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>, ffi.Size)>>('ICU4XLocale_strict_cmp')
      .asFunction<
          int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
              int)>(isLeaf: true);

  /// Deprecated
  ///
  /// Use `create_from_string("en").
  factory ICU4XLocale.en() {
    final result = _createEnFfi();
    return ICU4XLocale._(result);
  }
  static final _createEnFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XLocale_create_en')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Deprecated
  ///
  /// Use `create_from_string("bn").
  factory ICU4XLocale.bn() {
    final result = _createBnFfi();
    return ICU4XLocale._(result);
  }
  static final _createBnFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XLocale_create_bn')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);
}

/// A locale canonicalizer.
///
/// See the [Rust documentation for `LocaleCanonicalizer`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleCanonicalizer.html) for more information.
class ICU4XLocaleCanonicalizer implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLocaleCanonicalizer._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XLocaleCanonicalizer_destroy'));

  /// Create a new [`ICU4XLocaleCanonicalizer`].
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleCanonicalizer.html#method.new) for more information.
  factory ICU4XLocaleCanonicalizer(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XLocaleCanonicalizer._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleCanonicalizer_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Create a new [`ICU4XLocaleCanonicalizer`] with extended data.
  ///
  /// See the [Rust documentation for `new_with_expander`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleCanonicalizer.html#method.new_with_expander) for more information.
  factory ICU4XLocaleCanonicalizer.extended(ICU4XDataProvider provider) {
    final result = _createExtendedFfi(provider._underlying);
    return result.isOk
        ? ICU4XLocaleCanonicalizer._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createExtendedFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XLocaleCanonicalizer_create_extended')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// FFI version of `LocaleCanonicalizer::canonicalize()`.
  ///
  /// See the [Rust documentation for `canonicalize`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleCanonicalizer.html#method.canonicalize) for more information.
  ICU4XTransformResult canonicalize(ICU4XLocale locale) {
    final result = _canonicalizeFfi(_underlying, locale._underlying);
    return ICU4XTransformResult._(result);
  }

  static final _canonicalizeFfi = _capi<
              ffi.NativeFunction<
                  ffi.Uint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XLocaleCanonicalizer_canonicalize')
      .asFunction<
          int Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `Direction`](https://docs.rs/icu/latest/icu/locid_transform/enum.Direction.html) for more information.
enum ICU4XLocaleDirection {
  leftToRight.__(0),
  rightToLeft.__(1),
  unknown.__(2);

  const ICU4XLocaleDirection.__(this._id);

  // ignore: unused_element
  factory ICU4XLocaleDirection._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `LocaleDirectionality`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleDirectionality.html) for more information.
class ICU4XLocaleDirectionality implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLocaleDirectionality._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XLocaleDirectionality_destroy'));

  /// Construct a new ICU4XLocaleDirectionality instance
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleDirectionality.html#method.new) for more information.
  factory ICU4XLocaleDirectionality(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XLocaleDirectionality._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleDirectionality_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Construct a new ICU4XLocaleDirectionality instance with a custom expander
  ///
  /// See the [Rust documentation for `new_with_expander`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleDirectionality.html#method.new_with_expander) for more information.
  factory ICU4XLocaleDirectionality.withExpander(
      ICU4XDataProvider provider, ICU4XLocaleExpander expander) {
    final result =
        _createWithExpanderFfi(provider._underlying, expander._underlying);
    return result.isOk
        ? ICU4XLocaleDirectionality._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithExpanderFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XLocaleDirectionality_create_with_expander')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `get`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleDirectionality.html#method.get) for more information.
  ICU4XLocaleDirection get(ICU4XLocale locale) {
    final result = _getFfi(_underlying, locale._underlying);
    return ICU4XLocaleDirection._(result);
  }

  static final _getFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleDirectionality_get')
      .asFunction<
          int Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `is_left_to_right`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleDirectionality.html#method.is_left_to_right) for more information.
  bool isLeftToRight(ICU4XLocale locale) {
    final result = _isLeftToRightFfi(_underlying, locale._underlying);
    return result;
  }

  static final _isLeftToRightFfi = _capi<
              ffi.NativeFunction<
                  ffi.Bool Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XLocaleDirectionality_is_left_to_right')
      .asFunction<
          bool Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `is_right_to_left`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleDirectionality.html#method.is_right_to_left) for more information.
  bool isRightToLeft(ICU4XLocale locale) {
    final result = _isRightToLeftFfi(_underlying, locale._underlying);
    return result;
  }

  static final _isRightToLeftFfi = _capi<
              ffi.NativeFunction<
                  ffi.Bool Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XLocaleDirectionality_is_right_to_left')
      .asFunction<
          bool Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `LocaleDisplayNamesFormatter`](https://docs.rs/icu/latest/icu/displaynames/struct.LocaleDisplayNamesFormatter.html) for more information.
class ICU4XLocaleDisplayNamesFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLocaleDisplayNamesFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XLocaleDisplayNamesFormatter_destroy'));

  /// Creates a new `LocaleDisplayNamesFormatter` from locale data and an options bag.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/displaynames/struct.LocaleDisplayNamesFormatter.html#method.try_new) for more information.
  factory ICU4XLocaleDisplayNamesFormatter.tryNew(ICU4XDataProvider provider,
      ICU4XLocale locale, ICU4XDisplayNamesOptionsV1 options) {
    final result = _tryNewFfi(
        provider._underlying, locale._underlying, options._underlying);
    return result.isOk
        ? ICU4XLocaleDisplayNamesFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _tryNewFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>,
                      _ICU4XDisplayNamesOptionsV1Ffi)>>(
          'ICU4XLocaleDisplayNamesFormatter_try_new')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              _ICU4XDisplayNamesOptionsV1Ffi)>(isLeaf: true);

  /// Returns the locale-specific display name of a locale.
  ///
  /// See the [Rust documentation for `of`](https://docs.rs/icu/latest/icu/displaynames/struct.LocaleDisplayNamesFormatter.html#method.of) for more information.
  String of(ICU4XLocale locale) {
    final writeable = _Writeable();
    final result =
        _ofFfi(_underlying, locale._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _ofFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XLocaleDisplayNamesFormatter_of')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// A locale expander.
///
/// See the [Rust documentation for `LocaleExpander`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleExpander.html) for more information.
class ICU4XLocaleExpander implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLocaleExpander._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XLocaleExpander_destroy'));

  /// Create a new [`ICU4XLocaleExpander`].
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleExpander.html#method.new) for more information.
  factory ICU4XLocaleExpander(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XLocaleExpander._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleExpander_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Create a new [`ICU4XLocaleExpander`] with extended data.
  ///
  /// See the [Rust documentation for `new_extended`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleExpander.html#method.new_extended) for more information.
  factory ICU4XLocaleExpander.extended(ICU4XDataProvider provider) {
    final result = _createExtendedFfi(provider._underlying);
    return result.isOk
        ? ICU4XLocaleExpander._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createExtendedFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XLocaleExpander_create_extended')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// FFI version of `LocaleExpander::maximize()`.
  ///
  /// See the [Rust documentation for `maximize`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleExpander.html#method.maximize) for more information.
  ICU4XTransformResult maximize(ICU4XLocale locale) {
    final result = _maximizeFfi(_underlying, locale._underlying);
    return ICU4XTransformResult._(result);
  }

  static final _maximizeFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleExpander_maximize')
      .asFunction<
          int Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// FFI version of `LocaleExpander::minimize()`.
  ///
  /// See the [Rust documentation for `minimize`](https://docs.rs/icu/latest/icu/locid_transform/struct.LocaleExpander.html#method.minimize) for more information.
  ICU4XTransformResult minimize(ICU4XLocale locale) {
    final result = _minimizeFfi(_underlying, locale._underlying);
    return ICU4XTransformResult._(result);
  }

  static final _minimizeFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleExpander_minimize')
      .asFunction<
          int Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// Collection of configurations for the ICU4X fallback algorithm.
///
/// See the [Rust documentation for `LocaleFallbackConfig`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbackConfig.html) for more information.
class _ICU4XLocaleFallbackConfigFfi extends ffi.Struct {
  @ffi.Uint32()
  external int priority;
  external _SliceFfi2Utf8 extensionKey;
  @ffi.Uint32()
  external int fallbackSupplement;
}

class ICU4XLocaleFallbackConfig {
  final _ICU4XLocaleFallbackConfigFfi _underlying;

  // ignore: unused_element
  ICU4XLocaleFallbackConfig._(this._underlying);

  factory ICU4XLocaleFallbackConfig() {
    final pointer = ffi2.calloc<_ICU4XLocaleFallbackConfigFfi>();
    final result = ICU4XLocaleFallbackConfig._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  ICU4XLocaleFallbackPriority get priority =>
      ICU4XLocaleFallbackPriority._(_underlying.priority);
  set priority(ICU4XLocaleFallbackPriority priority) {
    _underlying.priority = priority._id;
  }

  String get extensionKey => _underlying.extensionKey._asDart;
  set extensionKey(String extensionKey) {
    final alloc = ffi2.calloc;
    alloc.free(_underlying.extensionKey._bytes);
    final extensionKeySlice = _SliceFfi2Utf8._fromDart(extensionKey, alloc);
    _underlying.extensionKey = extensionKeySlice;
  }

  ICU4XLocaleFallbackSupplement get fallbackSupplement =>
      ICU4XLocaleFallbackSupplement._(_underlying.fallbackSupplement);
  set fallbackSupplement(ICU4XLocaleFallbackSupplement fallbackSupplement) {
    _underlying.fallbackSupplement = fallbackSupplement._id;
  }

  @override
  bool operator ==(Object other) =>
      other is ICU4XLocaleFallbackConfig &&
      other._underlying.priority == _underlying.priority &&
      other._underlying.extensionKey == _underlying.extensionKey &&
      other._underlying.fallbackSupplement == _underlying.fallbackSupplement;

  @override
  int get hashCode => Object.hashAll([
        _underlying.priority,
        _underlying.extensionKey,
        _underlying.fallbackSupplement,
      ]);
}

/// An iterator over the locale under fallback.
///
/// See the [Rust documentation for `LocaleFallbackIterator`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbackIterator.html) for more information.
class ICU4XLocaleFallbackIterator implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLocaleFallbackIterator._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XLocaleFallbackIterator_destroy'));

  /// Gets a snapshot of the current state of the locale.
  ///
  /// See the [Rust documentation for `get`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbackIterator.html#method.get) for more information.
  ICU4XLocale get get {
    final result = _getFfi(_underlying);
    return ICU4XLocale._(result);
  }

  static final _getFfi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleFallbackIterator_get')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Performs one step of the fallback algorithm, mutating the locale.
  ///
  /// See the [Rust documentation for `step`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbackIterator.html#method.step) for more information.
  void step() {
    _stepFfi(_underlying);
  }

  static final _stepFfi =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XLocaleFallbackIterator_step')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// Priority mode for the ICU4X fallback algorithm.
///
/// See the [Rust documentation for `LocaleFallbackPriority`](https://docs.rs/icu/latest/icu/locid_transform/fallback/enum.LocaleFallbackPriority.html) for more information.
enum ICU4XLocaleFallbackPriority {
  language.__(0),
  region.__(1),
  collation.__(2);

  const ICU4XLocaleFallbackPriority.__(this._id);

  // ignore: unused_element
  factory ICU4XLocaleFallbackPriority._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// What additional data is required to load when performing fallback.
///
/// See the [Rust documentation for `LocaleFallbackSupplement`](https://docs.rs/icu/latest/icu/locid_transform/fallback/enum.LocaleFallbackSupplement.html) for more information.
enum ICU4XLocaleFallbackSupplement {
  none.__(0),
  collation.__(1);

  const ICU4XLocaleFallbackSupplement.__(this._id);

  // ignore: unused_element
  factory ICU4XLocaleFallbackSupplement._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// An object that runs the ICU4X locale fallback algorithm.
///
/// See the [Rust documentation for `LocaleFallbacker`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbacker.html) for more information.
class ICU4XLocaleFallbacker implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLocaleFallbacker._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XLocaleFallbacker_destroy'));

  /// Creates a new `ICU4XLocaleFallbacker` from a data provider.
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbacker.html#method.new) for more information.
  factory ICU4XLocaleFallbacker(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XLocaleFallbacker._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XLocaleFallbacker_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Creates a new `ICU4XLocaleFallbacker` without data for limited functionality.
  ///
  /// See the [Rust documentation for `new_without_data`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbacker.html#method.new_without_data) for more information.
  factory ICU4XLocaleFallbacker.withoutData() {
    final result = _createWithoutDataFfi();
    return ICU4XLocaleFallbacker._(result);
  }
  static final _createWithoutDataFfi =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function()>>(
              'ICU4XLocaleFallbacker_create_without_data')
          .asFunction<ffi.Pointer<ffi.Opaque> Function()>(isLeaf: true);

  /// Associates this `ICU4XLocaleFallbacker` with configuration options.
  ///
  /// See the [Rust documentation for `for_config`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbacker.html#method.for_config) for more information.
  ICU4XLocaleFallbackerWithConfig forConfig(ICU4XLocaleFallbackConfig config) {
    final result = _forConfigFfi(_underlying, config._underlying);
    return result.isOk
        ? ICU4XLocaleFallbackerWithConfig._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }

  static final _forConfigFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, _ICU4XLocaleFallbackConfigFfi)>>(
          'ICU4XLocaleFallbacker_for_config')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              _ICU4XLocaleFallbackConfigFfi)>(isLeaf: true);
}

/// An object that runs the ICU4X locale fallback algorithm with specific configurations.
///
/// See the [Rust documentation for `LocaleFallbacker`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbacker.html) for more information.
///
/// See the [Rust documentation for `LocaleFallbackerWithConfig`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbackerWithConfig.html) for more information.
class ICU4XLocaleFallbackerWithConfig implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLocaleFallbackerWithConfig._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XLocaleFallbackerWithConfig_destroy'));

  /// Creates an iterator from a locale with each step of fallback.
  ///
  /// See the [Rust documentation for `fallback_for`](https://docs.rs/icu/latest/icu/locid_transform/fallback/struct.LocaleFallbacker.html#method.fallback_for) for more information.
  ICU4XLocaleFallbackIterator fallbackForLocale(ICU4XLocale locale) {
    final result = _fallbackForLocaleFfi(_underlying, locale._underlying);
    return ICU4XLocaleFallbackIterator._(result);
  }

  static final _fallbackForLocaleFfi = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Opaque> Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XLocaleFallbackerWithConfig_fallback_for_locale')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An object allowing control over the logging used
class ICU4XLogger implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XLogger._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XLogger_destroy'));

  /// Initialize the logger using `simple_logger`
  ///
  /// Requires the `simple_logger` Cargo feature.
  ///
  /// Returns `false` if there was already a logger set.
  static bool initSimpleLogger() {
    final result = _initSimpleLoggerFfi();
    return result;
  }

  static final _initSimpleLoggerFfi =
      _capi<ffi.NativeFunction<ffi.Bool Function()>>(
              'ICU4XLogger_init_simple_logger')
          .asFunction<bool Function()>(isLeaf: true);

  /// Deprecated: since ICU4X 1.4, this now happens automatically if the `log` feature is enabled.
  static bool initConsoleLogger() {
    final result = _initConsoleLoggerFfi();
    return result;
  }

  static final _initConsoleLoggerFfi =
      _capi<ffi.NativeFunction<ffi.Bool Function()>>(
              'ICU4XLogger_init_console_logger')
          .asFunction<bool Function()>(isLeaf: true);
}

/// An object capable of computing the metazone from a timezone.
///
/// This can be used via `maybe_calculate_metazone()` on [`ICU4XCustomTimeZone`].
///
/// [`ICU4XCustomTimeZone`]: crate::timezone::ffi::ICU4XCustomTimeZone
///
/// See the [Rust documentation for `MetazoneCalculator`](https://docs.rs/icu/latest/icu/timezone/struct.MetazoneCalculator.html) for more information.
class ICU4XMetazoneCalculator implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XMetazoneCalculator._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XMetazoneCalculator_destroy'));

  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/timezone/struct.MetazoneCalculator.html#method.new) for more information.
  factory ICU4XMetazoneCalculator(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XMetazoneCalculator._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XMetazoneCalculator_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
}

/// See the [Rust documentation for `Ordering`](https://docs.rs/core/latest/core/cmp/enum.Ordering.html) for more information.
enum ICU4XOrdering {
  less.__(-1),
  equal.__(0),
  greater.__(1);

  const ICU4XOrdering.__(this._id);

  // ignore: unused_element
  factory ICU4XOrdering._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// FFI version of `PluralRules::categories()` data.
class _ICU4XPluralCategoriesFfi extends ffi.Struct {
  @ffi.Bool()
  external bool zero;
  @ffi.Bool()
  external bool one;
  @ffi.Bool()
  external bool two;
  @ffi.Bool()
  external bool few;
  @ffi.Bool()
  external bool many;
  @ffi.Bool()
  external bool other;
}

class ICU4XPluralCategories {
  final _ICU4XPluralCategoriesFfi _underlying;

  // ignore: unused_element
  ICU4XPluralCategories._(this._underlying);

  factory ICU4XPluralCategories() {
    final pointer = ffi2.calloc<_ICU4XPluralCategoriesFfi>();
    final result = ICU4XPluralCategories._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  bool get zero => _underlying.zero;
  set zero(bool zero) {
    _underlying.zero = zero;
  }

  bool get one => _underlying.one;
  set one(bool one) {
    _underlying.one = one;
  }

  bool get two => _underlying.two;
  set two(bool two) {
    _underlying.two = two;
  }

  bool get few => _underlying.few;
  set few(bool few) {
    _underlying.few = few;
  }

  bool get many => _underlying.many;
  set many(bool many) {
    _underlying.many = many;
  }

  bool get other => _underlying.other;
  set other(bool other) {
    _underlying.other = other;
  }

  @override
  bool operator ==(Object other) =>
      other is ICU4XPluralCategories &&
      other._underlying.zero == _underlying.zero &&
      other._underlying.one == _underlying.one &&
      other._underlying.two == _underlying.two &&
      other._underlying.few == _underlying.few &&
      other._underlying.many == _underlying.many &&
      other._underlying.other == _underlying.other;

  @override
  int get hashCode => Object.hashAll([
        _underlying.zero,
        _underlying.one,
        _underlying.two,
        _underlying.few,
        _underlying.many,
        _underlying.other,
      ]);
}

/// FFI version of `PluralCategory`.
///
/// See the [Rust documentation for `PluralCategory`](https://docs.rs/icu/latest/icu/plurals/enum.PluralCategory.html) for more information.
enum ICU4XPluralCategory {
  zero.__(0),
  one.__(1),
  two.__(2),
  few.__(3),
  many.__(4),
  other.__(5);

  const ICU4XPluralCategory.__(this._id);

  // ignore: unused_element
  factory ICU4XPluralCategory._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;

  /// Construct from a string in the format
  /// [specified in TR35](https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules)
  ///
  /// See the [Rust documentation for `get_for_cldr_string`](https://docs.rs/icu/latest/icu/plurals/enum.PluralCategory.html#method.get_for_cldr_string) for more information.
  ///
  /// See the [Rust documentation for `get_for_cldr_bytes`](https://docs.rs/icu/latest/icu/plurals/enum.PluralCategory.html#method.get_for_cldr_bytes) for more information.
  factory ICU4XPluralCategory.getForCldrString(String s) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final result = _getForCldrStringFfi(sSlice._bytes, sSlice._length);
    alloc.releaseAll();
    return result.isOk
        ? ICU4XPluralCategory._(result.union.ok)
        : throw VoidError();
  }
  static final _getForCldrStringFfi = _capi<
          ffi.NativeFunction<
              _ResultUint32Void Function(ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XPluralCategory_get_for_cldr_string')
      .asFunction<_ResultUint32Void Function(ffi.Pointer<ffi2.Utf8>, int)>(
          isLeaf: true);
}

/// FFI version of `PluralOperands`.
///
/// See the [Rust documentation for `PluralOperands`](https://docs.rs/icu/latest/icu/plurals/struct.PluralOperands.html) for more information.
class ICU4XPluralOperands implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XPluralOperands._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XPluralOperands_destroy'));

  /// Construct for a given string representing a number
  ///
  /// See the [Rust documentation for `from_str`](https://docs.rs/icu/latest/icu/plurals/struct.PluralOperands.html#method.from_str) for more information.
  factory ICU4XPluralOperands.fromString(String s) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final result = _createFromStringFfi(sSlice._bytes, sSlice._length);
    alloc.releaseAll();
    return result.isOk
        ? ICU4XPluralOperands._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFromStringFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XPluralOperands_create_from_string')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi2.Utf8>, int)>(
          isLeaf: true);
}

/// FFI version of `PluralRules`.
///
/// See the [Rust documentation for `PluralRules`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html) for more information.
class ICU4XPluralRules implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XPluralRules._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XPluralRules_destroy'));

  /// Construct an [`ICU4XPluralRules`] for the given locale, for cardinal numbers
  ///
  /// See the [Rust documentation for `try_new_cardinal`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.try_new_cardinal) for more information.
  factory ICU4XPluralRules.cardinal(
      ICU4XDataProvider provider, ICU4XLocale locale) {
    final result = _createCardinalFfi(provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XPluralRules._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createCardinalFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XPluralRules_create_cardinal')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Construct an [`ICU4XPluralRules`] for the given locale, for ordinal numbers
  ///
  /// See the [Rust documentation for `try_new_ordinal`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.try_new_ordinal) for more information.
  factory ICU4XPluralRules.ordinal(
      ICU4XDataProvider provider, ICU4XLocale locale) {
    final result = _createOrdinalFfi(provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XPluralRules._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createOrdinalFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XPluralRules_create_ordinal')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Get the category for a given number represented as operands
  ///
  /// See the [Rust documentation for `category_for`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.category_for) for more information.
  ICU4XPluralCategory categoryFor(ICU4XPluralOperands op) {
    final result = _categoryForFfi(_underlying, op._underlying);
    return ICU4XPluralCategory._(result);
  }

  static final _categoryForFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XPluralRules_category_for')
      .asFunction<
          int Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Get all of the categories needed in the current locale
  ///
  /// See the [Rust documentation for `categories`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.categories) for more information.
  ICU4XPluralCategories get categories {
    final result = _categoriesFfi(_underlying);
    return ICU4XPluralCategories._(result);
  }

  static final _categoriesFfi = _capi<
          ffi.NativeFunction<
              _ICU4XPluralCategoriesFfi Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XPluralRules_categories')
      .asFunction<_ICU4XPluralCategoriesFfi Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
}

/// FFI version of `PluralRulesWithRanges`.
///
/// See the [Rust documentation for `PluralRulesWithRanges`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html) for more information.
class ICU4XPluralRulesWithRanges implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XPluralRulesWithRanges._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XPluralRulesWithRanges_destroy'));

  /// Construct an [`ICU4XPluralRulesWithRanges`] for the given locale, for cardinal numbers
  ///
  /// See the [Rust documentation for `try_new_cardinal`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.try_new_cardinal) for more information.
  factory ICU4XPluralRulesWithRanges.cardinal(
      ICU4XDataProvider provider, ICU4XLocale locale) {
    final result = _createCardinalFfi(provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XPluralRulesWithRanges._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createCardinalFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPluralRulesWithRanges_create_cardinal')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Construct an [`ICU4XPluralRulesWithRanges`] for the given locale, for ordinal numbers
  ///
  /// See the [Rust documentation for `try_new_ordinal`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.try_new_ordinal) for more information.
  factory ICU4XPluralRulesWithRanges.ordinal(
      ICU4XDataProvider provider, ICU4XLocale locale) {
    final result = _createOrdinalFfi(provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XPluralRulesWithRanges._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createOrdinalFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPluralRulesWithRanges_create_ordinal')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Get the category for a given number represented as operands
  ///
  /// See the [Rust documentation for `category_for`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.category_for) for more information.
  ICU4XPluralCategory categoryFor(ICU4XPluralOperands op) {
    final result = _categoryForFfi(_underlying, op._underlying);
    return ICU4XPluralCategory._(result);
  }

  static final _categoryForFfi = _capi<
              ffi.NativeFunction<
                  ffi.Uint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPluralRulesWithRanges_category_for')
      .asFunction<
          int Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Get all of the categories needed in the current locale
  ///
  /// See the [Rust documentation for `categories`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.categories) for more information.
  ICU4XPluralCategories get categories {
    final result = _categoriesFfi(_underlying);
    return ICU4XPluralCategories._(result);
  }

  static final _categoriesFfi = _capi<
              ffi.NativeFunction<
                  _ICU4XPluralCategoriesFfi Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPluralRulesWithRanges_categories')
      .asFunction<_ICU4XPluralCategoriesFfi Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Get the appropriate category for a numeric range.
  ///
  /// See the [Rust documentation for `category_for_range`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.category_for_range) for more information.
  ICU4XPluralCategory categoryForRange(
      ICU4XPluralOperands start, ICU4XPluralOperands end) {
    final result =
        _categoryForRangeFfi(_underlying, start._underlying, end._underlying);
    return ICU4XPluralCategory._(result);
  }

  static final _categoryForRangeFfi = _capi<
              ffi.NativeFunction<
                  ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPluralRulesWithRanges_category_for_range')
      .asFunction<
          int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Get the appropriate category for a numeric range from the categories of its endpoints.
  ///
  /// See the [Rust documentation for `resolve_range`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRulesWithRanges.html#method.resolve_range) for more information.
  ICU4XPluralCategory resolveRange(
      ICU4XPluralCategory start, ICU4XPluralCategory end) {
    final result = _resolveRangeFfi(_underlying, start._id, end._id);
    return ICU4XPluralCategory._(result);
  }

  static final _resolveRangeFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32,
                  ffi.Uint32)>>('ICU4XPluralRulesWithRanges_resolve_range')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int, int)>(
          isLeaf: true);
}

/// A type capable of looking up a property value from a string name.
///
/// See the [Rust documentation for `PropertyValueNameToEnumMapper`](https://docs.rs/icu/latest/icu/properties/names/struct.PropertyValueNameToEnumMapper.html) for more information.
///
/// See the [Rust documentation for `PropertyValueNameToEnumMapperBorrowed`](https://docs.rs/icu/latest/icu/properties/names/struct.PropertyValueNameToEnumMapperBorrowed.html) for more information.
class ICU4XPropertyValueNameToEnumMapper implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XPropertyValueNameToEnumMapper._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XPropertyValueNameToEnumMapper_destroy'));

  /// Get the property value matching the given name, using strict matching
  ///
  /// Returns -1 if the name is unknown for this property
  ///
  /// See the [Rust documentation for `get_strict`](https://docs.rs/icu/latest/icu/properties/names/struct.PropertyValueNameToEnumMapperBorrowed.html#method.get_strict) for more information.
  int getStrict(String name) {
    final alloc = ffi2.Arena();
    final nameSlice = _SliceFfi2Utf8._fromDart(name, alloc);

    final result =
        _getStrictFfi(_underlying, nameSlice._bytes, nameSlice._length);
    alloc.releaseAll();
    return result;
  }

  static final _getStrictFfi = _capi<
          ffi.NativeFunction<
              ffi.Int16 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XPropertyValueNameToEnumMapper_get_strict')
      .asFunction<
          int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
              int)>(isLeaf: true);

  /// Get the property value matching the given name, using loose matching
  ///
  /// Returns -1 if the name is unknown for this property
  ///
  /// See the [Rust documentation for `get_loose`](https://docs.rs/icu/latest/icu/properties/names/struct.PropertyValueNameToEnumMapperBorrowed.html#method.get_loose) for more information.
  int getLoose(String name) {
    final alloc = ffi2.Arena();
    final nameSlice = _SliceFfi2Utf8._fromDart(name, alloc);

    final result =
        _getLooseFfi(_underlying, nameSlice._bytes, nameSlice._length);
    alloc.releaseAll();
    return result;
  }

  static final _getLooseFfi = _capi<
          ffi.NativeFunction<
              ffi.Int16 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XPropertyValueNameToEnumMapper_get_loose')
      .asFunction<
          int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
              int)>(isLeaf: true);

  /// See the [Rust documentation for `get_name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.GeneralCategory.html#method.get_name_to_enum_mapper) for more information.
  factory ICU4XPropertyValueNameToEnumMapper.loadGeneralCategory(
      ICU4XDataProvider provider) {
    final result = _loadGeneralCategoryFfi(provider._underlying);
    return result.isOk
        ? ICU4XPropertyValueNameToEnumMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadGeneralCategoryFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPropertyValueNameToEnumMapper_load_general_category')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.BidiClass.html#method.name_to_enum_mapper) for more information.
  factory ICU4XPropertyValueNameToEnumMapper.loadBidiClass(
      ICU4XDataProvider provider) {
    final result = _loadBidiClassFfi(provider._underlying);
    return result.isOk
        ? ICU4XPropertyValueNameToEnumMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadBidiClassFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPropertyValueNameToEnumMapper_load_bidi_class')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.EastAsianWidth.html#method.name_to_enum_mapper) for more information.
  factory ICU4XPropertyValueNameToEnumMapper.loadEastAsianWidth(
      ICU4XDataProvider provider) {
    final result = _loadEastAsianWidthFfi(provider._underlying);
    return result.isOk
        ? ICU4XPropertyValueNameToEnumMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadEastAsianWidthFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPropertyValueNameToEnumMapper_load_east_asian_width')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.IndicSyllabicCategory.html#method.name_to_enum_mapper) for more information.
  factory ICU4XPropertyValueNameToEnumMapper.loadIndicSyllabicCategory(
      ICU4XDataProvider provider) {
    final result = _loadIndicSyllabicCategoryFfi(provider._underlying);
    return result.isOk
        ? ICU4XPropertyValueNameToEnumMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadIndicSyllabicCategoryFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPropertyValueNameToEnumMapper_load_indic_syllabic_category')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.LineBreak.html#method.name_to_enum_mapper) for more information.
  factory ICU4XPropertyValueNameToEnumMapper.loadLineBreak(
      ICU4XDataProvider provider) {
    final result = _loadLineBreakFfi(provider._underlying);
    return result.isOk
        ? ICU4XPropertyValueNameToEnumMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadLineBreakFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPropertyValueNameToEnumMapper_load_line_break')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `get_name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.GraphemeClusterBreak.html#method.get_name_to_enum_mapper) for more information.
  factory ICU4XPropertyValueNameToEnumMapper.loadGraphemeClusterBreak(
      ICU4XDataProvider provider) {
    final result = _loadGraphemeClusterBreakFfi(provider._underlying);
    return result.isOk
        ? ICU4XPropertyValueNameToEnumMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadGraphemeClusterBreakFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPropertyValueNameToEnumMapper_load_grapheme_cluster_break')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.WordBreak.html#method.name_to_enum_mapper) for more information.
  factory ICU4XPropertyValueNameToEnumMapper.loadWordBreak(
      ICU4XDataProvider provider) {
    final result = _loadWordBreakFfi(provider._underlying);
    return result.isOk
        ? ICU4XPropertyValueNameToEnumMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadWordBreakFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPropertyValueNameToEnumMapper_load_word_break')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.SentenceBreak.html#method.name_to_enum_mapper) for more information.
  factory ICU4XPropertyValueNameToEnumMapper.loadSentenceBreak(
      ICU4XDataProvider provider) {
    final result = _loadSentenceBreakFfi(provider._underlying);
    return result.isOk
        ? ICU4XPropertyValueNameToEnumMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadSentenceBreakFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPropertyValueNameToEnumMapper_load_sentence_break')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.Script.html#method.name_to_enum_mapper) for more information.
  factory ICU4XPropertyValueNameToEnumMapper.loadScript(
      ICU4XDataProvider provider) {
    final result = _loadScriptFfi(provider._underlying);
    return result.isOk
        ? ICU4XPropertyValueNameToEnumMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadScriptFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XPropertyValueNameToEnumMapper_load_script')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
}

/// See the [Rust documentation for `RegionDisplayNames`](https://docs.rs/icu/latest/icu/displaynames/struct.RegionDisplayNames.html) for more information.
class ICU4XRegionDisplayNames implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XRegionDisplayNames._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XRegionDisplayNames_destroy'));

  /// Creates a new `RegionDisplayNames` from locale data and an options bag.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/displaynames/struct.RegionDisplayNames.html#method.try_new) for more information.
  factory ICU4XRegionDisplayNames.tryNew(
      ICU4XDataProvider provider, ICU4XLocale locale) {
    final result = _tryNewFfi(provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XRegionDisplayNames._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _tryNewFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XRegionDisplayNames_try_new')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the locale specific display name of a region.
  /// Note that the funtion returns an empty string in case the display name for a given
  /// region code is not found.
  ///
  /// See the [Rust documentation for `of`](https://docs.rs/icu/latest/icu/displaynames/struct.RegionDisplayNames.html#method.of) for more information.
  String of(String region) {
    final alloc = ffi2.Arena();
    final regionSlice = _SliceFfi2Utf8._fromDart(region, alloc);

    final writeable = _Writeable();
    final result = _ofFfi(_underlying, regionSlice._bytes, regionSlice._length,
        writeable._underlying);
    alloc.releaseAll();
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _ofFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XRegionDisplayNames_of')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// Thin wrapper around a vector that maps visual indices to source indices
///
/// `map[visualIndex] = sourceIndex`
///
/// Produced by `reorder_visual()` on [`ICU4XBidi`].
class ICU4XReorderedIndexMap implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XReorderedIndexMap._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XReorderedIndexMap_destroy'));

  /// Get this as a slice/array of indices
  SizeList get asSlice {
    final result = _asSliceFfi(_underlying);
    return result._asDart;
  }

  static final _asSliceFfi =
      _capi<ffi.NativeFunction<SizeList Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XReorderedIndexMap_as_slice')
          .asFunction<SizeList Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// The length of this map
  int get len {
    final result = _lenFfi(_underlying);
    return result;
  }

  static final _lenFfi =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XReorderedIndexMap_len')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Get element at `index`. Returns 0 when out of bounds
  /// (note that 0 is also a valid in-bounds value, please use `len()`
  /// to avoid out-of-bounds)
  int get(int index) {
    final result = _getFfi(_underlying, index);
    return result;
  }

  static final _getFfi = _capi<
          ffi.NativeFunction<
              ffi.Size Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Size)>>('ICU4XReorderedIndexMap_get')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);
}

/// Increment used in a rounding operation.
///
/// See the [Rust documentation for `RoundingIncrement`](https://docs.rs/fixed_decimal/latest/fixed_decimal/enum.RoundingIncrement.html) for more information.
enum ICU4XRoundingIncrement {
  multiplesOf1.__(0),
  multiplesOf2.__(1),
  multiplesOf5.__(2),
  multiplesOf25.__(3);

  const ICU4XRoundingIncrement.__(this._id);

  // ignore: unused_element
  factory ICU4XRoundingIncrement._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// An object that represents the Script_Extensions property for a single character
///
/// See the [Rust documentation for `ScriptExtensionsSet`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptExtensionsSet.html) for more information.
class ICU4XScriptExtensionsSet implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XScriptExtensionsSet._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XScriptExtensionsSet_destroy'));

  /// Check if the Script_Extensions property of the given code point covers the given script
  ///
  /// See the [Rust documentation for `contains`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptExtensionsSet.html#method.contains) for more information.
  bool contains(int script) {
    final result = _containsFfi(_underlying, script);
    return result;
  }

  static final _containsFfi = _capi<
          ffi.NativeFunction<
              ffi.Bool Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint16)>>('ICU4XScriptExtensionsSet_contains')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Get the number of scripts contained in here
  ///
  /// See the [Rust documentation for `iter`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptExtensionsSet.html#method.iter) for more information.
  int get count {
    final result = _countFfi(_underlying);
    return result;
  }

  static final _countFfi =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XScriptExtensionsSet_count')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Get script at index, returning an error if out of bounds
  ///
  /// See the [Rust documentation for `iter`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptExtensionsSet.html#method.iter) for more information.
  int scriptAt(int index) {
    final result = _scriptAtFfi(_underlying, index);
    return result.isOk ? result.union.ok : throw VoidError();
  }

  static final _scriptAtFfi = _capi<
          ffi.NativeFunction<
              _ResultUint16Void Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Size)>>('ICU4XScriptExtensionsSet_script_at')
      .asFunction<_ResultUint16Void Function(ffi.Pointer<ffi.Opaque>, int)>(
          isLeaf: true);
}

/// An ICU4X ScriptWithExtensions map object, capable of holding a map of codepoints to scriptextensions values
///
/// See the [Rust documentation for `ScriptWithExtensions`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptWithExtensions.html) for more information.
class ICU4XScriptWithExtensions implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XScriptWithExtensions._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XScriptWithExtensions_destroy'));

  /// See the [Rust documentation for `script_with_extensions`](https://docs.rs/icu/latest/icu/properties/script/fn.script_with_extensions.html) for more information.
  factory ICU4XScriptWithExtensions(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XScriptWithExtensions._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XScriptWithExtensions_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Get the Script property value for a code point
  ///
  /// See the [Rust documentation for `get_script_val`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_val) for more information.
  int getScriptVal(int codePoint) {
    final result = _getScriptValFfi(_underlying, codePoint);
    return result;
  }

  static final _getScriptValFfi = _capi<
          ffi.NativeFunction<
              ffi.Uint16 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XScriptWithExtensions_get_script_val')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Check if the Script_Extensions property of the given code point covers the given script
  ///
  /// See the [Rust documentation for `has_script`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.has_script) for more information.
  bool hasScript(int codePoint, int script) {
    final result = _hasScriptFfi(_underlying, codePoint, script);
    return result;
  }

  static final _hasScriptFfi = _capi<
          ffi.NativeFunction<
              ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32,
                  ffi.Uint16)>>('ICU4XScriptWithExtensions_has_script')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, int, int)>(
          isLeaf: true);

  /// Borrow this object for a slightly faster variant with more operations
  ///
  /// See the [Rust documentation for `as_borrowed`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptWithExtensions.html#method.as_borrowed) for more information.
  ICU4XScriptWithExtensionsBorrowed get asBorrowed {
    final result = _asBorrowedFfi(_underlying);
    return ICU4XScriptWithExtensionsBorrowed._(result);
  }

  static final _asBorrowedFfi = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XScriptWithExtensions_as_borrowed')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Get a list of ranges of code points that contain this script in their Script_Extensions values
  ///
  /// See the [Rust documentation for `get_script_extensions_ranges`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_extensions_ranges) for more information.
  CodePointRangeIterator iterRangesForScript(int script) {
    final result = _iterRangesForScriptFfi(_underlying, script);
    return CodePointRangeIterator._(result);
  }

  static final _iterRangesForScriptFfi = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Opaque> Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Uint16)>>(
          'ICU4XScriptWithExtensions_iter_ranges_for_script')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);
}

/// A slightly faster ICU4XScriptWithExtensions object
///
/// See the [Rust documentation for `ScriptWithExtensionsBorrowed`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html) for more information.
class ICU4XScriptWithExtensionsBorrowed implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XScriptWithExtensionsBorrowed._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XScriptWithExtensionsBorrowed_destroy'));

  /// Get the Script property value for a code point
  ///
  /// See the [Rust documentation for `get_script_val`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_val) for more information.
  int getScriptVal(int codePoint) {
    final result = _getScriptValFfi(_underlying, codePoint);
    return result;
  }

  static final _getScriptValFfi = _capi<
              ffi.NativeFunction<
                  ffi.Uint16 Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32)>>(
          'ICU4XScriptWithExtensionsBorrowed_get_script_val')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Get the Script property value for a code point
  ///
  /// See the [Rust documentation for `get_script_extensions_val`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_extensions_val) for more information.
  ICU4XScriptExtensionsSet getScriptExtensionsVal(int codePoint) {
    final result = _getScriptExtensionsValFfi(_underlying, codePoint);
    return ICU4XScriptExtensionsSet._(result);
  }

  static final _getScriptExtensionsValFfi = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Opaque> Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Uint32)>>(
          'ICU4XScriptWithExtensionsBorrowed_get_script_extensions_val')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Check if the Script_Extensions property of the given code point covers the given script
  ///
  /// See the [Rust documentation for `has_script`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.has_script) for more information.
  bool hasScript(int codePoint, int script) {
    final result = _hasScriptFfi(_underlying, codePoint, script);
    return result;
  }

  static final _hasScriptFfi = _capi<
          ffi.NativeFunction<
              ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Uint32,
                  ffi.Uint16)>>('ICU4XScriptWithExtensionsBorrowed_has_script')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, int, int)>(
          isLeaf: true);

  /// Build the CodePointSetData corresponding to a codepoints matching a particular script
  /// in their Script_Extensions
  ///
  /// See the [Rust documentation for `get_script_extensions_set`](https://docs.rs/icu/latest/icu/properties/script/struct.ScriptWithExtensionsBorrowed.html#method.get_script_extensions_set) for more information.
  ICU4XCodePointSetData getScriptExtensionsSet(int script) {
    final result = _getScriptExtensionsSetFfi(_underlying, script);
    return ICU4XCodePointSetData._(result);
  }

  static final _getScriptExtensionsSetFfi = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Opaque> Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Uint16)>>(
          'ICU4XScriptWithExtensionsBorrowed_get_script_extensions_set')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);
}

/// See the [Rust documentation for `WordType`](https://docs.rs/icu/latest/icu/segmenter/enum.WordType.html) for more information.
enum ICU4XSegmenterWordType {
  none.__(0),
  number.__(1),
  letter.__(2);

  const ICU4XSegmenterWordType.__(this._id);

  // ignore: unused_element
  factory ICU4XSegmenterWordType._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `SentenceBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.SentenceBreakIterator.html) for more information.
class ICU4XSentenceBreakIteratorLatin1 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XSentenceBreakIteratorLatin1._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XSentenceBreakIteratorLatin1_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.SentenceBreakIterator.html#method.next) for more information.
  int next() {
    final result = _nextFfi(_underlying);
    return result;
  }

  static final _nextFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XSentenceBreakIteratorLatin1_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `SentenceBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.SentenceBreakIterator.html) for more information.
class ICU4XSentenceBreakIteratorUtf16 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XSentenceBreakIteratorUtf16._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XSentenceBreakIteratorUtf16_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.SentenceBreakIterator.html#method.next) for more information.
  int next() {
    final result = _nextFfi(_underlying);
    return result;
  }

  static final _nextFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XSentenceBreakIteratorUtf16_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `SentenceBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.SentenceBreakIterator.html) for more information.
class ICU4XSentenceBreakIteratorUtf8 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XSentenceBreakIteratorUtf8._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XSentenceBreakIteratorUtf8_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.SentenceBreakIterator.html#method.next) for more information.
  int next() {
    final result = _nextFfi(_underlying);
    return result;
  }

  static final _nextFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XSentenceBreakIteratorUtf8_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An ICU4X sentence-break segmenter, capable of finding sentence breakpoints in strings.
///
/// See the [Rust documentation for `SentenceSegmenter`](https://docs.rs/icu/latest/icu/segmenter/struct.SentenceSegmenter.html) for more information.
class ICU4XSentenceSegmenter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XSentenceSegmenter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XSentenceSegmenter_destroy'));

  /// Construct an [`ICU4XSentenceSegmenter`].
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/segmenter/struct.SentenceSegmenter.html#method.new) for more information.
  factory ICU4XSentenceSegmenter(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XSentenceSegmenter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XSentenceSegmenter_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Segments a (potentially ill-formed) UTF-8 string.
  ///
  /// See the [Rust documentation for `segment_utf8`](https://docs.rs/icu/latest/icu/segmenter/struct.SentenceSegmenter.html#method.segment_utf8) for more information.
  ICU4XSentenceBreakIteratorUtf8 segmentUtf8(String input) {
    final alloc = ffi2.Arena();
    final inputSlice = _SliceFfi2Utf8._fromDart(input, alloc);

    final result =
        _segmentUtf8Ffi(_underlying, inputSlice._bytes, inputSlice._length);
    alloc.releaseAll();
    return ICU4XSentenceBreakIteratorUtf8._(result);
  }

  static final _segmentUtf8Ffi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XSentenceSegmenter_segment_utf8')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Segments a UTF-16 string.
  ///
  /// See the [Rust documentation for `segment_utf16`](https://docs.rs/icu/latest/icu/segmenter/struct.SentenceSegmenter.html#method.segment_utf16) for more information.
  ICU4XSentenceBreakIteratorUtf16 segmentUtf16(Uint16List input) {
    final alloc = ffi2.Arena();
    final inputSlice = _SliceFfiUint16._fromDart(input, alloc);

    final result =
        _segmentUtf16Ffi(_underlying, inputSlice._bytes, inputSlice._length);
    alloc.releaseAll();
    return ICU4XSentenceBreakIteratorUtf16._(result);
  }

  static final _segmentUtf16Ffi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Uint16>,
                  ffi.Size)>>('ICU4XSentenceSegmenter_segment_utf16')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Uint16>, int)>(isLeaf: true);

  /// Segments a Latin-1 string.
  ///
  /// See the [Rust documentation for `segment_latin1`](https://docs.rs/icu/latest/icu/segmenter/struct.SentenceSegmenter.html#method.segment_latin1) for more information.
  ICU4XSentenceBreakIteratorLatin1 segmentLatin1(Uint8List input) {
    final alloc = ffi2.Arena();
    final inputSlice = _SliceFfiUint8._fromDart(input, alloc);

    final result =
        _segmentLatin1Ffi(_underlying, inputSlice._bytes, inputSlice._length);
    alloc.releaseAll();
    return ICU4XSentenceBreakIteratorLatin1._(result);
  }

  static final _segmentLatin1Ffi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Uint8>,
                  ffi.Size)>>('ICU4XSentenceSegmenter_segment_latin1')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);
}

/// An ICU4X Time object representing a time in terms of hour, minute, second, nanosecond
///
/// See the [Rust documentation for `Time`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html) for more information.
class ICU4XTime implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XTime._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XTime_destroy'));

  /// Creates a new [`ICU4XTime`] given field values
  ///
  /// See the [Rust documentation for `Time`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html) for more information.
  factory ICU4XTime(int hour, int minute, int second, int nanosecond) {
    final result = _createFfi(hour, minute, second, nanosecond);
    return result.isOk
        ? ICU4XTime._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Uint8, ffi.Uint8, ffi.Uint8,
                  ffi.Uint32)>>('ICU4XTime_create')
      .asFunction<_ResultOpaqueUint32 Function(int, int, int, int)>(
          isLeaf: true);

  /// Creates a new [`ICU4XTime`] representing midnight (00:00.000).
  ///
  /// See the [Rust documentation for `Time`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html) for more information.
  factory ICU4XTime.midnight() {
    final result = _createMidnightFfi();
    return result.isOk
        ? ICU4XTime._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createMidnightFfi =
      _capi<ffi.NativeFunction<_ResultOpaqueUint32 Function()>>(
              'ICU4XTime_create_midnight')
          .asFunction<_ResultOpaqueUint32 Function()>(isLeaf: true);

  /// Returns the hour in this time
  ///
  /// See the [Rust documentation for `hour`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.hour) for more information.
  int get hour {
    final result = _hourFfi(_underlying);
    return result;
  }

  static final _hourFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XTime_hour')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the minute in this time
  ///
  /// See the [Rust documentation for `minute`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.minute) for more information.
  int get minute {
    final result = _minuteFfi(_underlying);
    return result;
  }

  static final _minuteFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XTime_minute')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the second in this time
  ///
  /// See the [Rust documentation for `second`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.second) for more information.
  int get second {
    final result = _secondFfi(_underlying);
    return result;
  }

  static final _secondFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XTime_second')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Returns the nanosecond in this time
  ///
  /// See the [Rust documentation for `nanosecond`](https://docs.rs/icu/latest/icu/calendar/types/struct.Time.html#structfield.nanosecond) for more information.
  int get nanosecond {
    final result = _nanosecondFfi(_underlying);
    return result;
  }

  static final _nanosecondFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XTime_nanosecond')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An ICU4X TimeFormatter object capable of formatting an [`ICU4XTime`] type (and others) as a string
///
/// See the [Rust documentation for `TimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.TimeFormatter.html) for more information.
class ICU4XTimeFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XTimeFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XTimeFormatter_destroy'));

  /// Creates a new [`ICU4XTimeFormatter`] from locale data.
  ///
  /// See the [Rust documentation for `try_new_with_length`](https://docs.rs/icu/latest/icu/datetime/struct.TimeFormatter.html#method.try_new_with_length) for more information.
  factory ICU4XTimeFormatter.withLength(
      ICU4XDataProvider provider, ICU4XLocale locale, ICU4XTimeLength length) {
    final result = _createWithLengthFfi(
        provider._underlying, locale._underlying, length._id);
    return result.isOk
        ? ICU4XTimeFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithLengthFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XTimeFormatter_create_with_length')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Formats a [`ICU4XTime`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.TimeFormatter.html#method.format) for more information.
  String formatTime(ICU4XTime value) {
    final writeable = _Writeable();
    final result =
        _formatTimeFfi(_underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatTimeFfi = _capi<
          ffi.NativeFunction<
              _ResultVoidUint32 Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XTimeFormatter_format_time')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Formats a [`ICU4XDateTime`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.TimeFormatter.html#method.format) for more information.
  String formatDatetime(ICU4XDateTime value) {
    final writeable = _Writeable();
    final result = _formatDatetimeFfi(
        _underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatDatetimeFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XTimeFormatter_format_datetime')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Formats a [`ICU4XIsoDateTime`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.TimeFormatter.html#method.format) for more information.
  String formatIsoDatetime(ICU4XIsoDateTime value) {
    final writeable = _Writeable();
    final result = _formatIsoDatetimeFfi(
        _underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatIsoDatetimeFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XTimeFormatter_format_iso_datetime')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `Time`](https://docs.rs/icu/latest/icu/datetime/options/length/enum.Time.html) for more information.
enum ICU4XTimeLength {
  full.__(0),
  long.__(1),
  medium.__(2),
  short.__(3);

  const ICU4XTimeLength.__(this._id);

  // ignore: unused_element
  factory ICU4XTimeLength._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// An ICU4X TimeZoneFormatter object capable of formatting an [`ICU4XCustomTimeZone`] type (and others) as a string
///
/// See the [Rust documentation for `TimeZoneFormatter`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html) for more information.
class ICU4XTimeZoneFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XTimeZoneFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XTimeZoneFormatter_destroy'));

  /// Creates a new [`ICU4XTimeZoneFormatter`] from locale data.
  ///
  /// Uses localized GMT as the fallback format.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html#method.try_new) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/datetime/time_zone/enum.FallbackFormat.html)
  factory ICU4XTimeZoneFormatter.withLocalizedGmtFallback(
      ICU4XDataProvider provider, ICU4XLocale locale) {
    final result = _createWithLocalizedGmtFallbackFfi(
        provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XTimeZoneFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithLocalizedGmtFallbackFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XTimeZoneFormatter_create_with_localized_gmt_fallback')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Creates a new [`ICU4XTimeZoneFormatter`] from locale data.
  ///
  /// Uses ISO-8601 as the fallback format.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html#method.try_new) for more information.
  ///
  /// Additional information: [1](https://docs.rs/icu/latest/icu/datetime/time_zone/enum.FallbackFormat.html)
  factory ICU4XTimeZoneFormatter.withIso8601Fallback(ICU4XDataProvider provider,
      ICU4XLocale locale, ICU4XIsoTimeZoneOptions options) {
    final result = _createWithIso8601FallbackFfi(
        provider._underlying, locale._underlying, options._underlying);
    return result.isOk
        ? ICU4XTimeZoneFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithIso8601FallbackFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, _ICU4XIsoTimeZoneOptionsFfi)>>(
          'ICU4XTimeZoneFormatter_create_with_iso_8601_fallback')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              _ICU4XIsoTimeZoneOptionsFfi)>(isLeaf: true);

  /// Loads generic non-location long format. Example: "Pacific Time"
  ///
  /// See the [Rust documentation for `include_generic_non_location_long`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html#method.include_generic_non_location_long) for more information.
  void loadGenericNonLocationLong(ICU4XDataProvider provider) {
    final result =
        _loadGenericNonLocationLongFfi(_underlying, provider._underlying);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _loadGenericNonLocationLongFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XTimeZoneFormatter_load_generic_non_location_long')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Loads generic non-location short format. Example: "PT"
  ///
  /// See the [Rust documentation for `include_generic_non_location_short`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html#method.include_generic_non_location_short) for more information.
  void loadGenericNonLocationShort(ICU4XDataProvider provider) {
    final result =
        _loadGenericNonLocationShortFfi(_underlying, provider._underlying);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _loadGenericNonLocationShortFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XTimeZoneFormatter_load_generic_non_location_short')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Loads specific non-location long format. Example: "Pacific Standard Time"
  ///
  /// See the [Rust documentation for `include_specific_non_location_long`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html#method.include_specific_non_location_long) for more information.
  void loadSpecificNonLocationLong(ICU4XDataProvider provider) {
    final result =
        _loadSpecificNonLocationLongFfi(_underlying, provider._underlying);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _loadSpecificNonLocationLongFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XTimeZoneFormatter_load_specific_non_location_long')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Loads specific non-location short format. Example: "PST"
  ///
  /// See the [Rust documentation for `include_specific_non_location_short`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html#method.include_specific_non_location_short) for more information.
  void loadSpecificNonLocationShort(ICU4XDataProvider provider) {
    final result =
        _loadSpecificNonLocationShortFfi(_underlying, provider._underlying);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _loadSpecificNonLocationShortFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XTimeZoneFormatter_load_specific_non_location_short')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Loads generic location format. Example: "Los Angeles Time"
  ///
  /// See the [Rust documentation for `include_generic_location_format`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html#method.include_generic_location_format) for more information.
  void loadGenericLocationFormat(ICU4XDataProvider provider) {
    final result =
        _loadGenericLocationFormatFfi(_underlying, provider._underlying);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _loadGenericLocationFormatFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XTimeZoneFormatter_load_generic_location_format')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Loads localized GMT format. Example: "GMT-07:00"
  ///
  /// See the [Rust documentation for `include_localized_gmt_format`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html#method.include_localized_gmt_format) for more information.
  void includeLocalizedGmtFormat() {
    final result = _includeLocalizedGmtFormatFfi(_underlying);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _includeLocalizedGmtFormatFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XTimeZoneFormatter_include_localized_gmt_format')
      .asFunction<_ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Loads ISO-8601 format. Example: "-07:00"
  ///
  /// See the [Rust documentation for `include_iso_8601_format`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html#method.include_iso_8601_format) for more information.
  void loadIso8601Format(ICU4XIsoTimeZoneOptions options) {
    final result = _loadIso8601FormatFfi(_underlying, options._underlying);
    if (!result.isOk) {
      throw ICU4XError._(result.union.err);
    }
  }

  static final _loadIso8601FormatFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>, _ICU4XIsoTimeZoneOptionsFfi)>>(
          'ICU4XTimeZoneFormatter_load_iso_8601_format')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              _ICU4XIsoTimeZoneOptionsFfi)>(isLeaf: true);

  /// Formats a [`ICU4XCustomTimeZone`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html#method.format) for more information.
  ///
  /// See the [Rust documentation for `format_to_string`](https://docs.rs/icu/latest/icu/datetime/time_zone/struct.TimeZoneFormatter.html#method.format_to_string) for more information.
  String formatCustomTimeZone(ICU4XCustomTimeZone value) {
    final writeable = _Writeable();
    final result = _formatCustomTimeZoneFfi(
        _underlying, value._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatCustomTimeZoneFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XTimeZoneFormatter_format_custom_time_zone')
      .asFunction<
          _ResultVoidUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `TitlecaseMapper`](https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html) for more information.
class ICU4XTitlecaseMapper implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XTitlecaseMapper._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XTitlecaseMapper_destroy'));

  /// Construct a new `ICU4XTitlecaseMapper` instance
  ///
  /// See the [Rust documentation for `new`](https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html#method.new) for more information.
  factory ICU4XTitlecaseMapper(ICU4XDataProvider provider) {
    final result = _createFfi(provider._underlying);
    return result.isOk
        ? ICU4XTitlecaseMapper._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XTitlecaseMapper_create')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Returns the full titlecase mapping of the given string
  ///
  /// The `v1` refers to the version of the options struct, which may change as we add more options
  ///
  /// See the [Rust documentation for `titlecase_segment`](https://docs.rs/icu/latest/icu/casemap/struct.TitlecaseMapper.html#method.titlecase_segment) for more information.
  String titlecaseSegmentV1(
      String s, ICU4XLocale locale, ICU4XTitlecaseOptionsV1 options) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final writeable = _Writeable();
    final result = _titlecaseSegmentV1Ffi(
        _underlying,
        sSlice._bytes,
        sSlice._length,
        locale._underlying,
        options._underlying,
        writeable._underlying);
    alloc.releaseAll();
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _titlecaseSegmentV1Ffi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi2.Utf8>,
                      ffi.Size,
                      ffi.Pointer<ffi.Opaque>,
                      _ICU4XTitlecaseOptionsV1Ffi,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XTitlecaseMapper_titlecase_segment_v1')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>,
              int,
              ffi.Pointer<ffi.Opaque>,
              _ICU4XTitlecaseOptionsV1Ffi,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `TitlecaseOptions`](https://docs.rs/icu/latest/icu/casemap/titlecase/struct.TitlecaseOptions.html) for more information.
class _ICU4XTitlecaseOptionsV1Ffi extends ffi.Struct {
  @ffi.Uint32()
  external int leadingAdjustment;
  @ffi.Uint32()
  external int trailingCase;
}

class ICU4XTitlecaseOptionsV1 {
  final _ICU4XTitlecaseOptionsV1Ffi _underlying;

  // ignore: unused_element
  ICU4XTitlecaseOptionsV1._(this._underlying);

  factory ICU4XTitlecaseOptionsV1() {
    final pointer = ffi2.calloc<_ICU4XTitlecaseOptionsV1Ffi>();
    final result = ICU4XTitlecaseOptionsV1._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  ICU4XLeadingAdjustment get leadingAdjustment =>
      ICU4XLeadingAdjustment._(_underlying.leadingAdjustment);
  set leadingAdjustment(ICU4XLeadingAdjustment leadingAdjustment) {
    _underlying.leadingAdjustment = leadingAdjustment._id;
  }

  ICU4XTrailingCase get trailingCase =>
      ICU4XTrailingCase._(_underlying.trailingCase);
  set trailingCase(ICU4XTrailingCase trailingCase) {
    _underlying.trailingCase = trailingCase._id;
  }

  /// See the [Rust documentation for `default`](https://docs.rs/icu/latest/icu/casemap/titlecase/struct.TitlecaseOptions.html#method.default) for more information.
  factory ICU4XTitlecaseOptionsV1.options() {
    final result = _defaultOptionsFfi();
    return ICU4XTitlecaseOptionsV1._(result);
  }
  static final _defaultOptionsFfi =
      _capi<ffi.NativeFunction<_ICU4XTitlecaseOptionsV1Ffi Function()>>(
              'ICU4XTitlecaseOptionsV1_default_options')
          .asFunction<_ICU4XTitlecaseOptionsV1Ffi Function()>(isLeaf: true);

  @override
  bool operator ==(Object other) =>
      other is ICU4XTitlecaseOptionsV1 &&
      other._underlying.leadingAdjustment == _underlying.leadingAdjustment &&
      other._underlying.trailingCase == _underlying.trailingCase;

  @override
  int get hashCode => Object.hashAll([
        _underlying.leadingAdjustment,
        _underlying.trailingCase,
      ]);
}

/// See the [Rust documentation for `TrailingCase`](https://docs.rs/icu/latest/icu/casemap/titlecase/enum.TrailingCase.html) for more information.
enum ICU4XTrailingCase {
  lower.__(0),
  unchanged.__(1);

  const ICU4XTrailingCase.__(this._id);

  // ignore: unused_element
  factory ICU4XTrailingCase._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// FFI version of `TransformResult`.
///
/// See the [Rust documentation for `TransformResult`](https://docs.rs/icu/latest/icu/locid_transform/enum.TransformResult.html) for more information.
enum ICU4XTransformResult {
  modified.__(0),
  unmodified.__(1);

  const ICU4XTransformResult.__(this._id);

  // ignore: unused_element
  factory ICU4XTransformResult._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// An ICU4X Unicode Set Property object, capable of querying whether a code point is contained in a set based on a Unicode property.
///
/// See the [Rust documentation for `properties`](https://docs.rs/icu/latest/icu/properties/index.html) for more information.
///
/// See the [Rust documentation for `UnicodeSetData`](https://docs.rs/icu/latest/icu/properties/sets/struct.UnicodeSetData.html) for more information.
///
/// See the [Rust documentation for `UnicodeSetDataBorrowed`](https://docs.rs/icu/latest/icu/properties/sets/struct.UnicodeSetDataBorrowed.html) for more information.
class ICU4XUnicodeSetData implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XUnicodeSetData._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XUnicodeSetData_destroy'));

  /// Checks whether the string is in the set.
  ///
  /// See the [Rust documentation for `contains`](https://docs.rs/icu/latest/icu/properties/sets/struct.UnicodeSetDataBorrowed.html#method.contains) for more information.
  bool contains(String s) {
    final alloc = ffi2.Arena();
    final sSlice = _SliceFfi2Utf8._fromDart(s, alloc);

    final result = _containsFfi(_underlying, sSlice._bytes, sSlice._length);
    alloc.releaseAll();
    return result;
  }

  static final _containsFfi = _capi<
          ffi.NativeFunction<
              ffi.Bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XUnicodeSetData_contains')
      .asFunction<
          bool Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi2.Utf8>,
              int)>(isLeaf: true);

  /// Checks whether the code point is in the set.
  ///
  /// See the [Rust documentation for `contains_char`](https://docs.rs/icu/latest/icu/properties/sets/struct.UnicodeSetDataBorrowed.html#method.contains_char) for more information.
  bool containsChar(int cp) {
    final result = _containsCharFfi(_underlying, cp);
    return result;
  }

  static final _containsCharFfi = _capi<
          ffi.NativeFunction<
              ffi.Bool Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XUnicodeSetData_contains_char')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// Checks whether the code point (specified as a 32 bit integer, in UTF-32) is in the set.
  bool contains32(int cp) {
    final result = _contains32Ffi(_underlying, cp);
    return result;
  }

  static final _contains32Ffi = _capi<
          ffi.NativeFunction<
              ffi.Bool Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Uint32)>>('ICU4XUnicodeSetData_contains32')
      .asFunction<bool Function(ffi.Pointer<ffi.Opaque>, int)>(isLeaf: true);

  /// See the [Rust documentation for `basic_emoji`](https://docs.rs/icu/latest/icu/properties/sets/fn.basic_emoji.html) for more information.
  factory ICU4XUnicodeSetData.loadBasicEmoji(ICU4XDataProvider provider) {
    final result = _loadBasicEmojiFfi(provider._underlying);
    return result.isOk
        ? ICU4XUnicodeSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadBasicEmojiFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XUnicodeSetData_load_basic_emoji')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// See the [Rust documentation for `exemplars_main`](https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_main.html) for more information.
  factory ICU4XUnicodeSetData.loadExemplarsMain(
      ICU4XDataProvider provider, ICU4XLocale locale) {
    final result =
        _loadExemplarsMainFfi(provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XUnicodeSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadExemplarsMainFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XUnicodeSetData_load_exemplars_main')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `exemplars_auxiliary`](https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_auxiliary.html) for more information.
  factory ICU4XUnicodeSetData.loadExemplarsAuxiliary(
      ICU4XDataProvider provider, ICU4XLocale locale) {
    final result =
        _loadExemplarsAuxiliaryFfi(provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XUnicodeSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadExemplarsAuxiliaryFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XUnicodeSetData_load_exemplars_auxiliary')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `exemplars_punctuation`](https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_punctuation.html) for more information.
  factory ICU4XUnicodeSetData.loadExemplarsPunctuation(
      ICU4XDataProvider provider, ICU4XLocale locale) {
    final result =
        _loadExemplarsPunctuationFfi(provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XUnicodeSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadExemplarsPunctuationFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XUnicodeSetData_load_exemplars_punctuation')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `exemplars_numbers`](https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_numbers.html) for more information.
  factory ICU4XUnicodeSetData.loadExemplarsNumbers(
      ICU4XDataProvider provider, ICU4XLocale locale) {
    final result =
        _loadExemplarsNumbersFfi(provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XUnicodeSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadExemplarsNumbersFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XUnicodeSetData_load_exemplars_numbers')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// See the [Rust documentation for `exemplars_index`](https://docs.rs/icu/latest/icu/properties/exemplar_chars/fn.exemplars_index.html) for more information.
  factory ICU4XUnicodeSetData.loadExemplarsIndex(
      ICU4XDataProvider provider, ICU4XLocale locale) {
    final result =
        _loadExemplarsIndexFfi(provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XUnicodeSetData._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _loadExemplarsIndexFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XUnicodeSetData_load_exemplars_index')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// A Week calculator, useful to be passed in to `week_of_year()` on Date and DateTime types
///
/// See the [Rust documentation for `WeekCalculator`](https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html) for more information.
class ICU4XWeekCalculator implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XWeekCalculator._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XWeekCalculator_destroy'));

  /// Creates a new [`ICU4XWeekCalculator`] from locale data.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#method.try_new) for more information.
  factory ICU4XWeekCalculator(ICU4XDataProvider provider, ICU4XLocale locale) {
    final result = _createFfi(provider._underlying, locale._underlying);
    return result.isOk
        ? ICU4XWeekCalculator._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XWeekCalculator_create')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Additional information: [1](https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#structfield.first_weekday), [2](https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#structfield.min_week_days)
  factory ICU4XWeekCalculator.fromFirstDayOfWeekAndMinWeekDays(
      ICU4XIsoWeekday firstWeekday, int minWeekDays) {
    final result = _createFromFirstDayOfWeekAndMinWeekDaysFfi(
        firstWeekday._id, minWeekDays);
    return ICU4XWeekCalculator._(result);
  }
  static final _createFromFirstDayOfWeekAndMinWeekDaysFfi = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi.Opaque> Function(ffi.Uint32, ffi.Uint8)>>(
          'ICU4XWeekCalculator_create_from_first_day_of_week_and_min_week_days')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(int, int)>(isLeaf: true);

  /// Returns the weekday that starts the week for this object's locale
  ///
  /// See the [Rust documentation for `first_weekday`](https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#structfield.first_weekday) for more information.
  ICU4XIsoWeekday get firstWeekday {
    final result = _firstWeekdayFfi(_underlying);
    return ICU4XIsoWeekday._(result);
  }

  static final _firstWeekdayFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XWeekCalculator_first_weekday')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// The minimum number of days overlapping a year required for a week to be
  /// considered part of that year
  ///
  /// See the [Rust documentation for `min_week_days`](https://docs.rs/icu/latest/icu/calendar/week/struct.WeekCalculator.html#structfield.min_week_days) for more information.
  int get minWeekDays {
    final result = _minWeekDaysFfi(_underlying);
    return result;
  }

  static final _minWeekDaysFfi =
      _capi<ffi.NativeFunction<ffi.Uint8 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XWeekCalculator_min_week_days')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `WeekOf`](https://docs.rs/icu/latest/icu/calendar/week/struct.WeekOf.html) for more information.
class _ICU4XWeekOfFfi extends ffi.Struct {
  @ffi.Uint16()
  external int week;
  @ffi.Uint32()
  external int unit;
}

class ICU4XWeekOf {
  final _ICU4XWeekOfFfi _underlying;

  // ignore: unused_element
  ICU4XWeekOf._(this._underlying);

  factory ICU4XWeekOf() {
    final pointer = ffi2.calloc<_ICU4XWeekOfFfi>();
    final result = ICU4XWeekOf._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  int get week => _underlying.week;
  set week(int week) {
    _underlying.week = week;
  }

  ICU4XWeekRelativeUnit get unit => ICU4XWeekRelativeUnit._(_underlying.unit);
  set unit(ICU4XWeekRelativeUnit unit) {
    _underlying.unit = unit._id;
  }

  @override
  bool operator ==(Object other) =>
      other is ICU4XWeekOf &&
      other._underlying.week == _underlying.week &&
      other._underlying.unit == _underlying.unit;

  @override
  int get hashCode => Object.hashAll([
        _underlying.week,
        _underlying.unit,
      ]);
}

/// See the [Rust documentation for `RelativeUnit`](https://docs.rs/icu/latest/icu/calendar/week/enum.RelativeUnit.html) for more information.
enum ICU4XWeekRelativeUnit {
  previous.__(0),
  current.__(1),
  next.__(2);

  const ICU4XWeekRelativeUnit.__(this._id);

  // ignore: unused_element
  factory ICU4XWeekRelativeUnit._(int id) =>
      values.firstWhere((value) => value._id == id);

  final int _id;
}

/// See the [Rust documentation for `WordBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html) for more information.
class ICU4XWordBreakIteratorLatin1 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XWordBreakIteratorLatin1._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XWordBreakIteratorLatin1_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.next) for more information.
  int next() {
    final result = _nextFfi(_underlying);
    return result;
  }

  static final _nextFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XWordBreakIteratorLatin1_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Return the status value of break boundary.
  ///
  /// See the [Rust documentation for `word_type`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.word_type) for more information.
  ICU4XSegmenterWordType get wordType {
    final result = _wordTypeFfi(_underlying);
    return ICU4XSegmenterWordType._(result);
  }

  static final _wordTypeFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XWordBreakIteratorLatin1_word_type')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Return true when break boundary is word-like such as letter/number/CJK
  ///
  /// See the [Rust documentation for `is_word_like`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.is_word_like) for more information.
  bool get isWordLike {
    final result = _isWordLikeFfi(_underlying);
    return result;
  }

  static final _isWordLikeFfi =
      _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XWordBreakIteratorLatin1_is_word_like')
          .asFunction<bool Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `WordBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html) for more information.
class ICU4XWordBreakIteratorUtf16 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XWordBreakIteratorUtf16._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XWordBreakIteratorUtf16_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.next) for more information.
  int next() {
    final result = _nextFfi(_underlying);
    return result;
  }

  static final _nextFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XWordBreakIteratorUtf16_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Return the status value of break boundary.
  ///
  /// See the [Rust documentation for `word_type`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.word_type) for more information.
  ICU4XSegmenterWordType get wordType {
    final result = _wordTypeFfi(_underlying);
    return ICU4XSegmenterWordType._(result);
  }

  static final _wordTypeFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XWordBreakIteratorUtf16_word_type')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Return true when break boundary is word-like such as letter/number/CJK
  ///
  /// See the [Rust documentation for `is_word_like`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.is_word_like) for more information.
  bool get isWordLike {
    final result = _isWordLikeFfi(_underlying);
    return result;
  }

  static final _isWordLikeFfi =
      _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XWordBreakIteratorUtf16_is_word_like')
          .asFunction<bool Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// See the [Rust documentation for `WordBreakIterator`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html) for more information.
class ICU4XWordBreakIteratorUtf8 implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XWordBreakIteratorUtf8._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XWordBreakIteratorUtf8_destroy'));

  /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
  /// out of range of a 32-bit signed integer.
  ///
  /// See the [Rust documentation for `next`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.next) for more information.
  int next() {
    final result = _nextFfi(_underlying);
    return result;
  }

  static final _nextFfi =
      _capi<ffi.NativeFunction<ffi.Int32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XWordBreakIteratorUtf8_next')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Return the status value of break boundary.
  ///
  /// See the [Rust documentation for `word_type`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.word_type) for more information.
  ICU4XSegmenterWordType get wordType {
    final result = _wordTypeFfi(_underlying);
    return ICU4XSegmenterWordType._(result);
  }

  static final _wordTypeFfi =
      _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XWordBreakIteratorUtf8_word_type')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Return true when break boundary is word-like such as letter/number/CJK
  ///
  /// See the [Rust documentation for `is_word_like`](https://docs.rs/icu/latest/icu/segmenter/struct.WordBreakIterator.html#method.is_word_like) for more information.
  bool get isWordLike {
    final result = _isWordLikeFfi(_underlying);
    return result;
  }

  static final _isWordLikeFfi =
      _capi<ffi.NativeFunction<ffi.Bool Function(ffi.Pointer<ffi.Opaque>)>>(
              'ICU4XWordBreakIteratorUtf8_is_word_like')
          .asFunction<bool Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

/// An ICU4X word-break segmenter, capable of finding word breakpoints in strings.
///
/// See the [Rust documentation for `WordSegmenter`](https://docs.rs/icu/latest/icu/segmenter/struct.WordSegmenter.html) for more information.
class ICU4XWordSegmenter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XWordSegmenter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XWordSegmenter_destroy'));

  /// Construct an [`ICU4XWordSegmenter`] with automatically selecting the best available LSTM
  /// or dictionary payload data.
  ///
  /// Note: currently, it uses dictionary for Chinese and Japanese, and LSTM for Burmese,
  /// Khmer, Lao, and Thai.
  ///
  /// See the [Rust documentation for `new_auto`](https://docs.rs/icu/latest/icu/segmenter/struct.WordSegmenter.html#method.new_auto) for more information.
  factory ICU4XWordSegmenter.auto(ICU4XDataProvider provider) {
    final result = _createAutoFfi(provider._underlying);
    return result.isOk
        ? ICU4XWordSegmenter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createAutoFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XWordSegmenter_create_auto')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Construct an [`ICU4XWordSegmenter`] with LSTM payload data for Burmese, Khmer, Lao, and
  /// Thai.
  ///
  /// Warning: [`ICU4XWordSegmenter`] created by this function doesn't handle Chinese or
  /// Japanese.
  ///
  /// See the [Rust documentation for `new_lstm`](https://docs.rs/icu/latest/icu/segmenter/struct.WordSegmenter.html#method.new_lstm) for more information.
  factory ICU4XWordSegmenter.lstm(ICU4XDataProvider provider) {
    final result = _createLstmFfi(provider._underlying);
    return result.isOk
        ? ICU4XWordSegmenter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createLstmFfi = _capi<
          ffi.NativeFunction<
              _ResultOpaqueUint32 Function(
                  ffi.Pointer<ffi.Opaque>)>>('ICU4XWordSegmenter_create_lstm')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Construct an [`ICU4XWordSegmenter`] with dictionary payload data for Chinese, Japanese,
  /// Burmese, Khmer, Lao, and Thai.
  ///
  /// See the [Rust documentation for `new_dictionary`](https://docs.rs/icu/latest/icu/segmenter/struct.WordSegmenter.html#method.new_dictionary) for more information.
  factory ICU4XWordSegmenter.dictionary(ICU4XDataProvider provider) {
    final result = _createDictionaryFfi(provider._underlying);
    return result.isOk
        ? ICU4XWordSegmenter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createDictionaryFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XWordSegmenter_create_dictionary')
      .asFunction<_ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);

  /// Segments a (potentially ill-formed) UTF-8 string.
  ///
  /// See the [Rust documentation for `segment_utf8`](https://docs.rs/icu/latest/icu/segmenter/struct.WordSegmenter.html#method.segment_utf8) for more information.
  ICU4XWordBreakIteratorUtf8 segmentUtf8(String input) {
    final alloc = ffi2.Arena();
    final inputSlice = _SliceFfi2Utf8._fromDart(input, alloc);

    final result =
        _segmentUtf8Ffi(_underlying, inputSlice._bytes, inputSlice._length);
    alloc.releaseAll();
    return ICU4XWordBreakIteratorUtf8._(result);
  }

  static final _segmentUtf8Ffi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi2.Utf8>,
                  ffi.Size)>>('ICU4XWordSegmenter_segment_utf8')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi2.Utf8>, int)>(isLeaf: true);

  /// Segments a UTF-16 string.
  ///
  /// See the [Rust documentation for `segment_utf16`](https://docs.rs/icu/latest/icu/segmenter/struct.WordSegmenter.html#method.segment_utf16) for more information.
  ICU4XWordBreakIteratorUtf16 segmentUtf16(Uint16List input) {
    final alloc = ffi2.Arena();
    final inputSlice = _SliceFfiUint16._fromDart(input, alloc);

    final result =
        _segmentUtf16Ffi(_underlying, inputSlice._bytes, inputSlice._length);
    alloc.releaseAll();
    return ICU4XWordBreakIteratorUtf16._(result);
  }

  static final _segmentUtf16Ffi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Uint16>,
                  ffi.Size)>>('ICU4XWordSegmenter_segment_utf16')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Uint16>, int)>(isLeaf: true);

  /// Segments a Latin-1 string.
  ///
  /// See the [Rust documentation for `segment_latin1`](https://docs.rs/icu/latest/icu/segmenter/struct.WordSegmenter.html#method.segment_latin1) for more information.
  ICU4XWordBreakIteratorLatin1 segmentLatin1(Uint8List input) {
    final alloc = ffi2.Arena();
    final inputSlice = _SliceFfiUint8._fromDart(input, alloc);

    final result =
        _segmentLatin1Ffi(_underlying, inputSlice._bytes, inputSlice._length);
    alloc.releaseAll();
    return ICU4XWordBreakIteratorLatin1._(result);
  }

  static final _segmentLatin1Ffi = _capi<
          ffi.NativeFunction<
              ffi.Pointer<ffi.Opaque> Function(
                  ffi.Pointer<ffi.Opaque>,
                  ffi.Pointer<ffi.Uint8>,
                  ffi.Size)>>('ICU4XWordSegmenter_segment_latin1')
      .asFunction<
          ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);
}

/// An object capable of formatting a date time with time zone to a string.
///
/// See the [Rust documentation for `ZonedDateTimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.ZonedDateTimeFormatter.html) for more information.
class ICU4XZonedDateTimeFormatter implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ICU4XZonedDateTimeFormatter._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer =
      ffi.NativeFinalizer(_capi('ICU4XZonedDateTimeFormatter_destroy'));

  /// Creates a new [`ICU4XZonedDateTimeFormatter`] from locale data.
  ///
  /// This function has `date_length` and `time_length` arguments and uses default options
  /// for the time zone.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.ZonedDateTimeFormatter.html#method.try_new) for more information.
  factory ICU4XZonedDateTimeFormatter.withLengths(
      ICU4XDataProvider provider,
      ICU4XLocale locale,
      ICU4XDateLength dateLength,
      ICU4XTimeLength timeLength) {
    final result = _createWithLengthsFfi(provider._underlying,
        locale._underlying, dateLength._id, timeLength._id);
    return result.isOk
        ? ICU4XZonedDateTimeFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithLengthsFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>, ffi.Uint32, ffi.Uint32)>>(
          'ICU4XZonedDateTimeFormatter_create_with_lengths')
      .asFunction<
          _ResultOpaqueUint32 Function(ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>, int, int)>(isLeaf: true);

  /// Creates a new [`ICU4XZonedDateTimeFormatter`] from locale data.
  ///
  /// This function has `date_length` and `time_length` arguments and uses an ISO-8601 style
  /// fallback for the time zone with the given configurations.
  ///
  /// See the [Rust documentation for `try_new`](https://docs.rs/icu/latest/icu/datetime/struct.ZonedDateTimeFormatter.html#method.try_new) for more information.
  factory ICU4XZonedDateTimeFormatter.withLengthsAndIso8601TimeZoneFallback(
      ICU4XDataProvider provider,
      ICU4XLocale locale,
      ICU4XDateLength dateLength,
      ICU4XTimeLength timeLength,
      ICU4XIsoTimeZoneOptions zoneOptions) {
    final result = _createWithLengthsAndIso8601TimeZoneFallbackFfi(
        provider._underlying,
        locale._underlying,
        dateLength._id,
        timeLength._id,
        zoneOptions._underlying);
    return result.isOk
        ? ICU4XZonedDateTimeFormatter._(result.union.ok)
        : throw ICU4XError._(result.union.err);
  }
  static final _createWithLengthsAndIso8601TimeZoneFallbackFfi = _capi<
              ffi.NativeFunction<
                  _ResultOpaqueUint32 Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Uint32,
                      ffi.Uint32,
                      _ICU4XIsoTimeZoneOptionsFfi)>>(
          'ICU4XZonedDateTimeFormatter_create_with_lengths_and_iso_8601_time_zone_fallback')
      .asFunction<
          _ResultOpaqueUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              int,
              int,
              _ICU4XIsoTimeZoneOptionsFfi)>(isLeaf: true);

  /// Formats a [`ICU4XDateTime`] and [`ICU4XCustomTimeZone`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.ZonedDateTimeFormatter.html#method.format) for more information.
  String formatDatetimeWithCustomTimeZone(
      ICU4XDateTime datetime, ICU4XCustomTimeZone timeZone) {
    final writeable = _Writeable();
    final result = _formatDatetimeWithCustomTimeZoneFfi(_underlying,
        datetime._underlying, timeZone._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatDatetimeWithCustomTimeZoneFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XZonedDateTimeFormatter_format_datetime_with_custom_time_zone')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  /// Formats a [`ICU4XIsoDateTime`] and [`ICU4XCustomTimeZone`] to a string.
  ///
  /// See the [Rust documentation for `format`](https://docs.rs/icu/latest/icu/datetime/struct.ZonedDateTimeFormatter.html#method.format) for more information.
  String formatIsoDatetimeWithCustomTimeZone(
      ICU4XIsoDateTime datetime, ICU4XCustomTimeZone timeZone) {
    final writeable = _Writeable();
    final result = _formatIsoDatetimeWithCustomTimeZoneFfi(_underlying,
        datetime._underlying, timeZone._underlying, writeable._underlying);
    return result.isOk
        ? writeable.finalize()
        : throw ICU4XError._(result.union.err);
  }

  static final _formatIsoDatetimeWithCustomTimeZoneFfi = _capi<
              ffi.NativeFunction<
                  _ResultVoidUint32 Function(
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>,
                      ffi.Pointer<ffi.Opaque>)>>(
          'ICU4XZonedDateTimeFormatter_format_iso_datetime_with_custom_time_zone')
      .asFunction<
          _ResultVoidUint32 Function(
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>,
              ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}

class SizeList extends ffi.Struct {
  external ffi.Pointer<ffi.Size> _bytes;

  @ffi.Size()
  external int _length;

  // ignore: unused_element
  SizeList get _asDart => this;

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! SizeList || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes[i] != _bytes[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

class _ResultBoolUint32Union extends ffi.Union {
  @ffi.Bool()
  external bool ok;

  @ffi.Uint32()
  external int err;
}

class _ResultBoolUint32 extends ffi.Struct {
  external _ResultBoolUint32Union union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultICU4XWeekOfFfiUint32Union extends ffi.Union {
  external _ICU4XWeekOfFfi ok;

  @ffi.Uint32()
  external int err;
}

class _ResultICU4XWeekOfFfiUint32 extends ffi.Struct {
  external _ResultICU4XWeekOfFfiUint32Union union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultInt32Uint32Union extends ffi.Union {
  @ffi.Int32()
  external int ok;

  @ffi.Uint32()
  external int err;
}

class _ResultInt32Uint32 extends ffi.Struct {
  external _ResultInt32Uint32Union union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultOpaqueUint32Union extends ffi.Union {
  external ffi.Pointer<ffi.Opaque> ok;

  @ffi.Uint32()
  external int err;
}

class _ResultOpaqueUint32 extends ffi.Struct {
  external _ResultOpaqueUint32Union union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultUint16VoidUnion extends ffi.Union {
  @ffi.Uint16()
  external int ok;
}

class _ResultUint16Void extends ffi.Struct {
  external _ResultUint16VoidUnion union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultUint32VoidUnion extends ffi.Union {
  @ffi.Uint32()
  external int ok;
}

class _ResultUint32Void extends ffi.Struct {
  external _ResultUint32VoidUnion union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultVoidUint32Union extends ffi.Union {
  @ffi.Uint32()
  external int err;
}

class _ResultVoidUint32 extends ffi.Struct {
  external _ResultVoidUint32Union union;

  @ffi.Bool()
  external bool isOk;
}

class _ResultVoidVoid extends ffi.Struct {
  @ffi.Bool()
  external bool isOk;
}

class _SliceFfi2Utf8 extends ffi.Struct {
  external ffi.Pointer<ffi2.Utf8> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfi2Utf8 _fromDart(String value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfi2Utf8>();
    final slice = pointer.ref;
    final units = Utf8Encoder().convert(value);
    slice._length = units.length;
    slice._bytes = allocator<ffi.Uint8>(slice._length).cast();
    slice._bytes.cast<ffi.Uint8>().asTypedList(slice._length).setAll(0, units);

    return slice;
  }

  // ignore: unused_element
  String get _asDart =>
      Utf8Decoder().convert(_bytes.cast<ffi.Uint8>().asTypedList(_length));

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfi2Utf8 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes.cast<ffi.Uint8>()[i] != _bytes.cast<ffi.Uint8>()[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

class _SliceFfiUint16 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint16> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfiUint16 _fromDart(Uint16List value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfiUint16>();
    final slice = pointer.ref;
    slice._length = value.length;
    slice._bytes = allocator(slice._length);
    slice._bytes.asTypedList(slice._length).setAll(0, value);

    return slice;
  }

  // ignore: unused_element
  Uint16List get _asDart => _bytes.asTypedList(_length);

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfiUint16 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes[i] != _bytes[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

class _SliceFfiUint32 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint32> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfiUint32 _fromDart(Uint32List value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfiUint32>();
    final slice = pointer.ref;
    slice._length = value.length;
    slice._bytes = allocator(slice._length);
    slice._bytes.asTypedList(slice._length).setAll(0, value);

    return slice;
  }

  // ignore: unused_element
  Uint32List get _asDart => _bytes.asTypedList(_length);

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfiUint32 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes[i] != _bytes[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

class _SliceFfiUint8 extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> _bytes;

  @ffi.Size()
  external int _length;

  /// Produces a slice from a Dart object. The Dart object's data is copied into the given allocator
  /// as it cannot be borrowed directly, and gets freed with the slice object.
  // ignore: unused_element
  static _SliceFfiUint8 _fromDart(Uint8List value, ffi.Allocator allocator) {
    final pointer = allocator<_SliceFfiUint8>();
    final slice = pointer.ref;
    slice._length = value.length;
    slice._bytes = allocator(slice._length);
    slice._bytes.asTypedList(slice._length).setAll(0, value);

    return slice;
  }

  // ignore: unused_element
  Uint8List get _asDart => _bytes.asTypedList(_length);

  // This is expensive
  @override
  bool operator ==(Object other) {
    if (other is! _SliceFfiUint8 || other._length != _length) {
      return false;
    }

    for (var i = 0; i < _length; i++) {
      if (other._bytes[i] != _bytes[i]) {
        return false;
      }
    }
    return true;
  }

  // This is cheap
  @override
  int get hashCode => _length.hashCode;
}

/// An unspecified error value
class VoidError {}

class _Writeable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  _Writeable() : _underlying = _create(0);
  static final _create =
      _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Size)>>(
              'diplomat_buffer_writeable_create')
          .asFunction<ffi.Pointer<ffi.Opaque> Function(int)>();

  String finalize() {
    final string =
        _getBytes(_underlying).toDartString(length: _len(_underlying));
    _destroy(_underlying);
    return string;
  }

  static final _len =
      _capi<ffi.NativeFunction<ffi.Size Function(ffi.Pointer<ffi.Opaque>)>>(
              'diplomat_buffer_writeable_len')
          .asFunction<int Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
  static final _getBytes = _capi<
              ffi.NativeFunction<
                  ffi.Pointer<ffi2.Utf8> Function(ffi.Pointer<ffi.Opaque>)>>(
          'diplomat_buffer_writeable_get_bytes')
      .asFunction<ffi.Pointer<ffi2.Utf8> Function(ffi.Pointer<ffi.Opaque>)>(
          isLeaf: true);
  static final _destroy =
      _capi<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Opaque>)>>(
              'diplomat_buffer_writeable_destroy')
          .asFunction<void Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}
