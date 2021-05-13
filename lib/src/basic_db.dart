import 'dart:ffi';
import 'dart:io';
import 'dart:typed_data';
import 'package:ffi/ffi.dart';
import 'native.dart';

///
/// Created by iceKylin on 2021/5/13
///
final NativeFlKvDB _native = NativeFlKvDB(_loadLibrary());

DynamicLibrary _loadLibrary() {
  return Platform.isAndroid ? DynamicLibrary.open("libflkv.so") : DynamicLibrary.process();
}

class KvNullPointerError extends Error {
  final String? message;

  KvNullPointerError(this.message);
}

/// KvDB keep a pointer of native FlKv
class KvDB {
  Pointer<FlKv> _flKv;
  String name;

  KvDB._(this.name, this._flKv);

  /// Open a db by [path]
  factory KvDB.open(String path) {
    var pointer = _native.db_open(path.toNativeUtf8().cast(), false);
    if (pointer == nullptr) {
      throw KvNullPointerError("Open DB Error!");
    }
    return KvDB._(path, pointer);
  }

  /// Open a db in memory with [name]
  factory KvDB.openInMemory() {
    var pointer = _native.db_open("inmemory".toNativeUtf8().cast(), true);
    if (pointer == nullptr) {
      throw KvNullPointerError("Open DB Error!");
    }
    return KvDB._("inmemory", pointer);
  }

  /// Put [key] with data [value] into the db
  bool put(Uint8List key, Uint8List value) => _native.db_put(_flKv, _allocateKvBuffer(key), _allocateKvBuffer(value));

  /// Batch put
  /// [sync] if sync will auto call flush
  bool putBatch(KvBatch batch, bool sync) {
    return _native.db_put_batch(_flKv, batch._batch, sync);
  }

  /// Get value by [key]
  Uint8List? get(Uint8List key) {
    var pointer = _native.db_get(_flKv, _allocateKvBuffer(key));
    if (pointer == nullptr) return null;
    return pointer.ref.data.asTypedList(pointer.ref.length);
  }

  /// Delete by [key]
  delete(Uint8List key) => _native.db_delete(_flKv, _allocateKvBuffer(key));

  /// Flush the changes from memory to  disk
  bool flush() => _native.db_flush(_flKv);

  /// Close the db. when this method called,
  /// the pointer [_flKv] will be released and should not use it again
  /// Calling [close] more than once is allowed, and
  /// will have no further effect.
  void close() => _native.db_close(_flKv);
}

/// KvBatch keep a pointer of native FlKvBatch
/// the pointer will be free while call [KvDB.putBatch]
/// do not use it in other place!
class KvBatch {
  Pointer<FlKvBatch> _batch;

  KvBatch._(this._batch);

  /// Create a batch
  factory KvBatch.create() {
    var pointer = _native.db_create_batch();
    if (pointer == nullptr) {
      throw KvNullPointerError("Create Batch Error!");
    }
    return KvBatch._(pointer);
  }

  /// Put kv into the batch
  bool putKv(Uint8List key, Uint8List value) =>
      _native.batch_add_kv(_batch, _allocateKvBuffer(key), _allocateKvBuffer(value));

  /// Clear the batch
  bool clear() => _native.batch_clear(_batch);
}

/// Create a pointer of [KvBuffer]
Pointer<KvBuffer> _allocateKvBuffer(Uint8List data) {
  Pointer<Uint8> p = calloc.allocate(data.length);
  for (var i = 0, len = data.length; i < len; ++i) {
    p[i] = data[i];
  }
  final dd = calloc.allocate<KvBuffer>(data.length + 4);
  dd.ref.data = p;
  dd.ref.length = data.length;
  return dd;
}
