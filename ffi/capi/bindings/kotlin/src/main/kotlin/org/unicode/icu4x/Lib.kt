package org.unicode.icu4x;

import com.sun.jna.JNIEnv
import com.sun.jna.Library
import com.sun.jna.Memory
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure
import com.sun.jna.Union
import java.util.Collections

// We spawn a cleaner for the library which is responsible for cleaning opaque types.
val CLEANER = java.lang.ref.Cleaner.create()

interface DiplomatWriteLib: Library {
    fun diplomat_buffer_write_create(size: Long): Pointer 
    fun diplomat_buffer_write_get_bytes(diplomatWrite: Pointer): Pointer
    fun diplomat_buffer_write_len(diplomatWrite: Pointer): Long
    fun diplomat_buffer_write_destroy(diplomatWrite: Pointer)
}


object DW {

    val libClass: Class<DiplomatWriteLib> = DiplomatWriteLib::class.java
    val lib: DiplomatWriteLib = Native.load("icu4x", libClass)

    fun writeToString (write: Pointer): String {
        try {
            val pointer = lib.diplomat_buffer_write_get_bytes(write)
            if (pointer == null) {
                throw OutOfMemoryError();
            }
            val len = lib.diplomat_buffer_write_len(write)
            val bytes = pointer.getByteArray(0, len.toInt())
            return bytes.decodeToString();
        } finally {
            lib.diplomat_buffer_write_destroy(write);
        }
    }
}

internal interface DiplomatJVMRuntimeLib: Library {
    fun create_rust_jvm_cookie(env: JNIEnv, obj: Object): Pointer
    fun destroy_rust_jvm_cookie(obj_pointer: Pointer): Unit
}

internal class DiplomatJVMRuntime {
    companion object {
        val libClass: Class<DiplomatJVMRuntimeLib> = DiplomatJVMRuntimeLib::class.java
        val lib: DiplomatJVMRuntimeLib = Native.load("icu4x", libClass, Collections.singletonMap(Library.OPTION_ALLOW_OBJECTS, true))

        fun buildRustCookie(obj: Object): Pointer {
            return lib.create_rust_jvm_cookie(JNIEnv.CURRENT, obj);
        }

        fun dropRustCookie(obj_pointer: Pointer): Unit {
            lib.destroy_rust_jvm_cookie(obj_pointer);
        }
    }
}

interface DiplomatAllocateLib: Library {
    fun diplomat_alloc(size: Long, align: Long): Pointer
}



internal object PrimitiveArrayTools {

    val libClass: Class<DiplomatAllocateLib> = DiplomatAllocateLib::class.java
    val lib: DiplomatAllocateLib = Native.load("icu4x", libClass)

    fun allocateGarbageCollectedMemory(size: Long): Memory? {
        // we can't use the Memory constructor for a memory of size 0
        // so, if the size is zero, then we return null
        if (size > 0L)
            return Memory(size)
        else
            return null
    }

    fun allocateOwnedMemory(size: Long, align: Long): Pointer? {
        // we can't use the Memory constructor for a memory of size 0
        // so, if the size is zero, then we return null
        if (size > 0L)
            return lib.diplomat_alloc(size, align)
        else
            return null
    }

    val boolAlign: Long =  1
    val byteAlign: Long =  1
    val shortAlign: Long =  2
    val intalign: Long =  4
    val longAlign: Long =  8
    val uByteAlign: Long =  1
    val uShortAlign: Long =  2
    val uIntalign: Long =  4
    val uLongAlign: Long =  8
    val floatAlign: Long =  4
    val doubleAlign: Long =  8


