use thiserror::Error;


#[derive(Error, Debug)]
pub enum V39Error
{
    #[error("Reinitialization of already initialized item {0} is invalid")]
    Reinit(String),

    #[error("{0}")]
    NoSuitableDevie(String),

    #[error("{0}")]
    VulkanError(#[from] vulkanalia::vk::ErrorCode),
}
