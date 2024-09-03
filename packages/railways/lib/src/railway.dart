import 'package:bloc/bloc.dart';
import 'package:railways/src/event.dart';

import 'state.dart';

typedef Emitter<State> = void Function(State state);

abstract class Railway<State> {
  State state;

  Railway(State inititalState) : state = inititalState;

  Res add<Res, E extends Event<State, Res>>(E event) {
    return event.on(state, emit);
  }

  void emit(State state) {
    this.state = state;
  }

  RailwayRef<State> get ref => RailwayRef(() => state, emit);
}

class RailwayRef<State> {
  final State Function() getState;
  final Emitter<State> emit;

  const RailwayRef(this.getState, this.emit);
}
