use crate::renderer::render_prelude::*;
use crate::renderer::VulkanProps;
use crate::renderer::DEVICE_EXTENSIONS;
use vulkanalia::window as vk_window;
use vulkanalia::vk::KhrSurfaceExtension;
use winit::window::Window;


pub struct Device
{
    pub physical: vk::PhysicalDevice,
    pub logical: vulkanalia::Device,
    pub properties: DeviceProperties,
    pub stats: DeviceStats,
    pub graphics: Option<vk::Queue>,
    pub compute: Option<vk::Queue>,
    pub transfer: Option<vk::Queue>,
    pub present: Option<vk::Queue>,
}


impl Device
{
    pub fn init(instance: &Instance, window: &Window, props: &mut VulkanProps) -> V39Result<()>
    {
        let surface = unsafe {vk_window::create_surface(instance, window, window)}?;
        
        let mut dev_info = None;
        let mut error_msg = String::new();
        let requirements = DeviceProperties::default();

        for device in unsafe {instance.enumerate_physical_devices()}?
        {
            match Self::check_device(instance, device, &requirements, surface)
            {
                Ok(props) => dev_info = Some((device, props)),
                Err(e) => error_msg = e.to_string(), 
            }
        }

        if dev_info.is_none() {return Err(V39Error::NoSuitableDevie(error_msg))}
        let (physical, mut properties) = dev_info.unwrap();

        let stats = properties.stats.take().unwrap();
        info!("Found suitable graphics device: {}", stats.props.device_name);

        let priorities = &[1.0];
        let mut queues = vec![];
        let mut added_indeces: Vec<u32> = vec![];
        
        if let Some(index) = stats.graphics_family_index
        {
            if !added_indeces.contains(&index) {
                queues.push(vk::DeviceQueueCreateInfo::builder()
                    .queue_priorities(priorities)
                    .queue_family_index(index)
                );

                added_indeces.push(index);
            }
        }

        if let Some(index) = stats.compute_family_index
        {
           if !added_indeces.contains(&index) {
                queues.push(vk::DeviceQueueCreateInfo::builder()
                    .queue_priorities(priorities)
                    .queue_family_index(index)
                );

                added_indeces.push(index);
            }
        }

        if let Some(index) = stats.transfer_family_index
        {
            if !added_indeces.contains(&index) {
                queues.push(vk::DeviceQueueCreateInfo::builder()
                    .queue_priorities(priorities)
                    .queue_family_index(index)
                );

                added_indeces.push(index);
            }
        }

        if let Some(index) = stats.present_family_index
        {
            if !added_indeces.contains(&index)
            {
                queues.push(vk::DeviceQueueCreateInfo::builder()
                    .queue_priorities(priorities)
                    .queue_family_index(index)
                )
            }
        }

        let layers = {
            if crate::renderer::VALIDATION_ENABLED 
            {
                vec![crate::renderer::VALIDATION_LAYER.as_ptr()]
            }
            
            else
            {
                vec![]
            }
        };

        let features = vk::PhysicalDeviceFeatures::builder();

        let extensions = DEVICE_EXTENSIONS.iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();

        let create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&queues)
            .enabled_layer_names(&layers)
            .enabled_extension_names(&extensions)
            .enabled_features(&features);

        let logical = unsafe {instance.create_device(physical, &create_info, alloc())}?;

        let graphics = {
            match stats.graphics_family_index
            {
                Some(index) => Some(unsafe {logical.get_device_queue(index, 0)}),
                None => None,
            }
        };

        let compute = {
            match stats.compute_family_index
            {
                Some(index) => Some(unsafe {logical.get_device_queue(index, 0)}),
                None => None,
            }
        };

        let transfer = {
            match stats.transfer_family_index
            {
                Some(index) => Some(unsafe {logical.get_device_queue(index, 0)}),
                None => None,
            }
        };

        let present = {
            match stats.present_family_index
            {
                Some(index) => Some(unsafe {logical.get_device_queue(index, 0)}),
                None => None,
            }
        };

        props.device = Some(Self {physical, properties, stats, logical, graphics, compute, transfer, present});
        props.surface = Some(surface);
        info!("Vulkan Device Created");
        Ok(())
    }

    pub fn destroy(&mut self)
    {
        unsafe {self.logical.destroy_device(alloc())};
        info!("Vulkan Device Destroyed");
    }

    fn check_device(instance: &Instance, device: vk::PhysicalDevice, requirements: &DeviceProperties, surface: vk::SurfaceKHR) -> V39Result<DeviceProperties>
    {
        let props = DeviceProperties::from_device(instance, device, surface);
        let _ = props.meets_requirements(requirements)?;
        Ok(props)
    }
}


