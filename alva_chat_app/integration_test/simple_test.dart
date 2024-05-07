import 'package:flutter_test/flutter_test.dart';
import 'package:alva_chat_app/main.dart';
import 'package:alva_chat_app/src/ignore_me/mandelbrot_related.dart';
import 'package:alva_chat_app/src/rust/frb_generated.dart';
import 'package:integration_test/integration_test.dart';

Future<void> main() async {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  setUpAll(() async => await RustLib.init());

  testWidgets('can see page', (WidgetTester tester) async {
    await tester.pumpWidget(const MyApp());
    await tester.pumpAndSettle();
    expect(find.byType(MandelbrotPageUI), findsOneWidget);
  });
}
