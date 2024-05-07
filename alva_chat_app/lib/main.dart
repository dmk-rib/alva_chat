import 'package:flutter/material.dart';
import 'package:alva_chat_app/src/rust/frb_generated.dart';

import 'src/ui/chat_page.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
        colorScheme: const ColorScheme.light(
          background: Colors.white,
          primary: Colors.blue,
        ),
      ),
      home: Scaffold(
        body: ChatPage(),
      ),
    );
  }
}
