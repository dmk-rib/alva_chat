import 'package:alva_chat_app/src/rust/api/conversation.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';

class ChatPage extends StatelessWidget {
  ChatPage({super.key});

  final messages = <Message>[
    const Message(user: true, text: """
    layoutBoundary=up5 NEEDS-PAINT NEEDS-COMPOSITING-BITS-UPDATE
Another exception was thrown: RenderBox was not laid out: RenderStack#b61c9 relayoutBoundary=up4  
NEEDS-PAINT NEEDS-COMPOSITING-BITS-UPDATE
Another exception was thrown: RenderBox was not laid out: RenderPhysicalShape#396ff
relayoutBoundary=up9
Performing hot reload.
    """),
    const Message(user: false, text: """
    layoutBoundary=up5 NEEDS-PAINT NEEDS-COMPOSITING-BITS-UPDATE
Another exception was thrown: RenderBox was not laid out: RenderStack#b61c9 relayoutBoundary=up4  
NEEDS-PAINT NEEDS-COMPOSITING-BITS-UPDATE
Another exception was thrown: RenderBox was not laid out: RenderPhysicalShape#396ff
relayoutBoundary=up9
Performing hot reload.
    """),
  ];

  final TextEditingController controller = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: ListView.builder(
        primary: true,
        itemBuilder: (context, index) {
          final item = messages[index % 2];
          return ListTile(
            enabled: false,
            title: Text(
              item.text,
              textAlign: item.user ? TextAlign.end : TextAlign.start,
              style: item.user
                  ? const TextStyle(color: Colors.lightBlue)
                  : const TextStyle(color: Colors.grey),
            ),
          );
        },
      ),
      bottomNavigationBar: Card.filled(
        margin: const EdgeInsets.all(16.0),
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 8.0),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              //  TextField(),
              Expanded(
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 8.0),
                  child: TextField(
                    controller: controller,
                    decoration: const InputDecoration(
                      border: OutlineInputBorder(),
                      hintText: 'Enter a promt',
                    ),
                    onSubmitted: _submit,
                  ),
                ),
              ),
              IconButton.filled(
                onPressed: () async => await _submit(controller.text),
                icon: const Icon(Icons.message),
              )
            ],
          ),
        ),
      ),
    );
  }

  Future<void> _submit(String value) async {
    debugPrint("Submited value is $value");
  }
}