    fun copy(arr: ByteArray, ptr: Pointer?) : Slice {
        val slice = Slice()
        slice.data = if (ptr != null) {
            ptr.write(0, arr, 0, arr.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(arr.size.toLong().toULong())
        return slice
    }

    fun borrow(boolArray: BooleanArray): Pair<Memory?, Slice> {
        val mem = allocateGarbageCollectedMemory(boolArray.size.toLong())
        val byteArray = boolArray.map {if (it) 1.toByte() else 0.toByte() }.toByteArray()
        val slice = copy(byteArray, mem)
        return Pair(mem, slice)
    }

    fun move(boolArray: BooleanArray): Pair<Pointer?, Slice> {
        val mem = allocateOwnedMemory(boolArray.size.toLong() * boolAlign, boolAlign)
        val byteArray = boolArray.map {if (it) 1.toByte() else 0.toByte() }.toByteArray()
        val slice = copy(byteArray, mem)
        return Pair(mem, slice)
    }

    fun borrow(byteArray: ByteArray): Pair<Memory?, Slice> {
        val mem = allocateGarbageCollectedMemory(byteArray.size.toLong())
        val slice = copy(byteArray, mem)
        return Pair(mem, slice)
    }

    fun move(byteArray: ByteArray): Pair<Pointer?, Slice> {
        val mem = allocateOwnedMemory(byteArray.size.toLong() * Byte.SIZE_BYTES.toLong(), uByteAlign, )
        val slice = copy(byteArray, mem)
        return Pair(mem, slice)
    }


    fun borrow(uByteArray: UByteArray): Pair<Memory?, Slice> {
        val mem = allocateGarbageCollectedMemory(uByteArray.size.toLong())
        val byteArray = uByteArray.asByteArray()
        val slice = copy(byteArray, mem)
        return Pair(mem, slice)
    }

    fun move(uByteArray: UByteArray): Pair<Pointer?, Slice> {
        val mem = allocateOwnedMemory(uByteArray.size.toLong() * Byte.SIZE_BYTES.toLong(), uByteAlign, )
        val byteArray = uByteArray.asByteArray()
        val slice = copy(byteArray, mem)
        return Pair(mem, slice)
    }


    fun borrow(shortArray: ShortArray): Pair<Memory?, Slice> {
        val mem = allocateGarbageCollectedMemory(Short.SIZE_BYTES * shortArray.size.toLong())
        val slice = copy(shortArray, mem)
        return Pair(mem, slice)
    }

    fun move(shortArray: ShortArray): Pair<Pointer?, Slice> {
        val mem = allocateOwnedMemory(Short.SIZE_BYTES * shortArray.size.toLong(), Short.SIZE_BYTES.toLong())
        val slice = copy(shortArray, mem)
        return Pair(mem, slice)
    }

    fun copy(arr: ShortArray, ptr: Pointer?) : Slice {
        val slice = Slice()
        slice.data = if (ptr != null) {
            ptr.write(0, arr, 0, arr.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(arr.size.toLong().toULong())
        return slice
    }

    fun borrow(uShortArray: UShortArray): Pair<Memory?, Slice> {
        val mem = allocateGarbageCollectedMemory(Short.SIZE_BYTES * uShortArray.size.toLong())
        val shortArray = uShortArray.asShortArray()
        val slice = copy(shortArray, mem)
        return Pair(mem, slice)
    }

    fun move(uShortArray: UShortArray): Pair<Pointer?, Slice> {
        val mem = allocateOwnedMemory(Short.SIZE_BYTES * uShortArray.size.toLong(), Short.SIZE_BYTES.toLong())
        val shortArray = uShortArray.asShortArray()
        val slice = copy(shortArray, mem)
        return Pair(mem, slice)
    }

    fun copy(arr: IntArray, ptr: Pointer?) : Slice {
        val slice = Slice()
        slice.data = if (ptr != null) {
            ptr.write(0, arr, 0, arr.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(arr.size.toLong().toULong())
        return slice
    }

    fun borrow(intArray: IntArray): Pair<Memory?, Slice> {
        val mem = allocateGarbageCollectedMemory(Int.SIZE_BYTES * intArray.size.toLong())
        val slice = copy(intArray, mem)
        return Pair(mem, slice)
    }

    fun move(intArray: IntArray): Pair<Pointer?, Slice> {
        val mem = allocateOwnedMemory(Int.SIZE_BYTES * intArray.size.toLong(), Int.SIZE_BYTES.toLong())
        val slice = copy(intArray, mem)
        return Pair(mem, slice)
    }

    fun borrow(uIntArray: UIntArray): Pair<Memory?, Slice> {
        val mem = allocateGarbageCollectedMemory(Int.SIZE_BYTES * uIntArray.size.toLong())
        val intArray = uIntArray.asIntArray()
        val slice = copy(intArray, mem)
        return Pair(mem, slice)
    }

    fun move(uIntArray: UIntArray): Pair<Pointer?, Slice> {
        val mem = allocateOwnedMemory(Int.SIZE_BYTES * uIntArray.size.toLong(), Int.SIZE_BYTES.toLong())
        val intArray = uIntArray.asIntArray()
        val slice = copy(intArray, mem)
        return Pair(mem, slice)
    }

    fun borrow(longArray: LongArray): Pair<Memory?, Slice> {
        val mem = allocateGarbageCollectedMemory(Long.SIZE_BYTES * longArray.size.toLong())
        val slice = copy(longArray, mem)
        return Pair(mem, slice)
    }

    fun move(longArray: LongArray): Pair<Pointer?, Slice> {
        val mem = allocateOwnedMemory(Long.SIZE_BYTES * longArray.size.toLong(), Long.SIZE_BYTES.toLong())
        val slice = copy(longArray, mem)
        return Pair(mem, slice)
    }

    fun copy(arr: LongArray, ptr: Pointer?) : Slice {
        val slice = Slice()
        slice.data = if (ptr != null) {
            ptr.write(0, arr, 0, arr.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(arr.size.toLong().toULong())
        return slice
    }

    fun borrow(uLongArray: ULongArray): Pair<Memory?, Slice> {
        val mem = allocateGarbageCollectedMemory(Long.SIZE_BYTES * uLongArray.size.toLong())
        val longArray = uLongArray.asLongArray()
        val slice = copy(longArray, mem)
        return Pair(mem, slice)
    }

    fun move(uLongArray: ULongArray): Pair<Pointer?, Slice> {
        val mem = allocateOwnedMemory(Long.SIZE_BYTES * uLongArray.size.toLong(), Long.SIZE_BYTES.toLong())
        val longArray = uLongArray.asLongArray()
        val slice = copy(longArray, mem)
        return Pair(mem, slice)
    }

    fun copy(arr: FloatArray, ptr: Pointer?) : Slice {
        val slice = Slice()
        slice.data = if (ptr != null) {
            ptr.write(0, arr, 0, arr.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(arr.size.toLong().toULong())
        return slice
    }

    fun borrow(floatArray: FloatArray): Pair<Memory?, Slice> {
        val mem = allocateGarbageCollectedMemory(Float.SIZE_BYTES * floatArray.size.toLong())
        val slice = copy(floatArray, mem)
        return Pair(mem, slice)
    }

    fun move(floatArray: FloatArray): Pair<Pointer?, Slice> {
        val mem = allocateOwnedMemory(Float.SIZE_BYTES * floatArray.size.toLong(), Float.SIZE_BYTES.toLong())
        val slice = copy(floatArray, mem)
        return Pair(mem, slice)
    }

    fun copy(arr: DoubleArray, ptr: Pointer?) : Slice {
        val slice = Slice()
        slice.data = if (ptr != null) {
            ptr.write(0, arr, 0, arr.size)
            ptr
        } else {
            Pointer(0)
        }
        slice.len = FFISizet(arr.size.toLong().toULong())
        return slice
    }

    fun borrow(doubleArray: DoubleArray): Pair<Memory?, Slice> {
        val mem = allocateGarbageCollectedMemory(Double.SIZE_BYTES * doubleArray.size.toLong())
        val slice = copy(doubleArray, mem)
        return Pair(mem, slice)
    }

    fun move(doubleArray: DoubleArray): Pair<Pointer?, Slice> {
        val mem = allocateOwnedMemory(Double.SIZE_BYTES * doubleArray.size.toLong(), Double.SIZE_BYTES.toLong())
        val slice = copy(doubleArray, mem)
        return Pair(mem, slice)
    }


    fun getByteArray(slice: Slice): ByteArray {
        return slice.data.getByteArray(0, slice.len.toInt())
    }

    @ExperimentalUnsignedTypes
    fun getUByteArray(slice: Slice): UByteArray {
        return slice.data.getByteArray(0, slice.len.toInt()).asUByteArray()
    }

    fun getIntArray(slice: Slice): IntArray {
        return slice.data.getIntArray(0, slice.len.toInt())
    }

    @ExperimentalUnsignedTypes
    fun getUIntArray(slice: Slice): UIntArray {
        return slice.data.getIntArray(0, slice.len.toInt()).asUIntArray()
    }

    fun getShortArray(slice: Slice): ShortArray{
        return slice.data.getShortArray(0, slice.len.toInt())
    }

    @ExperimentalUnsignedTypes
    fun getUShortArray(slice: Slice): UShortArray{
        return slice.data.getShortArray(0, slice.len.toInt()).asUShortArray()
    }

    fun getLongArray (slice: Slice): LongArray {
        return slice.data.getLongArray(0, slice.len.toInt())
    }

    @ExperimentalUnsignedTypes
    fun getULongArray (slice: Slice): ULongArray {
        return slice.data.getLongArray(0, slice.len.toInt()).asULongArray()
    }

    fun getFloatArray (slice: Slice): FloatArray {
        return slice.data.getFloatArray(0, slice.len.toInt())
    }

    fun getDoubleArray (slice: Slice): DoubleArray {
        return slice.data.getDoubleArray(0, slice.len.toInt())
    }

    fun borrowUtf8(str: String): Pair<Memory?, Slice> {
        return borrow(str.toByteArray())
    }

    fun moveUtf8(str: String): Pair<Pointer?, Slice> {
        return move(str.toByteArray())
    }

    fun borrowUtf16(str: String): Pair<Memory?, Slice> {
        return borrow(str.map {it.code.toShort()}.toShortArray())
    }

    fun moveUtf16(str: String): Pair<Pointer?, Slice> {
        return move(str.map {it.code.toShort()}.toShortArray())
    }

    fun getUtf8(slice: Slice): String {
        val byteArray = slice.data.getByteArray(0, slice.len.toInt())

        return byteArray.decodeToString()
    }

    fun getUtf16(slice: Slice): String {
        val shortArray = slice.data.getShortArray(0, slice.len.toInt())
        val charArray = shortArray.map { it.toInt().toChar() }.joinToString(  "")

        return charArray
    }

    fun borrowUtf8s(array: Array<String>): Pair<List<Memory?>, Slice> {
        val sliceSize = Slice.SIZE
        val mem = allocateGarbageCollectedMemory(sliceSize * array.size.toLong())
        val ptr = if (mem != null) {
            mem.share(0)
        } else {
            Pointer(0)
        }
        val mems: List<Memory?> = array.zip(0..array.size.toLong()).map { (str, idx) ->
            val (mem, slice) = borrowUtf8(str)
            ptr.setPointer(idx * sliceSize, slice.data)
            ptr.setLong(idx * sliceSize + Long.SIZE_BYTES, slice.len.toLong())
            mem
        }
        val slice = Slice()
        slice.data = ptr
        slice.len = FFISizet(array.size.toLong().toULong())
        return Pair(mems + mem, slice)
    }

    fun borrowUtf16s(array: Array<String>): Pair<List<Memory?>, Slice> {
        val sliceSize = Slice.SIZE
        val mem = allocateGarbageCollectedMemory(sliceSize * array.size.toLong())
        val ptr = if (mem != null) {
            mem.share(0)
        } else {
            Pointer(0)
        }
        val mems: List<Memory?> = array.zip(0..array.size.toLong()).map { (str, idx) ->
            val (mem, slice) = borrowUtf16(str)
            ptr.setPointer(idx * sliceSize, slice.data)
            ptr.setLong(idx * sliceSize + Long.SIZE_BYTES, slice.len.toLong())
            mem
        }
        val slice = Slice()
        slice.data = ptr
        slice.len = FFISizet(array.size.toLong().toULong())
        return Pair(mems + mem, slice)
    }

    fun getUtf16s(slice: Slice): List<String> {
        return (0..slice.len.toInt()).map { idx ->
            val thisSlice = Slice()
            val thisPtr = Pointer(slice.data.getLong(idx * Slice.SIZE))
            val thisLen = slice.data.getLong(idx * Slice.SIZE + Long.SIZE_BYTES)
            thisSlice.data = thisPtr
            thisSlice.len = FFISizet(thisLen.toULong())
            getUtf16(thisSlice)
        }
    }

    fun getUtf8s(slice: Slice): List<String> {
        return (0..slice.len.toInt()).map { idx ->
            val thisSlice = Slice()
            val thisPtr = Pointer(slice.data.getLong(idx * Slice.SIZE))
            val thisLen = slice.data.getLong(idx * Slice.SIZE + Long.SIZE_BYTES)
            thisSlice.data = thisPtr
            thisSlice.len = FFISizet(thisLen.toULong())
            getUtf8(thisSlice)
        }
    }
}

class FFISizet(val value: ULong = 0u): com.sun.jna.IntegerType(Native.SIZE_T_SIZE, value.toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toChar(): Char = this.toLong().toInt().toChar()
    override fun toShort(): Short = this.toLong().toShort()
    fun toULong(): ULong = this.toLong().toULong()
    constructor(): this(0u)
}

class FFIIsizet(val value: Long = 0): com.sun.jna.IntegerType(Native.SIZE_T_SIZE, value, true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toChar(): Char = this.toLong().toInt().toChar()
    override fun toShort(): Short = this.toLong().toShort()
}

class FFIUint8(val value: UByte = 0u): com.sun.jna.IntegerType(1, value.toByte().toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toChar(): Char = this.toLong().toInt().toChar()
    override fun toShort(): Short = this.toLong().toShort()
    fun toUByte(): UByte = this.toByte().toUByte()
    constructor(): this(0u)
}

class FFIUint16(val value: UShort = 0u): com.sun.jna.IntegerType(2, value.toShort().toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toChar(): Char = this.toLong().toInt().toChar()
    override fun toShort(): Short = this.toLong().toShort()
    fun toUShort(): UShort = this.toShort().toUShort()
    constructor(): this(0u)
}

class FFIUint32(val value: UInt = 0u): com.sun.jna.IntegerType(4, value.toInt().toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toChar(): Char = this.toLong().toInt().toChar()
    override fun toShort(): Short = this.toLong().toShort()
    fun toUInt(): UInt = this.toInt().toUInt()
    constructor(): this(0u)
}

class FFIUint64(val value: ULong = 0u): com.sun.jna.IntegerType(8, value.toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toChar(): Char = this.toLong().toInt().toChar()
    override fun toShort(): Short = this.toLong().toShort()
    fun toULong(): ULong = this.toLong().toULong()
    constructor(): this(0u)
}

class Slice: Structure(), Structure.ByValue {

    @JvmField var data: Pointer = Pointer(0)// Pointer to const char
    @JvmField var len: FFISizet = FFISizet() // FFISizet of 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("data", "len")
    }

    companion object {
        var SIZE: Long = Native.getNativeSize(Slice::class.java).toLong()
    }
}

internal fun <T> T.ok(): Result<T> {
    return Result.success(this)
}

internal fun <T> Throwable.err(): Result<T> {
    return Result.failure(this)
}

class UByteError internal constructor(internal val value: UByte): Exception("Rust error result for UByte") {
    override fun toString(): String {
        return "UByte error with value " + value
    }

    fun getValue(): UByte = value
}

class ByteError internal constructor(internal val value: Byte): Exception("Rust error result for Byte") {
    override fun toString(): String {
        return "Byte error with value " + value
    }

    fun getValue(): Byte = value
}

class UShortError internal constructor(internal val value: UShort): Exception("Rust error result for UShort") {
    override fun toString(): String {
        return "UShort error with value " + value
    }

    fun getValue(): UShort = value
}

class ShortError internal constructor(internal val value: Short): Exception("Rust error result for Short") {
    override fun toString(): String {
        return "Short error with value " + value
    }

    fun getValue(): Short = value
}

class UIntError internal constructor(internal val value: UInt): Exception("Rust error result for UInt") {
    override fun toString(): String {
        return "UInt error with value " + value
    }

    fun getValue(): UInt = value
}

class IntError internal constructor(internal val value: Int): Exception("Rust error result for Int") {
    override fun toString(): String {
        return "Int error with value " + value
    }

    fun getValue(): Int = value
}

class ULongError internal constructor(internal val value: ULong): Exception("Rust error result for ULong") {
    override fun toString(): String {
        return "ULong error with value " + value
    }

    fun getValue(): ULong = value
}

class LongError internal constructor(internal val value: Long): Exception("Rust error result for Long") {
    override fun toString(): String {
        return "Long error with value " + value
    }

    fun getValue(): Long = value
}

class FloatError internal constructor(internal val value: Float): Exception("Rust error result for Float") {
    override fun toString(): String {
        return "Float error with value " + value
    }

    fun getValue(): Float = value
}

class DoubleError internal constructor(internal val value: Double): Exception("Rust error result for Double") {
    override fun toString(): String {
        return "Double error with value " + value
    }

    fun getValue(): Double = value
}

class CharError internal constructor(internal val value: Char): Exception("Rust error result for Char") {
    override fun toString(): String {
        return "Char error with value " + value
    }

    fun getValue(): Char = value
}

class BooleanError internal constructor(internal val value: Boolean): Exception("Rust error result for Boolean") {
    override fun toString(): String {
        return "Boolean error with value " + value
    }

    fun getValue(): Boolean = value
}

class UnitError internal constructor(): Exception("Rust error result for Unit") {
    override fun toString(): String {
        return "Unit error"
    }
}

internal class ResultDateTimeNativeIntUnion: Union() {
    @JvmField
    internal var ok: DateTimeNative = DateTimeNative()
    @JvmField
    internal var err: Int = Rfc9557ParseError.default().toNative()
}

class ResultDateTimeNativeInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultDateTimeNativeIntUnion = ResultDateTimeNativeIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultIsoDateTimeNativeIntUnion: Union() {
    @JvmField
    internal var ok: IsoDateTimeNative = IsoDateTimeNative()
    @JvmField
    internal var err: Int = Rfc9557ParseError.default().toNative()
}

class ResultIsoDateTimeNativeInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultIsoDateTimeNativeIntUnion = ResultIsoDateTimeNativeIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultPointerDecimalLimitErrorNativeUnion: Union() {
    @JvmField
    internal var ok: Pointer = Pointer(0)
}

class ResultPointerDecimalLimitErrorNative: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultPointerDecimalLimitErrorNativeUnion = ResultPointerDecimalLimitErrorNativeUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultPointerIntUnion: Union() {
    @JvmField
    internal var ok: Pointer = Pointer(0)
    @JvmField
    internal var err: Int = CalendarError.default().toNative()
}

class ResultPointerInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultPointerIntUnion = ResultPointerIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultPointerIntUnion: Union() {
    @JvmField
    internal var ok: Pointer = Pointer(0)
    @JvmField
    internal var err: Int = DataError.default().toNative()
}

class ResultPointerInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultPointerIntUnion = ResultPointerIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultPointerIntUnion: Union() {
    @JvmField
    internal var ok: Pointer = Pointer(0)
    @JvmField
    internal var err: Int = DecimalParseError.default().toNative()
}

class ResultPointerInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultPointerIntUnion = ResultPointerIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultPointerIntUnion: Union() {
    @JvmField
    internal var ok: Pointer = Pointer(0)
    @JvmField
    internal var err: Int = LocaleParseError.default().toNative()
}

class ResultPointerInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultPointerIntUnion = ResultPointerIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultPointerIntUnion: Union() {
    @JvmField
    internal var ok: Pointer = Pointer(0)
    @JvmField
    internal var err: Int = Rfc9557ParseError.default().toNative()
}

class ResultPointerInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultPointerIntUnion = ResultPointerIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultPointerTimeZoneInvalidOffsetErrorNativeUnion: Union() {
    @JvmField
    internal var ok: Pointer = Pointer(0)
}

class ResultPointerTimeZoneInvalidOffsetErrorNative: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultPointerTimeZoneInvalidOffsetErrorNativeUnion = ResultPointerTimeZoneInvalidOffsetErrorNativeUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultUnitIntUnion: Union() {
    @JvmField
    internal var err: Int = DataError.default().toNative()
}

class ResultUnitInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultUnitIntUnion = ResultUnitIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultUnitIntUnion: Union() {
    @JvmField
    internal var err: Int = LocaleParseError.default().toNative()
}

class ResultUnitInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultUnitIntUnion = ResultUnitIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultUnitUnitUnion: Union() {
}

class ResultUnitUnit: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultUnitUnitUnion = ResultUnitUnitUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultZonedDateTimeNativeIntUnion: Union() {
    @JvmField
    internal var ok: ZonedDateTimeNative = ZonedDateTimeNative()
    @JvmField
    internal var err: Int = Rfc9557ParseError.default().toNative()
}

class ResultZonedDateTimeNativeInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultZonedDateTimeNativeIntUnion = ResultZonedDateTimeNativeIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultZonedIsoDateTimeNativeIntUnion: Union() {
    @JvmField
    internal var ok: ZonedIsoDateTimeNative = ZonedIsoDateTimeNative()
    @JvmField
    internal var err: Int = Rfc9557ParseError.default().toNative()
}

class ResultZonedIsoDateTimeNativeInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultZonedIsoDateTimeNativeIntUnion = ResultZonedIsoDateTimeNativeIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}


internal class OptionFFIUint16: Structure(), Structure.ByValue  {
    @JvmField
    internal var value: FFIUint16 = FFIUint16()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): FFIUint16? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }
}
internal class OptionInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var value: Int = Int()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): Int? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }
}
internal class OptionIsoDateTimeNative: Structure(), Structure.ByValue  {
    @JvmField
    internal var value: IsoDateTimeNative = IsoDateTimeNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): IsoDateTimeNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }
}
internal class OptionSlice: Structure(), Structure.ByValue  {
    @JvmField
    internal var value: Slice = Slice()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): Slice? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }
}
internal class OptionTimeZoneAndCanonicalAndNormalizedNative: Structure(), Structure.ByValue  {
    @JvmField
    internal var value: TimeZoneAndCanonicalAndNormalizedNative = TimeZoneAndCanonicalAndNormalizedNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): TimeZoneAndCanonicalAndNormalizedNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }
}
internal class OptionTimeZoneAndCanonicalNative: Structure(), Structure.ByValue  {
    @JvmField
    internal var value: TimeZoneAndCanonicalNative = TimeZoneAndCanonicalNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): TimeZoneAndCanonicalNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }
}
internal class OptionUnit: Structure(), Structure.ByValue  {@JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): Unit? {
        if (isOk == 1.toByte()) {
            return Unit
        } else {
            return null
        }
    }
}
internal class OptionVariantOffsetsNative: Structure(), Structure.ByValue  {
    @JvmField
    internal var value: VariantOffsetsNative = VariantOffsetsNative()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): VariantOffsetsNative? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }
}
