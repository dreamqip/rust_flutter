import 'package:flutter/material.dart';
import 'package:rust_flutter/src/rust/api/simple.dart';
import 'package:rust_flutter/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  example() {
    var c = const Example(number: 100);
    return c.getNumber();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        home: Scaffold(
            appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
            body: Center(child: Text('Result: ${example()}'))));
  }
}
