import 'package:railways/src/railway.dart';

abstract class Event<State, Res> {
  Res on(State state, Emitter<State> emit);

  Res send(RailwayRef<State> ref) => on(ref.getState(), ref.emit);
}

/// Synchronous Event
abstract class BlockingEvent<State, T, Err>
    extends Event<State, EventResult<T, Err>> {}

/// Asynchronous Event
abstract class AsyncEvent<State, T, Err>
    extends Event<State, Future<EventResult<T, Err>>> {}

extension AsyncEventResult<T, Err> on Future<EventResult<T, Err>> {
  Future<R> when<R>(
    R Function(T value) ok,
    R Function(Err error) err,
  ) async {
    final res = await this;
    return res.when(ok, err);
  }
}

abstract class EventResult<T, E> {
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
}

class InvalidResultState implements Exception {}

class Ok<T, E> extends EventResult<T, E> {
  final T value;

  const Ok(this.value);

  @override
  E? err() => null;

  @override
  T? ok() => value;
}

class Err<T, E> extends EventResult<T, E> {
  final E value;

  const Err(this.value);

  @override
  E? err() => value;

  @override
  T? ok() => null;
}
