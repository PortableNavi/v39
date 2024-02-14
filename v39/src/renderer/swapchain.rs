use crate::renderer::render_prelude::*;
use crate::renderer::MAX_FRAMES_IN_FLIGHT;
use vk::KhrSwapchainExtension;
use winit::window::Window;


pub struct Swapchain
{
    surface_format: vk::SurfaceFormatKHR,
    images: [vk::Image; MAX_FRAMES_IN_FLIGHT],
    image_views: [vk::ImageView; MAX_FRAMES_IN_FLIGHT],
    swapchain: vk::SwapchainKHR,
    width: u32,
    height: u32,
    current_image: usize,
}


impl Swapchain
{
    pub fn init(vprops: &mut VulkanProps, window: &Window) -> V39Result<()>
    {
        let width = window.inner_size().width;
        let height = window.inner_size().height;
        
        let swapchain = Self::create(vprops, None, width, height)?;
       
        //vprops.swapchain = Some(Swapchain {swapchain, width, height});
        info!("Vulkan Swapchain Created");
        Ok(())
    }

    pub fn recreate(&mut self, vprops: &mut VulkanProps, window: &Window) -> V39Result<()>
    {
        let device = vprops.device.as_mut().unwrap();
        self.destroy(device);

        self.width = window.inner_size().width;
        self.height = window.inner_size().height;

        self.swapchain = Self::create(vprops, Some(self.swapchain), self.width, self.height)?;
        todo!()
    }

    pub fn next_image(&mut self, vprops: &mut VulkanProps, timeout_ns: u64, sync: &VulkanSync, fence: vk::Fence, window: &Window) -> V39Result<usize>
    {
        let device = &vprops.device.as_mut().unwrap().logical;
        let (next_image, suc) =  match unsafe {device.acquire_next_image_khr(self.swapchain, timeout_ns, sync.image_available[self.current_image], fence)}
        {
            Ok(result) => result,
            
            Err(vk::ErrorCode::OUT_OF_DATE_KHR) => { 
                self.recreate(vprops, window)?;
                let next_image = self.next_image(vprops, timeout_ns, sync, fence, window)?;
                (next_image as u32, vk::SuccessCode::SUCCESS)
            },
            
            Err(e) => return Err(e)?, 
        };

        if !(suc == vk::SuccessCode::SUCCESS || suc == vk::SuccessCode::SUBOPTIMAL_KHR)
        {
            return Err(V39Error::Renderer("Failed to aqcuire next image".into()));
        }

        Ok(next_image as usize)
    }

    pub fn present(&mut self, vprops: &mut VulkanProps, sync: &VulkanSync, present_image_index: usize) -> V39Result<()>
    {
        self.current_image = present_image_index;
        todo!()
    }

    pub fn destroy(&mut self, device: &mut crate::renderer::device::Device)
    {
        //TODO: Implement
    }

    fn create(vprops: &mut VulkanProps, old_swapchain: Option<vk::SwapchainKHR>, width: u32, height: u32) -> V39Result<vk::SwapchainKHR>
    {
        todo!()
    }
}