#[derive(Clone, Debug)]
pub struct DeviceProperties
{
    pub graphics: bool,
    pub transfer: bool,
    pub compute: bool,
    pub present: bool,
    pub discrete_gpu: bool,
    pub sampler_anisontropy: bool,
    pub extensions: Vec<String>,
    pub stats: Option<DeviceStats>,
}


#[derive(Clone, Debug)]
pub struct DeviceStats
{
    pub props: vk::PhysicalDeviceProperties,
    pub features: vk::PhysicalDeviceFeatures,
    pub memory: vk::PhysicalDeviceMemoryProperties,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub graphics_family_index: Option<u32>,
    pub transfer_family_index: Option<u32>,
    pub compute_family_index: Option<u32>,
    pub present_family_index: Option<u32>,
}


impl DeviceProperties
{
    fn from_device(instance: &Instance, device: vk::PhysicalDevice, surface: vk::SurfaceKHR) -> Self
    {
        let props = unsafe {instance.get_physical_device_properties(device)};
        let features = unsafe {instance.get_physical_device_features(device)};
        let memory = unsafe {instance.get_physical_device_memory_properties(device)};
        let capabilities = unsafe {instance.get_physical_device_surface_capabilities_khr(device, surface)}.expect("Unable to obtain Vulkan Surface Capabilities");
        let formats = unsafe {instance.get_physical_device_surface_formats_khr(device, surface)}.expect("Unable to obtain Vulkan Surface Formats");
        let present_modes = unsafe {instance.get_physical_device_surface_present_modes_khr(device, surface)}.expect("Unable to obtain Vulkan Surface Present Modes");
     
        info!("Querying device properties of {}", props.device_name);

        let queue_family_props = unsafe {instance.get_physical_device_queue_family_properties(device)};
        
        let graphics_family_index = queue_family_props.iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::GRAPHICS))
            .map(|i| i as u32);

        let transfer_family_index = queue_family_props.iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::TRANSFER))
            .map(|i| i as u32);

        let compute_family_index = queue_family_props.iter()
            .position(|p| p.queue_flags.contains(vk::QueueFlags::COMPUTE))
            .map(|i| i as u32);

        let mut present_family_index = None;

        for (idx, prop) in queue_family_props.iter() .enumerate()
        {
            if let Ok(true) = unsafe {instance.get_physical_device_surface_support_khr(device, idx as u32, surface)}
            {
                present_family_index = Some(idx as u32);
                break;
            }
        }

        let graphics = graphics_family_index.is_some();
        let transfer = transfer_family_index.is_some();
        let compute = compute_family_index.is_some();
        let present = present_family_index.is_some();

        let discrete_gpu = props.device_type == vk::PhysicalDeviceType::DISCRETE_GPU;
        let sampler_anisontropy = features.sampler_anisotropy > 0;
        
        let extensions = {
            let extensions = unsafe {instance.enumerate_device_extension_properties(device, None)}
                .unwrap_or_else(|_| vec![]);
            
            extensions
                .iter()
                .map(|p|p.extension_name.to_string())
                .collect()
        };

        let stats = Some(DeviceStats{
            graphics_family_index,
            transfer_family_index,
            compute_family_index,
            present_family_index,
            capabilities,
            present_modes,
            formats,
            props,
            features,
            memory,
        });

        Self{graphics, transfer, compute, present, stats, discrete_gpu, sampler_anisontropy, extensions}
    }

    fn meets_requirements(&self, req: &Self) -> V39Result<()>
    {
        assert(|| self.graphics || !req.graphics, "Graphics Queue")?;
        assert(|| self.compute || !req.compute, "Compute Queue")?;
        assert(|| self.transfer || !req.transfer, "Transfer Queue")?;
        assert(|| self.present || !req.present, "Presentation Queue")?;
        assert(|| self.sampler_anisontropy || !req.sampler_anisontropy, "Aampler Anisontropy")?;
        assert(|| self.discrete_gpu || !req.discrete_gpu, "Discrete GPU")?;
        
        assert(|| {
            for e in &req.extensions
            {
                if !self.extensions.contains(e)
                {
                    return false;
                }
            }

            true
        }, "Required Vulkan Extensions")?;

        Ok(())
    }
}


impl Default for DeviceProperties
{
    fn default() -> Self
    {
        let extension_names = DEVICE_EXTENSIONS
            .iter()
            .map(|e|e.to_string())
            .collect::<Vec<_>>();

        Self {
            graphics: true,
            transfer: true,
            present: true,
            compute: false,
            discrete_gpu: true,
            sampler_anisontropy: true,
            extensions: extension_names,
            stats: None,
        }
    }    
}


fn assert<F: FnOnce()->bool>(f: F, msg: &str) -> V39Result<()>
{
    if !f() {Err(V39Error::NoSuitableDevie(format!("GPU does not meet requirements: {msg}")))}
    else {Ok(())}
}
