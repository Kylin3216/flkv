import 'dart:convert';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flkv/flkv.dart';

void main() {
  runApp(MyApp());
}

class MyApp extends StatefulWidget {
  @override
  _MyAppState createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  @override
  void initState() {
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(
          title: const Text('Plugin example app'),
        ),
        body: Center(
          child: Text('Running'),
        ),
        floatingActionButton: FloatingActionButton(
          onPressed: () {
            var db = computeTime(() => KvDB.openInMemory(), des: "open db");
            var key = Uint8List.fromList(utf8.encode("key"));
            computeTime(() => db.put(key, Uint8List.fromList(utf8.encode("value"))), des: "put kv");
            var result = computeTime(() => db.get(key).toList(), des: "get value");
            print(utf8.decode(result));
            var batch = computeTime(() => KvBatch.create(), des: "create batch");
            computeTime(() {
              for (var i = 0; i < 100000; i++) {
                batch.putKv(Uint8List.fromList(utf8.encode("key$i")), Uint8List.fromList(utf8.encode("value$i")));
              }
            }, des: "batch insert 100000");
            computeTime(() => db.putBatch(batch, false), des: "put batch");
            db.close();
          },
          child: Icon(Icons.sync),
        ),
      ),
    );
  }

  T computeTime<T>(T Function() func, {String des = ""}) {
    var start = DateTime.now();
    var data = func();
    print("$des cost:${DateTime.now().difference(start).inMilliseconds}");
    return data;
  }
}
