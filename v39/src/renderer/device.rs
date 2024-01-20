use crate::renderer::render_prelude::*;
use crate::renderer::VulkanProps;
use vulkanalia::window as vk_window;
use winit::window::Window;


pub struct Device
{
    pub physical: vk::PhysicalDevice,
    pub properties: DeviceProperties,
}


impl Device
{
    pub fn init(instance: &Instance, window: &Window, props: &mut VulkanProps) -> V39Result<()>
    {
        let surface = unsafe {vk_window::create_surface(instance, window, window)}?;
        
        let mut dev_info = None;
        let requirements = DeviceProperties::default();

        for device in unsafe {instance.enumerate_physical_devices()}?
        {
            if let Ok(props) = Self::check_device(instance, device, &requirements)
            {
                dev_info = Some((device, props));
            }
        }

        if dev_info.is_none() {return Err(V39Error::NoSuitableDevie("No Suitable GPU found.".into()))}
        let (physical, properties) = dev_info.unwrap();

        props.device = Some(Self {physical, properties});
        props.surface = Some(surface);
        Ok(())
    }

    pub fn destroy(&mut self)
    {
        
    }

    fn check_device(instance: &Instance, device: vk::PhysicalDevice, requirements: &DeviceProperties) -> V39Result<DeviceProperties>
    {
        let props = unsafe {instance.get_physical_device_properties(device)};
        let feat = unsafe {instance.get_physical_device_features(device)};
        let mem = unsafe {instance.get_physical_device_memory_properties(device)};

        Ok(DeviceProperties::default())
    }
}


#[derive(Clone, Debug)]
pub struct DeviceProperties
{
    graphics: bool,
    transfer: bool,
    present: bool,
    discrete_gpu: bool,
    extensions: Vec<String>,
}

impl Default for DeviceProperties
{
    fn default() -> Self
    {
        Self {
            graphics: true,
            transfer: true,
            present: true,
            discrete_gpu: false,
            extensions: vec![],
        }
    }
}

