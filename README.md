# Railways

DAG-based parallel execution engine

## Layers of abstraction

### Expression

Used as a DSL when interacting with the crate in code.

Can be converted to `LogicalPlan` and `IR`

### LogicalPlan

Used as a dynamic node graph.

Can be converted to `IR` and embedded into an `Expr` tree

### Intermediate Representation (IR)

DAG-based representation of the Flow. The optimization happens here.

Optimized into a `PhysicalPlan`.

Optimization going to include:

- Thread-locality: use same thread for the mainline of the flow
- Early exit detection: Add termination points automatically to unhandled enum return type variants

### PhysicalPlan

Representation which gets passed down to the `Orchestrator` to execute.
Only contains information needed for the execution.

## Resources used

[Zhao, S., Dai, X., Bate, I., Burns, A. and Chang, W. (2020). DAG Scheduling and Analysis on Multiprocessor Systems: Exploitation of Parallelism and Dependency. [online] pp.128–140. doi:https://doi.org/10.1109/rtss49844.2020.00022.](https://ieeexplore.ieee.org/document/9355569)

[Kaur, G. (2016). A DAG based Task Scheduling Algorithms for Multiprocessor System - A Survey. International Journal of Grid and Distributed Computing, 9(9), pp.103–114. doi:https://doi.org/10.14257/ijgdc.2016.9.9.10.](https://article.nadiapub.com/IJGDC/vol9_no9/10.pdf)
