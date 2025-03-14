import 'package:test/test.dart';
import 'package:railways/src/event.dart';
import 'package:railways/src/railway.dart';

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
  test("Event result type inference check", () {
    final res = Test().on(10, (int a) {});

    final asyncres = res.when(
      (ok) => 10,
      (err) => 2,
    );

    print(asyncres.runtimeType);

    expect(asyncres is Future<int>, true);
    assert(asyncres is Future<void>, false);
  });
}
