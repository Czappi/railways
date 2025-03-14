import 'package:railways/src/error.dart';
import 'package:railways/src/event.dart';
import 'package:railways/src/task.dart';

abstract class Executor {
  const Executor();

  void execute(Task<dynamic> task);
}

mixin EventExecutor {
  Executor executor();
}

class SequentialExecutor extends Executor {
  @override
  void execute(Task task) async {
    await task.blockingRun();
  }
}

class ConcurrentExecutor extends Executor {
  @override
  void execute(Task task) {
    task.run();
  }
}

// TODO: Test DroppableEventExecutor
class DroppableEventExecutor extends Executor {
  final Set<Type> runningEvents = {};
  final Executor inner;

  DroppableEventExecutor(this.inner);

  @override
  void execute(Task task) {
    if (!runningEvents.contains(task.event)) {
      runningEvents.add(task.event);
      inner.execute(task);

      task.future.whenComplete(() => runningEvents.remove(task.event));
    } else {
      task.completeAs(Err(EventDropped()));
      // TODO: Event skip notification
    }
  }
}
