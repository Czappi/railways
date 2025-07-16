pub mod dsl;
//pub mod types;

mod plans {
    use crate::dsl::{LogicalNode, Node};

    /// - can be run multiple lines.
    /// - made of [Node]s
    /// - can act like a node
    struct Pipeline;

    /// parallel primitive
    fn join<'a, T: LogicalNode<'a> + ?Sized>(
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
}
