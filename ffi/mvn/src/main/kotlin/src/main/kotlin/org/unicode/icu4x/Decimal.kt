package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface DecimalLib: Library {
    fun icu4x_Decimal_destroy_mv1(handle: Pointer)
    fun icu4x_Decimal_from_int32_mv1(v: Int): Pointer
    fun icu4x_Decimal_from_uint32_mv1(v: FFIUint32): Pointer
    fun icu4x_Decimal_from_int64_mv1(v: Long): Pointer
    fun icu4x_Decimal_from_uint64_mv1(v: FFIUint64): Pointer
    fun icu4x_Decimal_from_double_with_integer_precision_mv1(f: Double): ResultPointerDecimalLimitErrorNative
    fun icu4x_Decimal_from_double_with_lower_magnitude_mv1(f: Double, magnitude: Short): ResultPointerDecimalLimitErrorNative
    fun icu4x_Decimal_from_double_with_significant_digits_mv1(f: Double, digits: FFIUint8): ResultPointerDecimalLimitErrorNative
    fun icu4x_Decimal_from_double_with_round_trip_precision_mv1(f: Double): ResultPointerDecimalLimitErrorNative
    fun icu4x_Decimal_from_string_mv1(v: Slice): ResultPointerInt
    fun icu4x_Decimal_digit_at_mv1(handle: Pointer, magnitude: Short): FFIUint8
    fun icu4x_Decimal_magnitude_start_mv1(handle: Pointer): Short
    fun icu4x_Decimal_magnitude_end_mv1(handle: Pointer): Short
    fun icu4x_Decimal_nonzero_magnitude_start_mv1(handle: Pointer): Short
    fun icu4x_Decimal_nonzero_magnitude_end_mv1(handle: Pointer): Short
    fun icu4x_Decimal_is_zero_mv1(handle: Pointer): Byte
    fun icu4x_Decimal_multiply_pow10_mv1(handle: Pointer, power: Short): Unit
    fun icu4x_Decimal_sign_mv1(handle: Pointer): Int
    fun icu4x_Decimal_set_sign_mv1(handle: Pointer, sign: Int): Unit
    fun icu4x_Decimal_apply_sign_display_mv1(handle: Pointer, signDisplay: Int): Unit
    fun icu4x_Decimal_trim_start_mv1(handle: Pointer): Unit
    fun icu4x_Decimal_trim_end_mv1(handle: Pointer): Unit
    fun icu4x_Decimal_trim_end_if_integer_mv1(handle: Pointer): Unit
    fun icu4x_Decimal_pad_start_mv1(handle: Pointer, position: Short): Unit
    fun icu4x_Decimal_pad_end_mv1(handle: Pointer, position: Short): Unit
    fun icu4x_Decimal_set_max_position_mv1(handle: Pointer, position: Short): Unit
    fun icu4x_Decimal_round_mv1(handle: Pointer, position: Short): Unit
    fun icu4x_Decimal_ceil_mv1(handle: Pointer, position: Short): Unit
    fun icu4x_Decimal_expand_mv1(handle: Pointer, position: Short): Unit
    fun icu4x_Decimal_floor_mv1(handle: Pointer, position: Short): Unit
    fun icu4x_Decimal_trunc_mv1(handle: Pointer, position: Short): Unit
    fun icu4x_Decimal_round_with_mode_mv1(handle: Pointer, position: Short, mode: Int): Unit
    fun icu4x_Decimal_round_with_mode_and_increment_mv1(handle: Pointer, position: Short, mode: Int, increment: Int): Unit
    fun icu4x_Decimal_concatenate_end_mv1(handle: Pointer, other: Pointer): ResultUnitUnit
    fun icu4x_Decimal_to_string_mv1(handle: Pointer, write: Pointer): Unit
}
/** See the [Rust documentation for `Decimal`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html) for more information.
*/
class Decimal internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class DecimalCleaner(val handle: Pointer, val lib: DecimalLib) : Runnable {
        override fun run() {
            lib.icu4x_Decimal_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<DecimalLib> = DecimalLib::class.java
        internal val lib: DecimalLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct an [Decimal] from an integer.
        *
        *See the [Rust documentation for `Decimal`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html) for more information.
        */
        fun from(v: Int): Decimal {
            
            val returnVal = lib.icu4x_Decimal_from_int32_mv1(v);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Decimal(handle, selfEdges)
            CLEANER.register(returnOpaque, Decimal.DecimalCleaner(handle, Decimal.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct an [Decimal] from an integer.
        *
        *See the [Rust documentation for `Decimal`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html) for more information.
        */
        fun from(v: UInt): Decimal {
            
            val returnVal = lib.icu4x_Decimal_from_uint32_mv1(FFIUint32(v));
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Decimal(handle, selfEdges)
            CLEANER.register(returnOpaque, Decimal.DecimalCleaner(handle, Decimal.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct an [Decimal] from an integer.
        *
        *See the [Rust documentation for `Decimal`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html) for more information.
        */
        fun from(v: Long): Decimal {
            
            val returnVal = lib.icu4x_Decimal_from_int64_mv1(v);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Decimal(handle, selfEdges)
            CLEANER.register(returnOpaque, Decimal.DecimalCleaner(handle, Decimal.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct an [Decimal] from an integer.
        *
        *See the [Rust documentation for `Decimal`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html) for more information.
        */
        fun from(v: ULong): Decimal {
            
            val returnVal = lib.icu4x_Decimal_from_uint64_mv1(FFIUint64(v));
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = Decimal(handle, selfEdges)
            CLEANER.register(returnOpaque, Decimal.DecimalCleaner(handle, Decimal.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct an [Decimal] from an integer-valued float
        *
        *See the [Rust documentation for `try_from_f64`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.try_from_f64) for more information.
        *
        *See the [Rust documentation for `FloatPrecision`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/enum.FloatPrecision.html) for more information.
        */
        fun fromDoubleWithIntegerPrecision(f: Double): Result<Decimal> {
            
            val returnVal = lib.icu4x_Decimal_from_double_with_integer_precision_mv1(f);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Decimal(handle, selfEdges)
                CLEANER.register(returnOpaque, Decimal.DecimalCleaner(handle, Decimal.lib));
                return returnOpaque.ok()
            } else {
                return DecimalLimitError().err()
            }
        }
        @JvmStatic
        
        /** Construct an [Decimal] from an float, with a given power of 10 for the lower magnitude
        *
        *See the [Rust documentation for `try_from_f64`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.try_from_f64) for more information.
        *
        *See the [Rust documentation for `FloatPrecision`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/enum.FloatPrecision.html) for more information.
        */
        fun fromDoubleWithLowerMagnitude(f: Double, magnitude: Short): Result<Decimal> {
            
            val returnVal = lib.icu4x_Decimal_from_double_with_lower_magnitude_mv1(f, magnitude);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Decimal(handle, selfEdges)
                CLEANER.register(returnOpaque, Decimal.DecimalCleaner(handle, Decimal.lib));
                return returnOpaque.ok()
            } else {
                return DecimalLimitError().err()
            }
        }
        @JvmStatic
        
        /** Construct an [Decimal] from an float, for a given number of significant digits
        *
        *See the [Rust documentation for `try_from_f64`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.try_from_f64) for more information.
        *
        *See the [Rust documentation for `FloatPrecision`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/enum.FloatPrecision.html) for more information.
        */
        fun fromDoubleWithSignificantDigits(f: Double, digits: UByte): Result<Decimal> {
            
            val returnVal = lib.icu4x_Decimal_from_double_with_significant_digits_mv1(f, FFIUint8(digits));
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Decimal(handle, selfEdges)
                CLEANER.register(returnOpaque, Decimal.DecimalCleaner(handle, Decimal.lib));
                return returnOpaque.ok()
            } else {
                return DecimalLimitError().err()
            }
        }
        @JvmStatic
        
        /** Construct an [Decimal] from an float, with enough digits to recover
        *the original floating point in IEEE 754 without needing trailing zeros
        *
        *See the [Rust documentation for `try_from_f64`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.try_from_f64) for more information.
        *
        *See the [Rust documentation for `FloatPrecision`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/enum.FloatPrecision.html) for more information.
        */
        fun fromDoubleWithRoundTripPrecision(f: Double): Result<Decimal> {
            
            val returnVal = lib.icu4x_Decimal_from_double_with_round_trip_precision_mv1(f);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Decimal(handle, selfEdges)
                CLEANER.register(returnOpaque, Decimal.DecimalCleaner(handle, Decimal.lib));
                return returnOpaque.ok()
            } else {
                return DecimalLimitError().err()
            }
        }
        @JvmStatic
        
        /** Construct an [Decimal] from a string.
        *
        *See the [Rust documentation for `try_from_str`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.try_from_str) for more information.
        */
        fun fromString(v: String): Result<Decimal> {
            val (vMem, vSlice) = PrimitiveArrayTools.borrowUtf8(v)
            
            val returnVal = lib.icu4x_Decimal_from_string_mv1(vSlice);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = Decimal(handle, selfEdges)
                CLEANER.register(returnOpaque, Decimal.DecimalCleaner(handle, Decimal.lib));
                if (vMem != null) vMem.close()
                return returnOpaque.ok()
            } else {
                return DecimalParseErrorError(DecimalParseError.fromNative(returnVal.union.err)).err()
            }
        }
    }
    
    /** See the [Rust documentation for `digit_at`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.digit_at) for more information.
    */
    fun digitAt(magnitude: Short): UByte {
        
        val returnVal = lib.icu4x_Decimal_digit_at_mv1(handle, magnitude);
        return (returnVal.toUByte())
    }
    
    /** See the [Rust documentation for `magnitude_range`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.magnitude_range) for more information.
    */
    fun magnitudeStart(): Short {
        
        val returnVal = lib.icu4x_Decimal_magnitude_start_mv1(handle);
        return (returnVal)
    }
    
    /** See the [Rust documentation for `magnitude_range`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.magnitude_range) for more information.
    */
    fun magnitudeEnd(): Short {
        
        val returnVal = lib.icu4x_Decimal_magnitude_end_mv1(handle);
        return (returnVal)
    }
    
    /** See the [Rust documentation for `nonzero_magnitude_start`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.nonzero_magnitude_start) for more information.
    */
    fun nonzeroMagnitudeStart(): Short {
        
        val returnVal = lib.icu4x_Decimal_nonzero_magnitude_start_mv1(handle);
        return (returnVal)
    }
    
    /** See the [Rust documentation for `nonzero_magnitude_end`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.nonzero_magnitude_end) for more information.
    */
    fun nonzeroMagnitudeEnd(): Short {
        
        val returnVal = lib.icu4x_Decimal_nonzero_magnitude_end_mv1(handle);
        return (returnVal)
    }
    
    /** See the [Rust documentation for `is_zero`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.is_zero) for more information.
    */
    fun isZero(): Boolean {
        
        val returnVal = lib.icu4x_Decimal_is_zero_mv1(handle);
        return (returnVal > 0)
    }
    
    /** Multiply the [Decimal] by a given power of ten.
    *
    *See the [Rust documentation for `multiply_pow10`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.multiply_pow10) for more information.
    */
    fun multiplyPow10(power: Short): Unit {
        
        val returnVal = lib.icu4x_Decimal_multiply_pow10_mv1(handle, power);
        
    }
    
    /** See the [Rust documentation for `sign`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.sign) for more information.
    */
    fun sign(): DecimalSign {
        
        val returnVal = lib.icu4x_Decimal_sign_mv1(handle);
        return (DecimalSign.fromNative(returnVal))
    }
    
    /** Set the sign of the [Decimal].
    *
    *See the [Rust documentation for `set_sign`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.set_sign) for more information.
    */
    fun setSign(sign: DecimalSign): Unit {
        
        val returnVal = lib.icu4x_Decimal_set_sign_mv1(handle, sign.toNative());
        
    }
    
    /** See the [Rust documentation for `apply_sign_display`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.apply_sign_display) for more information.
    */
    fun applySignDisplay(signDisplay: DecimalSignDisplay): Unit {
        
        val returnVal = lib.icu4x_Decimal_apply_sign_display_mv1(handle, signDisplay.toNative());
        
    }
    
    /** See the [Rust documentation for `trim_start`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.trim_start) for more information.
    */
    fun trimStart(): Unit {
        
        val returnVal = lib.icu4x_Decimal_trim_start_mv1(handle);
        
    }
    
    /** See the [Rust documentation for `trim_end`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.trim_end) for more information.
    */
    fun trimEnd(): Unit {
        
        val returnVal = lib.icu4x_Decimal_trim_end_mv1(handle);
        
    }
    
    /** See the [Rust documentation for `trim_end_if_integer`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.trim_end_if_integer) for more information.
    */
    fun trimEndIfInteger(): Unit {
        
        val returnVal = lib.icu4x_Decimal_trim_end_if_integer_mv1(handle);
        
    }
    
    /** Zero-pad the [Decimal] on the left to a particular position
    *
    *See the [Rust documentation for `pad_start`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.pad_start) for more information.
    */
    fun padStart(position: Short): Unit {
        
        val returnVal = lib.icu4x_Decimal_pad_start_mv1(handle, position);
        
    }
    
    /** Zero-pad the [Decimal] on the right to a particular position
    *
    *See the [Rust documentation for `pad_end`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.pad_end) for more information.
    */
    fun padEnd(position: Short): Unit {
        
        val returnVal = lib.icu4x_Decimal_pad_end_mv1(handle, position);
        
    }
    
    /** Truncate the [Decimal] on the left to a particular position, deleting digits if necessary. This is useful for, e.g. abbreviating years
    *("2022" -> "22")
    *
    *See the [Rust documentation for `set_max_position`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.set_max_position) for more information.
    */
    fun setMaxPosition(position: Short): Unit {
        
        val returnVal = lib.icu4x_Decimal_set_max_position_mv1(handle, position);
        
    }
    
    /** Round the number at a particular digit position.
    *
    *This uses half to even rounding, which resolves ties by selecting the nearest
    *even integer to the original value.
    *
    *See the [Rust documentation for `round`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.round) for more information.
    */
    fun round(position: Short): Unit {
        
        val returnVal = lib.icu4x_Decimal_round_mv1(handle, position);
        
    }
    
    /** See the [Rust documentation for `ceil`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.ceil) for more information.
    */
    fun ceil(position: Short): Unit {
        
        val returnVal = lib.icu4x_Decimal_ceil_mv1(handle, position);
        
    }
    
    /** See the [Rust documentation for `expand`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.expand) for more information.
    */
    fun expand(position: Short): Unit {
        
        val returnVal = lib.icu4x_Decimal_expand_mv1(handle, position);
        
    }
    
    /** See the [Rust documentation for `floor`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.floor) for more information.
    */
    fun floor(position: Short): Unit {
        
        val returnVal = lib.icu4x_Decimal_floor_mv1(handle, position);
        
    }
    
    /** See the [Rust documentation for `trunc`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.trunc) for more information.
    */
    fun trunc(position: Short): Unit {
        
        val returnVal = lib.icu4x_Decimal_trunc_mv1(handle, position);
        
    }
    
    /** See the [Rust documentation for `round_with_mode`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.round_with_mode) for more information.
    */
    fun roundWithMode(position: Short, mode: DecimalSignedRoundingMode): Unit {
        
        val returnVal = lib.icu4x_Decimal_round_with_mode_mv1(handle, position, mode.toNative());
        
    }
    
    /** See the [Rust documentation for `round_with_mode_and_increment`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.round_with_mode_and_increment) for more information.
    */
    fun roundWithModeAndIncrement(position: Short, mode: DecimalSignedRoundingMode, increment: DecimalRoundingIncrement): Unit {
        
        val returnVal = lib.icu4x_Decimal_round_with_mode_and_increment_mv1(handle, position, mode.toNative(), increment.toNative());
        
    }
    
    /** Concatenates `other` to the end of `self`.
    *
    *If successful, `other` will be set to 0 and a successful status is returned.
    *
    *If not successful, `other` will be unchanged and an error is returned.
    *
    *See the [Rust documentation for `concatenate_end`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.concatenate_end) for more information.
    */
    fun concatenateEnd(other: Decimal): Result<Unit> {
        
        val returnVal = lib.icu4x_Decimal_concatenate_end_mv1(handle, other.handle /* note this is a mutable reference. Think carefully about using, especially concurrently */);
        if (returnVal.isOk == 1.toByte()) {
            return Unit.ok()
        } else {
            return UnitError().err()
        }
    }
    
    /** Format the [Decimal] as a string.
    *
    *See the [Rust documentation for `write_to`](https://docs.rs/fixed_decimal/0.7.0/fixed_decimal/type.Decimal.html#method.write_to) for more information.
    */
    override fun toString(): String {
        val write = DW.lib.diplomat_buffer_write_create(0)
        val returnVal = lib.icu4x_Decimal_to_string_mv1(handle, write);
        
        val returnString = DW.writeToString(write)
        return returnString
    }

}