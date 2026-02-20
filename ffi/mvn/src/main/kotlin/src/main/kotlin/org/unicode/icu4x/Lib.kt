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



internal class GCSlice(val memory: Memory?, val slice: Slice) {
    fun close() {
        memory?.close()
    }

    // Stick the contained memory into a list of edges. Returns `this` for convenient chaining.
    fun into(edges: List<MutableList<Any>>): GCSlice {
        memory?.let {
            for(edge in edges) {
                edge.add(memory)
            }
        }
        return this
    }
}

internal class GCSlices(val memory: Memory?, val subMemory: List<Memory?>, val slice: Slice) {
    fun close() {
        memory?.close()
        subMemory.forEach { it?.close() }
    }

    // Stick this object into a list of edges. Returns `this` for convenient chaining.
    fun into(edges: List<MutableList<Any>>): GCSlices {
        for(edge in edges) {
            // Don't bother to split this into submemories, just use this object
            // to tie GC things together
            edge.add(this)
        }
        return this
    }
}


internal class OwnedSlice(val pointer: Pointer?, val slice: Slice) {

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

    fun borrow(boolArray: BooleanArray): GCSlice {
        val mem = allocateGarbageCollectedMemory(boolArray.size.toLong())
        val byteArray = boolArray.map {if (it) 1.toByte() else 0.toByte() }.toByteArray()
        val slice = copy(byteArray, mem)
        return GCSlice(mem, slice)
    }

    fun move(boolArray: BooleanArray): OwnedSlice {
        val mem = allocateOwnedMemory(boolArray.size.toLong() * boolAlign, boolAlign)
        val byteArray = boolArray.map {if (it) 1.toByte() else 0.toByte() }.toByteArray()
        val slice = copy(byteArray, mem)
        return OwnedSlice(mem, slice)
    }

    fun borrow(byteArray: ByteArray): GCSlice {
        val mem = allocateGarbageCollectedMemory(byteArray.size.toLong())
        val slice = copy(byteArray, mem)
        return GCSlice(mem, slice)
    }

    fun move(byteArray: ByteArray): OwnedSlice {
        val mem = allocateOwnedMemory(byteArray.size.toLong() * Byte.SIZE_BYTES.toLong(), uByteAlign, )
        val slice = copy(byteArray, mem)
        return OwnedSlice(mem, slice)
    }


    fun borrow(uByteArray: UByteArray): GCSlice {
        val mem = allocateGarbageCollectedMemory(uByteArray.size.toLong())
        val byteArray = uByteArray.asByteArray()
        val slice = copy(byteArray, mem)
        return GCSlice(mem, slice)
    }

    fun move(uByteArray: UByteArray): OwnedSlice {
        val mem = allocateOwnedMemory(uByteArray.size.toLong() * Byte.SIZE_BYTES.toLong(), uByteAlign, )
        val byteArray = uByteArray.asByteArray()
        val slice = copy(byteArray, mem)
        return OwnedSlice(mem, slice)
    }


    fun borrow(shortArray: ShortArray): GCSlice {
        val mem = allocateGarbageCollectedMemory(Short.SIZE_BYTES * shortArray.size.toLong())
        val slice = copy(shortArray, mem)
        return GCSlice(mem, slice)
    }

