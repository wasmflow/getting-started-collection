pub use crate::components::generated::greet::*;

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,
        _config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let greeting = format!("Hello {}", inputs.input);
        outputs.output.done(greeting)?;
        Ok(())
    }
}
