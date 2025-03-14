import 'dart:async';

import 'package:railways/src/event.dart';
import 'package:railways/src/railway.dart';

class ErrorEvent<State, Res> extends Event<State, Res> {
  final Error error;

  ErrorEvent(this.error);

  @override
  FutureOr<EventResult<Res, Error>> on(State state, Emitter<State> emit) {
    return Err(error);
  }
}

class UnexpectedException implements Error {
  final Object exception;
  @override
  final StackTrace stackTrace;

  UnexpectedException(this.exception, this.stackTrace);
}

class InvalidResultState extends Error {}

class EventDropped extends Error {}