    fun move(shortArray: ShortArray): OwnedSlice {
        val mem = allocateOwnedMemory(Short.SIZE_BYTES * shortArray.size.toLong(), Short.SIZE_BYTES.toLong())
        val slice = copy(shortArray, mem)
        return OwnedSlice(mem, slice)
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

    fun borrow(uShortArray: UShortArray): GCSlice {
        val mem = allocateGarbageCollectedMemory(Short.SIZE_BYTES * uShortArray.size.toLong())
        val shortArray = uShortArray.asShortArray()
        val slice = copy(shortArray, mem)
        return GCSlice(mem, slice)
    }

    fun move(uShortArray: UShortArray): OwnedSlice {
        val mem = allocateOwnedMemory(Short.SIZE_BYTES * uShortArray.size.toLong(), Short.SIZE_BYTES.toLong())
        val shortArray = uShortArray.asShortArray()
        val slice = copy(shortArray, mem)
        return OwnedSlice(mem, slice)
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

    fun borrow(intArray: IntArray): GCSlice {
        val mem = allocateGarbageCollectedMemory(Int.SIZE_BYTES * intArray.size.toLong())
        val slice = copy(intArray, mem)
        return GCSlice(mem, slice)
    }

    fun move(intArray: IntArray): OwnedSlice {
        val mem = allocateOwnedMemory(Int.SIZE_BYTES * intArray.size.toLong(), Int.SIZE_BYTES.toLong())
        val slice = copy(intArray, mem)
        return OwnedSlice(mem, slice)
    }

    fun borrow(uIntArray: UIntArray): GCSlice {
        val mem = allocateGarbageCollectedMemory(Int.SIZE_BYTES * uIntArray.size.toLong())
        val intArray = uIntArray.asIntArray()
        val slice = copy(intArray, mem)
        return GCSlice(mem, slice)
    }

    fun move(uIntArray: UIntArray): OwnedSlice {
        val mem = allocateOwnedMemory(Int.SIZE_BYTES * uIntArray.size.toLong(), Int.SIZE_BYTES.toLong())
        val intArray = uIntArray.asIntArray()
        val slice = copy(intArray, mem)
        return OwnedSlice(mem, slice)
    }

    fun borrow(longArray: LongArray): GCSlice {
        val mem = allocateGarbageCollectedMemory(Long.SIZE_BYTES * longArray.size.toLong())
        val slice = copy(longArray, mem)
        return GCSlice(mem, slice)
    }

    fun move(longArray: LongArray): OwnedSlice {
        val mem = allocateOwnedMemory(Long.SIZE_BYTES * longArray.size.toLong(), Long.SIZE_BYTES.toLong())
        val slice = copy(longArray, mem)
        return OwnedSlice(mem, slice)
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

    fun borrow(uLongArray: ULongArray): GCSlice {
        val mem = allocateGarbageCollectedMemory(Long.SIZE_BYTES * uLongArray.size.toLong())
        val longArray = uLongArray.asLongArray()
        val slice = copy(longArray, mem)
        return GCSlice(mem, slice)
    }

    fun move(uLongArray: ULongArray): OwnedSlice {
        val mem = allocateOwnedMemory(Long.SIZE_BYTES * uLongArray.size.toLong(), Long.SIZE_BYTES.toLong())
        val longArray = uLongArray.asLongArray()
        val slice = copy(longArray, mem)
        return OwnedSlice(mem, slice)
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

    fun borrow(floatArray: FloatArray): GCSlice {
        val mem = allocateGarbageCollectedMemory(Float.SIZE_BYTES * floatArray.size.toLong())
        val slice = copy(floatArray, mem)
        return GCSlice(mem, slice)
    }

    fun move(floatArray: FloatArray): OwnedSlice {
        val mem = allocateOwnedMemory(Float.SIZE_BYTES * floatArray.size.toLong(), Float.SIZE_BYTES.toLong())
        val slice = copy(floatArray, mem)
        return OwnedSlice(mem, slice)
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

    fun borrow(doubleArray: DoubleArray): GCSlice {
        val mem = allocateGarbageCollectedMemory(Double.SIZE_BYTES * doubleArray.size.toLong())
        val slice = copy(doubleArray, mem)
        return GCSlice(mem, slice)
    }

    fun move(doubleArray: DoubleArray): OwnedSlice {
        val mem = allocateOwnedMemory(Double.SIZE_BYTES * doubleArray.size.toLong(), Double.SIZE_BYTES.toLong())
        val slice = copy(doubleArray, mem)
        return OwnedSlice(mem, slice)
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

    fun borrowUtf8(str: String): GCSlice {
        return borrow(str.toByteArray())
    }

    fun moveUtf8(str: String): OwnedSlice {
        return move(str.toByteArray())
    }

    fun borrowUtf16(str: String): GCSlice {
        return borrow(str.map {it.code.toShort()}.toShortArray())
    }

    fun moveUtf16(str: String): OwnedSlice {
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

    fun borrowUtf8s(array: Array<String>): GCSlices {
        val sliceSize = Slice.SIZE
        val mem = allocateGarbageCollectedMemory(sliceSize * array.size.toLong())
        val ptr = if (mem != null) {
            mem.share(0)
        } else {
            Pointer(0)
        }
        val mems: List<Memory?> = array.zip(0..array.size.toLong()).map { (str, idx) ->
            val mem = borrowUtf8(str)
            ptr.setPointer(idx * sliceSize, mem.slice.data)
            ptr.setLong(idx * sliceSize + Long.SIZE_BYTES, mem.slice.len.toLong())
            mem.memory
        }
        val slice = Slice()
        slice.data = ptr
        slice.len = FFISizet(array.size.toLong().toULong())
        return GCSlices(mem, mems, slice)
    }

    fun borrowUtf16s(array: Array<String>): GCSlices {
        val sliceSize = Slice.SIZE
        val mem = allocateGarbageCollectedMemory(sliceSize * array.size.toLong())
        val ptr = if (mem != null) {
            mem.share(0)
        } else {
            Pointer(0)
        }
        val mems: List<Memory?> = array.zip(0..array.size.toLong()).map { (str, idx) ->
            val mem = borrowUtf16(str)
            ptr.setPointer(idx * sliceSize, mem.slice.data)
            ptr.setLong(idx * sliceSize + Long.SIZE_BYTES, mem.slice.len.toLong())
            mem.memory
        }
        val slice = Slice()
        slice.data = ptr
        slice.len = FFISizet(array.size.toLong().toULong())
        return GCSlices(mem, mems, slice)
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
    override fun toShort(): Short = this.toLong().toShort()
}

class FFIUint8(val value: UByte = 0u): com.sun.jna.IntegerType(1, value.toByte().toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toShort(): Short = this.toLong().toShort()
    fun toUByte(): UByte = this.toByte().toUByte()
    constructor(): this(0u)
}

class FFIUint16(val value: UShort = 0u): com.sun.jna.IntegerType(2, value.toShort().toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toShort(): Short = this.toLong().toShort()
    fun toUShort(): UShort = this.toShort().toUShort()
    constructor(): this(0u)
}

class FFIUint32(val value: UInt = 0u): com.sun.jna.IntegerType(4, value.toInt().toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
    override fun toShort(): Short = this.toLong().toShort()
    fun toUInt(): UInt = this.toInt().toUInt()
    constructor(): this(0u)
}

class FFIUint64(val value: ULong = 0u): com.sun.jna.IntegerType(8, value.toLong(), true)  {
    override fun toByte(): Byte = this.toLong().toByte()
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

internal class ResultByteIntUnion: Union() {
    @JvmField
    internal var ok: Byte = 0
    @JvmField
    internal var err: Int = 0
}

class ResultByteInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultByteIntUnion = ResultByteIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}
internal class ResultDateTimeNativeIntUnion: Union() {
    @JvmField
    internal var ok: DateTimeNative = DateTimeNative()
    @JvmField
    internal var err: Int = 0
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
    internal var err: Int = 0
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
    internal var err: Int = 0
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
    internal var err: Int = 0
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
    internal var err: Int = 0
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
    internal var err: Int = 0
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
internal class ResultZonedTimeNativeIntUnion: Union() {
    @JvmField
    internal var ok: ZonedTimeNative = ZonedTimeNative()
    @JvmField
    internal var err: Int = 0
}

class ResultZonedTimeNativeInt: Structure(), Structure.ByValue  {
    @JvmField
    internal var union: ResultZonedTimeNativeIntUnion = ResultZonedTimeNativeIntUnion()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("union", "isOk")
    }
}


internal class OptionUnit constructor(): Structure(), Structure.ByValue {@JvmField
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

internal class OptionBoolean constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: Boolean = false

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): Boolean? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: Boolean, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: Boolean): OptionBoolean {
            return OptionBoolean(value, 1)
        }

        internal fun none(): OptionBoolean {
            return OptionBoolean(false, 0)
        }
    }

}

internal class OptionByte constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: Byte = 0

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): Byte? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: Byte, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: Byte): OptionByte {
            return OptionByte(value, 1)
        }

        internal fun none(): OptionByte {
            return OptionByte(0, 0)
        }
    }

}

internal class OptionShort constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: Short = 0

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): Short? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: Short, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: Short): OptionShort {
            return OptionShort(value, 1)
        }

        internal fun none(): OptionShort {
            return OptionShort(0, 0)
        }
    }

}

