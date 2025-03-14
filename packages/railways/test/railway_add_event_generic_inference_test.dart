import 'package:test/test.dart';
import 'package:railways/src/event.dart';
import 'package:railways/src/railway.dart';

class TestRail extends Railway<int> {
  TestRail() : super(0);
}

class TestException implements Exception {
  TestException();
}

class Test extends Event<int, void> {
  @override
  Future<EventResult<void, Error>> on(int state, Emitter<int> emit) async {
    return const Ok(());
  }
}

void main() {
  test("Railway add event result type inference", () {
    final rail = TestRail();
    final res = Test().send(rail.ref);

    expect(res is Future<EventResult<void, Error>>, true);
    expect(res is Future<EventResult<bool, Error>>, false);
  });
}
