use crate::renderer::render_prelude::*;


#[derive(Clone, Debug)]
pub struct ImageData
{
    pub width: u32,
    pub height: u32,
    pub image_type: vk::ImageType,
    pub format: vk::Format,
    pub tiling: vk::ImageTiling,
    pub usage: vk::ImageUsageFlags,
    pub memory_flags: vk::MemoryPropertyFlags,
    pub aspect_flags: vk::ImageAspectFlags,
    
    pub depth: Option<u32>,
    pub mip_levels: Option<u32>,
    pub layer_count: Option<u32>,
    pub sample_count: Option<vk::SampleCountFlags>,
    pub sharing_mode: Option<vk::SharingMode>,
    pub view_type: Option<vk::ImageViewType>,
}


impl ImageData
{
    pub fn new(
        width: u32,
        height: u32,
        image_type: vk::ImageType,
        format: vk::Format,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        memory_flags: vk::MemoryPropertyFlags,
        aspect_flags: vk::ImageAspectFlags,
    ) -> Self
    {
        Self {
            width,
            height,
            image_type,
            format,
            tiling,
            usage,
            memory_flags,
            aspect_flags,

            depth: None,
            mip_levels: None,
            layer_count: None,
            sample_count: None,
            sharing_mode: None,
            view_type: None,
        }
    }
}


fn create_image(vprops: &mut VulkanProps, image_data: &ImageData) -> V39Result<(vk::Image)>
{
    let device = vprops.logical()?;

    let extent = vk::Extent3D {
        width: image_data.width,
        height: image_data.height,
        depth: image_data.depth.unwrap_or(1)
    };

    let create_info = vk::ImageCreateInfo::builder()
        .extent(extent)
        .image_type(image_data.image_type)
        .format(image_data.format)
        .tiling(image_data.tiling)
        .initial_layout(vk::ImageLayout::UNDEFINED)
        .usage(image_data.usage)
        .mip_levels(image_data.mip_levels.unwrap_or(4))
        .array_layers(image_data.layer_count.unwrap_or(1))
        .samples(image_data.sample_count.unwrap_or(vk::SampleCountFlags::_1))
        .sharing_mode(image_data.sharing_mode.unwrap_or(vk::SharingMode::EXCLUSIVE));

    let image = unsafe {device.create_image(&create_info, alloc())}?;

    Ok(image)
}


fn create_image_view(vprops: &mut VulkanProps, image: vk::Image, image_data: &ImageData) -> V39Result<vk::ImageView>
{
    let subresource_range = vk::ImageSubresourceRange::builder()
        .aspect_mask(image_data.aspect_flags)
        .base_mip_level(0)
        .level_count(1)
        .base_array_layer(0)
        .layer_count(image_data.layer_count.unwrap_or(1));

    let create_info = vk::ImageViewCreateInfo::builder()
        .image(image)
        .view_type(image_data.view_type.unwrap_or(vk::ImageViewType::_2D))
        .format(image_data.format)
        .subresource_range(subresource_range);

    Ok(unsafe {vprops.logical().unwrap().create_image_view(&create_info, alloc())}?)
}


fn memory(
    vprops: &mut VulkanProps,
    image: vk::Image,
    image_data: &ImageData,
    offset: Option<u64>
) -> V39Result<vk::DeviceMemory>
{
    let mem_props = vprops.device.as_ref().unwrap().stats.memory;
    let device = vprops.logical()?;
    let mem_req = unsafe {device.get_image_memory_requirements(image)};

    let mem_index = {
        let mut mem_index = None;

        for i in 0..mem_props.memory_type_count
        {
            if mem_req.memory_type_bits & (1 << i) == 1
                && mem_props.memory_types[i as usize].property_flags == image_data.memory_flags
            {
                mem_index = Some(i);
                break;
            }
        }

        match mem_index
        {
            Some(index) => index,
            None => return Err(V39Error::Renderer("No Suitable Memory Index".into())),
        }
    };
 
    let allocate_info = vk::MemoryAllocateInfo::builder()
        .allocation_size(mem_req.size)
        .memory_type_index(mem_index);

    let mem = unsafe {device.allocate_memory(&allocate_info, alloc())}?;
    unsafe {device.bind_image_memory(image, mem, offset.unwrap_or(0))}?;

    Ok(mem)
}


pub struct Image
{
    image_data: ImageData,
    image: vk::Image,
    view: Option<vk::ImageView>,
    memory: vk::DeviceMemory,
}


impl Image
{
    pub fn new(vprops: &mut VulkanProps, image_data: ImageData) -> V39Result<Self>
    {
        let image = create_image(vprops, &image_data)?;
        let memory = memory(vprops, image, &image_data, None)?;
        
        Ok(Self {image, memory, image_data, view: None})
    }

    pub fn with_view(vprops: &mut VulkanProps, image_data: ImageData) -> V39Result<Self>
    {
        let mut me = Self::new(vprops, image_data)?;
        me.create_view(vprops)?;
        Ok(me)
    }

    pub fn create_view(&mut self, vprops: &mut VulkanProps) -> V39Result<()>
    {
        if self.view.is_none()
        {
            self.view = Some(create_image_view(vprops, self.image, &self.image_data)?);
        }

        Ok(())
    }

    pub fn destroy(&mut self, vprops: &mut VulkanProps)
    {
        let device = vprops.logical().unwrap();

        if let Some(view) = self.view
        {
            unsafe {device.destroy_image_view(view, alloc())};
        }

        unsafe
        {
            device.free_memory(self.memory, alloc());
            device.destroy_image(self.image, alloc());
        }
    }
}

