use crate::renderer::render_prelude::*;
use crate::renderer::VulkanProps;
use crate::renderer::DEVICE_EXTENSIONS;
use vulkanalia::window as vk_window;
use winit::window::Window;


pub struct Device
{
    pub physical: vk::PhysicalDevice,
    pub properties: DeviceProperties,
    pub stats: DeviceStats,
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
            match Self::check_device(instance, device, &requirements)
            {
                Ok(props) => dev_info = Some((device, props)),
                Err(e) => error_msg = e.to_string(), 
            }
        }

        if dev_info.is_none() {return Err(V39Error::NoSuitableDevie(error_msg))}
        let (physical, mut properties) = dev_info.unwrap();

        let stats = properties.stats.take().unwrap();
        info!("Found suitable graphics device: {}", stats.props.device_name);

        props.device = Some(Self {physical, properties, stats});
        props.surface = Some(surface);
        Ok(())
    }

    pub fn destroy(&mut self)
    {
        
    }

    fn check_device(instance: &Instance, device: vk::PhysicalDevice, requirements: &DeviceProperties) -> V39Result<DeviceProperties>
    {
        let props = DeviceProperties::from_device(instance, device);
        let _ = props.meets_requirements(requirements)?;
        Ok(props)
    }
}


#[derive(Clone, Debug)]
pub struct DeviceProperties
{
    graphics: bool,
    transfer: bool,
    present: bool,
    compute: bool,
    discrete_gpu: bool,
    sampler_anisontropy: bool,
    extensions: Vec<String>,
    stats: Option<DeviceStats>,
}


#[derive(Clone, Debug)]
pub struct DeviceStats
{
    props: vk::PhysicalDeviceProperties,
    features: vk::PhysicalDeviceFeatures,
    memory: vk::PhysicalDeviceMemoryProperties,
    graphics_family_index: Option<u32>,
    present_family_index: Option<u32>,
    transfer_family_index: Option<u32>,
    compute_family_index: Option<u32>,
}


impl DeviceProperties
{
    fn from_device(instance: &Instance, device: vk::PhysicalDevice) -> Self
    {
        let props = unsafe {instance.get_physical_device_properties(device)};
        let features = unsafe {instance.get_physical_device_features(device)};
        let memory = unsafe {instance.get_physical_device_memory_properties(device)};
     
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

        let graphics = graphics_family_index.is_some();
        let transfer = transfer_family_index.is_some();
        let compute = compute_family_index.is_some();
        let present = false;

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
            present_family_index: None,
            graphics_family_index,
            transfer_family_index,
            compute_family_index,
            props,
            features,
            memory
        });

        Self{graphics, transfer, compute, present, stats, discrete_gpu, sampler_anisontropy, extensions}
    }

    fn meets_requirements(&self, req: &Self) -> V39Result<()>
    {
        assert(|| self.graphics || !req.graphics, "Graphics Queue")?;
        assert(|| self.present || !req.present, "Present Queue")?;
        assert(|| self.compute || !req.compute, "Compute Queue")?;
        assert(|| self.transfer || !req.transfer, "Transfer Queue")?;
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
