import 'dart:async';

import 'package:railways/railways.dart';
import 'package:railways/src/executor.dart';
import 'package:test/test.dart';

class TestState {
  final int value;

  TestState(this.value);
}

class TestSequentialRailway extends Railway<TestState> {
  TestSequentialRailway() : super(TestState(0), executor: SequentialExecutor());
}

class TestConcurrentRailway extends Railway<TestState> {
  TestConcurrentRailway() : super(TestState(0), executor: ConcurrentExecutor());
}

class TestEvent extends Event<TestState, bool> {
  @override
  FutureOr<EventResult<bool, Error>> on(state, Emitter<TestState> emit) async {
    //print("start");
    //print(state.value);
    emit(TestState(state.value + 1));
    //print(state.value);

    return Ok(state.value % 2 == 0);
  }
}

class TestEventWait extends Event<TestState, bool> {
  @override
  FutureOr<EventResult<bool, Error>> on(state, Emitter<TestState> emit) async {
    //print("start waited");
    await Future.delayed(const Duration(seconds: 3));

    //print("waited");
    //print(state.value);
    emit(TestState(state.value + 1));
    //print(state.value);

    return Ok(state.value % 2 == 0);
  }
}

void main() {
  test('sequential executor test', () async {
    final rail = TestSequentialRailway();

    final fut1 = TestEvent().send(rail.ref);
    final fut2 = TestEvent().send(rail.ref);
    final fut3 = TestEvent().send(rail.ref);
    final fut4 = TestEvent().send(rail.ref);

    final futures = await Future.wait([fut1, fut2, fut3, fut4]);

    expect(futures, equals(const [Ok(false), Ok(true), Ok(false), Ok(true)]));
  });

  test('concurrent executor test', () async {
    final rail = TestConcurrentRailway();

    final fut1 = TestEventWait().send(rail.ref); // state: 0
    final fut2 = TestEvent().send(rail.ref); // state: 0

    await Future.delayed(const Duration(seconds: 4));
    final fut3 = TestEventWait().send(
        rail.ref); // state: 1 (fut1 affected it, fut2 is replaced by fut1)

    await Future.delayed(const Duration(seconds: 4));
    final fut4 = TestEvent()
        .send(rail.ref); // state: 2 (fut2 affected it, fut1 not yet ran)

    await Future.delayed(const Duration(seconds: 4));
    final fut5 = TestEvent()
        .send(rail.ref); // state: 3 (fut2 affected it, fut1 not yet ran)

    final futures = await Future.wait([fut1, fut2, fut3, fut4, fut5]);

    expect(futures,
        equals(const [Ok(true), Ok(true), Ok(false), Ok(true), Ok(false)]));
  });
}
