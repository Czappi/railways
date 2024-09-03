import 'package:railways/src/event.dart';
import 'package:railways/src/railway.dart';

class Test extends AsyncEvent<int, void, String> {
  @override
  Future<EventResult<void, String>> on(int state, Emitter<int> emit) async {
    return const Ok(());
  }
}

void test(List<String> args) {
  final res = Test().on(10, (int a) {});

  final asyncres = res.when(
    (ok) => 10,
    (err) => 2,
  );

  assert(asyncres is Future<int>);
}
