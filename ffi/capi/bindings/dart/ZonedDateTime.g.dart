// generated by diplomat-tool

part of 'lib.g.dart';

final class _ZonedDateTimeFfi extends ffi.Struct {
  external ffi.Pointer<ffi.Opaque> date;
  external ffi.Pointer<ffi.Opaque> time;
  external ffi.Pointer<ffi.Opaque> zone;
}

/// An ICU4X DateTime object capable of containing a date, time, and zone for any calendar.
///
/// See the [Rust documentation for `ZonedDateTime`](https://docs.rs/icu/latest/icu/timezone/struct.ZonedDateTime.html) for more information.
final class ZonedDateTime {
  final Date date;
  final Time time;
  final TimeZoneInfo zone;

  // This struct contains borrowed fields, so this takes in a list of
  // "edges" corresponding to where each lifetime's data may have been borrowed from
  // and passes it down to individual fields containing the borrow.
  // This method does not attempt to handle any dependencies between lifetimes, the caller
  // should handle this when constructing edge arrays.
  // ignore: unused_element
  ZonedDateTime._fromFfi(_ZonedDateTimeFfi ffi) :
    date = Date._fromFfi(ffi.date, []),
    time = Time._fromFfi(ffi.time, []),
    zone = TimeZoneInfo._fromFfi(ffi.zone, []);

  // ignore: unused_element
  _ZonedDateTimeFfi _toFfi(ffi.Allocator temp) {
    final struct = ffi.Struct.create<_ZonedDateTimeFfi>();
    struct.date = date._ffi;
    struct.time = time._ffi;
    struct.zone = zone._ffi;
    return struct;
  }

  @override
  bool operator ==(Object other) =>
      other is ZonedDateTime &&
      other.date == date &&
      other.time == time &&
      other.zone == zone;

  @override
  int get hashCode => Object.hashAll([
        date,
        time,
        zone,
      ]);
}