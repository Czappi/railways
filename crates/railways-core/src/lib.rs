pub mod dsl;
pub mod physical;
pub mod plan;

mod plans {
    use petgraph::visit::ControlFlow;

    use crate::dsl::{
        execution_flow::ExecutionFlow, list::List, LogicalNode, LogicalSource, Node, Value,
    };

    /// - can be run multiple lines.
    /// - made of [Node]s
    /// - can act like a node
    struct Pipeline;

    /// parallel primitive
    ///
    /// joins all threads of execution into one
    ///
    /// all threads will be waited at this pont to finish execution
    fn join<'a, T: LogicalNode<'a> + ?Sized>(
        // Main "thread", which will be returned
        main: Node<'a, T>,
        // other "threads", which will be joined to the "main thread"
        parallels: &'a [Node<'a, dyn LogicalNode<'a>>],
    ) {
        todo!()
    }

    /// parallel primitive
    ///
    /// collects all the threads of execution into one
    ///
    /// these parallels can be scheduled as seen fit by the scheduler
    fn collect<'a, T: LogicalNode<'a> + ?Sized>(
        // Main "thread", which will be returned
        main: Node<'a, T>,
        // other "threads", which will be joined to the "main thread"
        parallels: &'a [Node<'a, dyn LogicalNode<'a>>],
    ) {
        todo!()
    }

    /// joines the nodes together into a tuple
    /// so it can return boths values
    fn tuple<'a, T0, T1>(tuple: (T0, T1))
    where
        T0: LogicalNode<'a> + Sized,
        T1: LogicalNode<'a> + Sized,
    {
        todo!()
    }

    /// parallel iterator
    ///
    /// iters through a list of values
    fn iter<'a, T>(list: List<'a, T>, iter_flow: ExecutionFlow<'a>) -> Value<'a, T> {
        todo!()
    }

    /// collector for parallel iterators
    fn iter_collect<'a, T>(value: Value<'a, T>, iter_flow: ExecutionFlow<'a>) -> List<'a, T> {
        todo!()
    }
}
