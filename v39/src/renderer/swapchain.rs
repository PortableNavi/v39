use crate::renderer::render_prelude::*;
use crate::renderer::MAX_FRAMES_IN_FLIGHT;
use crate::renderer::image::{Image, ImageData};
use vk::KhrSwapchainExtension;
use winit::window::Window;


pub struct Swapchain
{
    surface_format: vk::SurfaceFormatKHR,
    images: Vec<vk::Image>,
    image_views: Vec<vk::ImageView>,
    swapchain: vk::SwapchainKHR,
    width: u32,
    height: u32,
    current_image: usize,
    depth_buffer: Image,
}


impl Swapchain
{
    pub fn init(vprops: &mut VulkanProps, window: &Window) -> V39Result<()>
    {
        let width = window.inner_size().width;
        let height = window.inner_size().height;

        let (surface_format, swapchain) = Self::create(vprops, None, width, height)?;
        let (images, image_views) = Self::get_images(vprops, swapchain, surface_format.format)?;
        let depth_buffer = Self::get_depth_buffer(vprops, width, height)?;

        vprops.swapchain = Some(Swapchain {swapchain, width, height, surface_format, images, image_views, depth_buffer, current_image: 0});
        info!("Vulkan Swapchain Created");
        Ok(())
    }

    pub fn recreate(&mut self, vprops: &mut VulkanProps, window: &Window) -> V39Result<()>
    {
        let device = vprops.device.as_mut().unwrap();
        //self.destroy(device);

        self.width = window.inner_size().width;
        self.height = window.inner_size().height;

        (self.surface_format, self.swapchain) = Self::create(vprops, Some(self.swapchain), self.width, self.height)?;
        todo!()
    }

    pub fn next_image(&mut self, vprops: &mut VulkanProps, timeout_ns: u64, sync: &VulkanSync, fence: vk::Fence, window: &Window) -> V39Result<usize>
    {
        let device = vprops.logical().unwrap();
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

    pub fn present(&mut self, vprops: &mut VulkanProps, sync: &VulkanSync, present_image_index: usize, window: &Window) -> V39Result<()>
    {
        let wait_semaphores = &[sync.image_available[present_image_index]];
        let swapchains = &[self.swapchain];
        let indices = &[present_image_index as u32];

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(wait_semaphores)
            .swapchains(swapchains)
            .image_indices(indices);

        let mut device = vprops.device.as_mut().unwrap();
        
        if let Some(queue) = device.present
        {
            let result = unsafe {device.logical.queue_present_khr(queue, &present_info)};

            match result
            {
                Err(vk::ErrorCode::OUT_OF_DATE_KHR) => {
                    self.recreate(vprops, window);
                    self.present(vprops, sync, present_image_index, window)
                },

                Ok(vk::SuccessCode::SUBOPTIMAL_KHR) => {
                    self.recreate(vprops, window);
                    self.present(vprops, sync, present_image_index, window)
                },

                Err(e) => Err(e)?,
                _ => Ok(())
            }?;        
        }

        else
        {
            self.current_image = present_image_index;
        }
            
        Ok(())
    }

    pub fn destroy(&mut self, vprops: &mut VulkanProps)
    {
        self.depth_buffer.destroy(vprops);

        let device = vprops.logical().unwrap();

        for view in self.image_views.iter()
        {
            unsafe {device.destroy_image_view(*view, alloc())};
        }

        unsafe {device.destroy_swapchain_khr(self.swapchain, alloc())};
        info!("Swapchain Destroyed");
    }

    fn create(vprops: &mut VulkanProps, old_swapchain: Option<vk::SwapchainKHR>, width: u32, height: u32) -> V39Result<(vk::SurfaceFormatKHR, vk::SwapchainKHR)>
    {
        let mut device = &mut vprops.device.as_mut().unwrap();

        let preferred_format = vk::SurfaceFormatKHR::builder()
            .format(vk::Format::B8G8R8A8_UNORM)
            .color_space(vk::ColorSpaceKHR::SRGB_NONLINEAR)
            .build();

        let preferred_present_mode = vk::PresentModeKHR::MAILBOX;

        let format = {
            if device.stats.formats.contains(&preferred_format) {preferred_format}
            else {device.stats.formats[0]}
        };
        
        let present_mode = {
            if device.stats.present_modes.contains(&preferred_present_mode) {preferred_present_mode}
            else {vk::PresentModeKHR::FIFO}
        };

        let min_extent = device.stats.capabilities.min_image_extent;
        let max_extent = device.stats.capabilities.min_image_extent;

        let width = width.clamp(min_extent.width, max_extent.width);
        let height = height.clamp(min_extent.height, max_extent.height);
        let extent = vk::Extent2D {width, height};

        let min_image_count = device.stats.capabilities.min_image_count + 1;

        let mut sharing_mode = vk::SharingMode::CONCURRENT;
        let mut queues = vec![];

        if let Some(queue) = device.stats.present_family_index
        {
            queues.push(queue);
        }

        if let Some(queue) = device.stats.graphics_family_index
        {
            if queues.contains(&queue)
            {
                sharing_mode = vk::SharingMode::EXCLUSIVE;
            }

            else
            {
                queues.push(queue);
            }
        }

        let create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(vprops.surface.unwrap())
            .min_image_count(min_image_count)
            .image_format(format.format)
            .image_color_space(format.color_space)
            .image_extent(extent)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_array_layers(1)
            .present_mode(present_mode)
            .image_sharing_mode(sharing_mode)
            .queue_family_indices(&queues)
            .pre_transform(device.stats.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .old_swapchain(old_swapchain.unwrap_or_else(vk::SwapchainKHR::null));
        
        let swapchain = unsafe {device.logical.create_swapchain_khr(&create_info, alloc())}?;

        Ok((format, swapchain))
    }

    fn get_images(vprops: &mut VulkanProps, swapchain: vk::SwapchainKHR, format: vk::Format) 
        -> V39Result<(Vec<vk::Image>, Vec<vk::ImageView>)>
    {
        let device = vprops.device.as_ref().unwrap();
        
        let images = unsafe {device.logical.get_swapchain_images_khr(swapchain)}?;
        let mut views = vec![];
        
        for image in &images
        {
            let subresource_range = vk::ImageSubresourceRange::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1);

            let create_info = vk::ImageViewCreateInfo::builder()
                .image(*image)
                .view_type(vk::ImageViewType::_2D)
                .format(format)
                .subresource_range(subresource_range);

            let view = unsafe {device.logical.create_image_view(&create_info, alloc())}?;
            views.push(view)
        }

        Ok((images, views))
    }
    
    fn get_depth_buffer(vprops: &mut VulkanProps, width: u32, height: u32) -> V39Result<Image>
    {
        
        let format = vprops.device.as_ref().unwrap().stats.depth_format;

        let image_data = ImageData::new(
            width,
            height,
            vk::ImageType::_2D,
            format,
            vk::ImageTiling::OPTIMAL,
            vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
            vk::ImageAspectFlags::DEPTH,
        );

        let image = Image::with_view(vprops, image_data)?;
        Ok(image)
    }
}

