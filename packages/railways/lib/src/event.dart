import 'dart:async';

import 'package:railways/src/error.dart';
import 'package:railways/src/railway.dart';

abstract class Event<State, Res> {
  /// Function which runs on the [Event].
  ///
  /// Note that the `state` is from the time of the execution. (When the executor running unordered, this can cause **Data race**)
  FutureOr<EventResult<Res, Error>> on(State state, Emitter<State> emit);

  Future<EventResult<Res, Error>> send(RailwayRef<State> ref) async =>
      await on(ref.state, ref.emit);

  FutureOr<EventResult<Res, Error>> compute(State state, Emitter<State> emit) {
    try {
      return on(state, emit);
    } catch (error, stackTrace) {
      return ErrorEvent<State, Res>(UnexpectedException(error, stackTrace))
          .on(state, emit);
    }
  }
}

extension AsyncEventResult<T, E extends Error> on Future<EventResult<T, E>> {
  Future<R> when<R>(
    R Function(T value) ok,
    R Function(E error) err,
  ) async {
    final res = await this;
    return res.when(ok, err);
  }
}

abstract class EventResult<T, E extends Error> {
  const EventResult();

  T? ok();
  E? err();

  bool isOk() {
    return ok() != null;
  }

  bool isErr() {
    return err() != null;
  }

  R when<R>(
    R Function(T value) ok,
    R Function(E error) err,
  ) {
    if (isOk()) {
      return ok(this.ok() as T);
    } else if (isErr()) {
      return err(this.err() as E);
    } else {
      throw InvalidResultState();
    }
  }

  @override
  bool operator ==(Object other) =>
      other is EventResult<T, E> && ok() == other.ok() && err() == other.err();

  @override
  int get hashCode => Object.hash(ok(), err()); // <----- here
}

class Ok<T, E extends Error> extends EventResult<T, E> {
  final T value;

  const Ok(this.value);

  @override
  E? err() => null;

  @override
  T? ok() => value;

  @override
  String toString() {
    return "Ok($value)";
  }
}

class Err<T, E extends Error> extends EventResult<T, E> {
  final E value;

  const Err(this.value);

  @override
  E? err() => value;

  @override
  T? ok() => null;

  @override
  String toString() {
    return "Err($value)";
  }
}
