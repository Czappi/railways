pub mod dsl;
//pub mod types;

mod plans {

    struct TestStateManager;

    impl TestStateManager {
        fn a(&self, field: String) -> Result<String, ()> {
            Err(())
        }
    }
}