internal class OptionInt constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: Int = 0

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


    constructor(value: Int, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: Int): OptionInt {
            return OptionInt(value, 1)
        }

        internal fun none(): OptionInt {
            return OptionInt(0, 0)
        }
    }

}

internal class OptionLong constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: Long = 0

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): Long? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: Long, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: Long): OptionLong {
            return OptionLong(value, 1)
        }

        internal fun none(): OptionLong {
            return OptionLong(0, 0)
        }
    }

}

internal class OptionFFIUint8 constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: FFIUint8 = FFIUint8()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): FFIUint8? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: FFIUint8, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: FFIUint8): OptionFFIUint8 {
            return OptionFFIUint8(value, 1)
        }

        internal fun none(): OptionFFIUint8 {
            return OptionFFIUint8(FFIUint8(), 0)
        }
    }

}

internal class OptionFFIUint16 constructor(): Structure(), Structure.ByValue {
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


    constructor(value: FFIUint16, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: FFIUint16): OptionFFIUint16 {
            return OptionFFIUint16(value, 1)
        }

        internal fun none(): OptionFFIUint16 {
            return OptionFFIUint16(FFIUint16(), 0)
        }
    }

}

internal class OptionFFIUint32 constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: FFIUint32 = FFIUint32()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): FFIUint32? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: FFIUint32, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: FFIUint32): OptionFFIUint32 {
            return OptionFFIUint32(value, 1)
        }

        internal fun none(): OptionFFIUint32 {
            return OptionFFIUint32(FFIUint32(), 0)
        }
    }

}

