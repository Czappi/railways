import 'dart:async';

import 'package:bloc/bloc.dart';
import 'package:railways/src/error.dart';
import 'package:railways/src/event.dart';
import 'package:railways/src/executor.dart';
import 'package:railways/src/task.dart';

import 'package:railways/src/event.dart' as res show Err;

import 'state.dart';

typedef Emitter<State> = void Function(State state);

typedef StreamTransformer = Stream<dynamic> Function(Stream<dynamic> stream);

void concurrentExecutor(Task<dynamic> task) {
  task.run();
}

abstract class Railway<State> {
  late State _state;

  Railway(
    State initialState, {
    StreamTransformer? transformer,
    Executor? executor,
  }) {
    _state = initialState;
    _stateController.add(initialState);

    execution(transformer, executor ?? SequentialExecutor());
  }

  final _stateController = StreamController<State>.broadcast();
  final _eventSink = StreamController<dynamic>.broadcast();

  State get state => _state;

  Stream<State> get stream => _stateController.stream;

  void execution(StreamTransformer? transformer, Executor executor) {
    _eventSink.stream.listen((task) {
      if (task is Task<dynamic>) {
        executor.execute(task);
      }
    });
  }

  void addTask<Res>(Task<Future<EventResult<Res, Error>>> event) {
    _eventSink.add(event);
  }

  void emit(State state) {
    if (state is ShouldEmit && !state.shouldEmit(this.state as ShouldEmit)) {
      return;
    }

    this._stateController.add(state);
    this._state = state;
  }

  RailwayRef<State> get ref => RailwayRef(() => state, _eventSink.sink, emit);
}

class RailwayRef<State> {
  final State Function() _state;
  final StreamSink<dynamic> _sink;
  final Emitter<State> emit;

  const RailwayRef(this._state, this._sink, this.emit);

  State get state => _state();

  Future<EventResult<Res, Error>> add<E extends Event<State, Res>, Res>(
      E event) {
    final task = Task(event.compute(state, emit), event: event.runtimeType);

    _sink.add(task);

    return task.future;
  }
}
