import 'dart:async';

import 'package:railways/src/event.dart';

class Task<Res> {
  final Type event;
  final int id;

  final FutureOr<Res> operation;
  final Completer<Res> _completer = Completer();

  Task(this.operation, {Type? event, int? id})
      : event = event ?? operation.runtimeType,
        id = id ?? -1;

  Future<Res> get future => _completer.future;

  void completeAs(Res value) {
    _completer.complete(value);
  }

  void run() {
    _completer.complete(operation);
  }

  Future<void> blockingRun() async {
    run();

    await _completer.future;
  }
}
