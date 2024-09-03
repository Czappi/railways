import 'package:railways/src/event.dart';
import 'package:railways/src/railway.dart';

class TestRail extends Railway<int> {
  TestRail() : super(0);
}

class Test extends AsyncEvent<int, void, String> {
  @override
  Future<EventResult<void, String>> on(int state, Emitter<int> emit) async {
    return const Ok(());
  }
}

void test(List<String> args) {
  final rail = TestRail();
  final res = Test().send(rail.ref);

  assert(res is Future<EventResult<void, String>>);
}
