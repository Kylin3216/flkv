import Flutter
import UIKit

public class SwiftFlkvPlugin: NSObject, FlutterPlugin {
  public static func register(with registrar: FlutterPluginRegistrar) {
    
  }
    
  public static func dummyMethodToEnforceBundling() {
    let db = db_open("dummy", true)
    let pointer=UnsafeMutablePointer<KvBuffer>.init(bitPattern: 10)
    db_put(db, pointer, pointer)
    db_get(db, pointer)
    db_delete(db, pointer)
    db_flush(db)
    let batch=db_create_batch()
    batch_clear(batch)
    batch_add_kv(batch, pointer, pointer)
    db_put_batch(db, batch, true)
    db_close(db)
   }
}