internal class OptionFFIUint64 constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: FFIUint64 = FFIUint64()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): FFIUint64? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: FFIUint64, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: FFIUint64): OptionFFIUint64 {
            return OptionFFIUint64(value, 1)
        }

        internal fun none(): OptionFFIUint64 {
            return OptionFFIUint64(FFIUint64(), 0)
        }
    }

}

internal class OptionFFISizet constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: FFISizet = FFISizet()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): FFISizet? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: FFISizet, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: FFISizet): OptionFFISizet {
            return OptionFFISizet(value, 1)
        }

        internal fun none(): OptionFFISizet {
            return OptionFFISizet(FFISizet(), 0)
        }
    }

}

internal class OptionFFIIsizet constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: FFIIsizet = FFIIsizet()

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): FFIIsizet? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: FFIIsizet, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: FFIIsizet): OptionFFIIsizet {
            return OptionFFIIsizet(value, 1)
        }

        internal fun none(): OptionFFIIsizet {
            return OptionFFIIsizet(FFIIsizet(), 0)
        }
    }

}

internal class OptionFloat constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: Float = 0.0F

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): Float? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: Float, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: Float): OptionFloat {
            return OptionFloat(value, 1)
        }

        internal fun none(): OptionFloat {
            return OptionFloat(0.0F, 0)
        }
    }

}

internal class OptionDouble constructor(): Structure(), Structure.ByValue {
    @JvmField
    internal var value: Double = 0.0

    @JvmField
    internal var isOk: Byte = 0

    // Define the fields of the struct
    override fun getFieldOrder(): List<String> {
        return listOf("value", "isOk")
    }

    internal fun option(): Double? {
        if (isOk == 1.toByte()) {
            return value
        } else {
            return null
        }
    }


    constructor(value: Double, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: Double): OptionDouble {
            return OptionDouble(value, 1)
        }

        internal fun none(): OptionDouble {
            return OptionDouble(0.0, 0)
        }
    }

}

internal class OptionSlice constructor(): Structure(), Structure.ByValue {
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


    constructor(value: Slice, isOk: Byte): this() {
        this.value = value
        this.isOk = isOk
    }

    companion object {
        internal fun some(value: Slice): OptionSlice {
            return OptionSlice(value, 1)
        }

        internal fun none(): OptionSlice {
            return OptionSlice(Slice(), 0)
        }
    }

}

