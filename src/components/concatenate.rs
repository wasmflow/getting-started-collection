pub use crate::components::generated::concatenate::*;

#[async_trait::async_trait]
impl wasmflow_sdk::v1::ephemeral::BatchedComponent for Component {
    async fn job(
        inputs: Self::Inputs,
        outputs: Self::Outputs,
        config: Option<Self::Config>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        outputs
            .output
            .done(format!("{} {}", inputs.left, inputs.right))?;
        Ok(())
    }
}
