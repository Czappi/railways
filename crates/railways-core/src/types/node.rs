trait Node {
    type Properties;
    type State;
    type Input: Input;
    type Output: Output;
    type Error: std::error::Error;

    async fn execute(
        &self,
        state: &mut Self::State,
        input: Self::Input,
    ) -> Result<Self::Output, Self::Error>;

    async fn on_exit(&self, state: &Self::State);

    fn name(&self) -> String;
}

mod test {
    use crate::types::io::{OptionalTarget, Source, Target};

    use super::*;
    use std::convert::Infallible;

    #[derive(Debug)]
    struct TestNode {
        name: String,
    }

    struct TestNodeInput {
        value: Target<String>,
        optional_value: OptionalTarget<String>,
    }

    struct TestNodeOutput {
        value: Source<String>,
    }

    impl Node for TestNode {
        type Properties = ();
        type State = ();
        type Input = TestNodeInput;
        type Output = ();
        type Error = Infallible;

        async fn execute(
            &self,
            _state: &mut Self::State,
            input: Self::Input,
        ) -> Result<Self::Output, Self::Error> {
            Ok(())
        }

        async fn on_exit(&self, _state: &Self::State) {}

        fn name(&self) -> String {
            self.name.clone()
        }
    }
}
