abstract class State {
  /// Check if State have to be emited
  bool shouldEmit(State previous) {
    return true;
  }
}
