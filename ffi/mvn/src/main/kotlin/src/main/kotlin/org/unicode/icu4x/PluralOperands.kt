package org.unicode.icu4x;
import com.sun.jna.Callback
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure

internal interface PluralOperandsLib: Library {
    fun icu4x_PluralOperands_destroy_mv1(handle: Pointer)
    fun icu4x_PluralOperands_from_string_mv1(s: Slice): ResultPointerInt
    fun icu4x_PluralOperands_from_int64_mv1(i: Long): Pointer
    fun icu4x_PluralOperands_from_fixed_decimal_mv1(x: Pointer): Pointer
}
/** See the [Rust documentation for `PluralOperands`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralOperands.html) for more information.
*/
class PluralOperands internal constructor (
    internal val handle: Pointer,
    // These ensure that anything that is borrowed is kept alive and not cleaned
    // up by the garbage collector.
    internal val selfEdges: List<Any>,
)  {

    internal class PluralOperandsCleaner(val handle: Pointer, val lib: PluralOperandsLib) : Runnable {
        override fun run() {
            lib.icu4x_PluralOperands_destroy_mv1(handle)
        }
    }

    companion object {
        internal val libClass: Class<PluralOperandsLib> = PluralOperandsLib::class.java
        internal val lib: PluralOperandsLib = Native.load("icu4x", libClass)
        @JvmStatic
        
        /** Construct for a given string representing a number
        *
        *See the [Rust documentation for `from_str`](https://docs.rs/icu/2.1.1/icu/plurals/struct.PluralOperands.html#method.from_str) for more information.
        */
        fun fromString(s: String): Result<PluralOperands> {
            val (sMem, sSlice) = PrimitiveArrayTools.borrowUtf8(s)
            
            val returnVal = lib.icu4x_PluralOperands_from_string_mv1(sSlice);
            if (returnVal.isOk == 1.toByte()) {
                val selfEdges: List<Any> = listOf()
                val handle = returnVal.union.ok 
                val returnOpaque = PluralOperands(handle, selfEdges)
                CLEANER.register(returnOpaque, PluralOperands.PluralOperandsCleaner(handle, PluralOperands.lib));
                if (sMem != null) sMem.close()
                return returnOpaque.ok()
            } else {
                return DecimalParseErrorError(DecimalParseError.fromNative(returnVal.union.err)).err()
            }
        }
        @JvmStatic
        
        /** Construct for a given integer
        */
        fun from(i: Long): PluralOperands {
            
            val returnVal = lib.icu4x_PluralOperands_from_int64_mv1(i);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PluralOperands(handle, selfEdges)
            CLEANER.register(returnOpaque, PluralOperands.PluralOperandsCleaner(handle, PluralOperands.lib));
            return returnOpaque
        }
        @JvmStatic
        
        /** Construct from a FixedDecimal
        *
        *Retains at most 18 digits each from the integer and fraction parts.
        */
        fun fromFixedDecimal(x: Decimal): PluralOperands {
            
            val returnVal = lib.icu4x_PluralOperands_from_fixed_decimal_mv1(x.handle);
            val selfEdges: List<Any> = listOf()
            val handle = returnVal 
            val returnOpaque = PluralOperands(handle, selfEdges)
            CLEANER.register(returnOpaque, PluralOperands.PluralOperandsCleaner(handle, PluralOperands.lib));
            return returnOpaque
        }
    }

}