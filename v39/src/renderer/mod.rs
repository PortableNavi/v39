use vulkanalia::loader::{LibloadingLoader, LIBRARY};
use vulkanalia::{vk::ExtDebugUtilsExtension, vk::KhrSwapchainExtension, vk::KhrSurfaceExtension};
use winit::window::Window;
use std::collections::HashSet;
use once_cell::sync::OnceCell;

mod device;
mod render_prelude;
mod swapchain;

pub(crate) mod allocator;

use render_prelude::*;


pub(crate) const VALIDATION_ENABLED: bool = cfg!(debug_assertions);
pub(crate) const VALIDATION_LAYER: vk::ExtensionName = vk::ExtensionName::from_bytes(b"VK_LAYER_KHRONOS_validation");
pub(crate) const DEVICE_EXTENSIONS: &[vk::ExtensionName] = &[vk::KHR_SWAPCHAIN_EXTENSION.name];
pub(crate) const MAX_FRAMES_IN_FLIGHT: usize = 3;


static INSTANCE: OnceCell<Renderer> = OnceCell::new();


pub(crate) struct Renderer
{
    props: VulkanProps,
    entry: Entry,
    sync: VulkanSync,
    instance: Instance,
}


impl Renderer
{
    pub(crate) fn init(window: &Window) -> V39Result<&'static Self>
    {
        let loader = unsafe {LibloadingLoader::new(LIBRARY)}.expect("Vulkan Loader Failed");
        let entry = unsafe {Entry::new(loader)}.expect("Vulkan Entry Failed");

        let mut debug_info = match VALIDATION_ENABLED
        {
            true => {
                Some(vk::DebugUtilsMessengerCreateInfoEXT::builder()
                    .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::all())
                    .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
                    .user_callback(Some(debug_callback))
                )
            },

            _ => None,
        };
        
        let instance = create_instance(window, &entry, &mut debug_info)?;

        let messenger = match VALIDATION_ENABLED
        {
            true => {
               
                Some(unsafe {instance.create_debug_utils_messenger_ext(&debug_info.unwrap(), None)?})
            },

            _ => None,
        };

        allocator::init_allocator();
        let mut props = VulkanProps::default();

        device::Device::init(&instance, window, &mut props)?;
        swapchain::Swapchain::init(&mut props, window)?;

        let sync = VulkanSync::new(&props)?;

        let renderer = Renderer {
            props,
            entry,
            instance,
            sync,
        };
        
        if INSTANCE.set(renderer).is_err()
        {
            return Err(V39Error::Reinit("Renderer".into()));
        }

        info!("Renderer Initialized");
        Ok(INSTANCE.get().unwrap())
    }

    pub(crate) fn destroy(&mut self)
    {
        self.sync.deytroy(&mut self.props);
        self.props.destroy(&self.instance);
    }
}


#[derive(Default)]
pub(crate) struct VulkanProps
{
    device: Option<device::Device>,
    surface: Option<vk::SurfaceKHR>,
    swapchain: Option<swapchain::Swapchain>,
}

impl VulkanProps
{
    fn destroy(&mut self, instance: &Instance)
    {
        if let Some(ref mut device) = self.device 
        {
            if let Some(ref mut swapchain) = self.swapchain {swapchain.destroy(device)};
            device.destroy();
        }
        
        if let Some(surface) = self.surface
        {
            unsafe {instance.destroy_surface_khr(surface, alloc())}
        }
    }
}


#[derive(Clone, Debug)]
pub(crate) struct VulkanSync
{
    image_available: [vk::Semaphore; MAX_FRAMES_IN_FLIGHT],
    render_finished: [vk::Semaphore; MAX_FRAMES_IN_FLIGHT],
}


impl VulkanSync
{
    pub(crate) fn new(vprops: &VulkanProps) -> V39Result<Self>
    {
        let semaphore_info = vk::SemaphoreCreateInfo::builder();
        let mut image_available = [vk::Semaphore::null(); MAX_FRAMES_IN_FLIGHT];
        let mut render_finished = [vk::Semaphore::null(); MAX_FRAMES_IN_FLIGHT];

        let device = vprops.device.as_ref().unwrap();

        for i in 0..MAX_FRAMES_IN_FLIGHT
        {
            unsafe
            {
              image_available[i] = device.logical.create_semaphore(&semaphore_info, None)?;
              render_finished[i] = device.logical.create_semaphore(&semaphore_info, None)?;
            }
        }

        Ok(Self{
            image_available,
            render_finished,
        })
    }

    pub(crate) fn deytroy(&mut self, vprops: &VulkanProps)
    {
        let device = &vprops.device.as_ref().unwrap();

        unsafe 
        {
            for (s1, s2) in self.render_finished.iter().zip(self.image_available.iter())
            {
                device.logical.destroy_semaphore(*s1, None);
                device.logical.destroy_semaphore(*s2, None);
            }
        }
    }
}


extern "system" fn debug_callback(
    severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    kind: vk::DebugUtilsMessageTypeFlagsEXT,
    data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _: *mut std::ffi::c_void,
) -> vk::Bool32
{
    let data = unsafe {*data};
    let msg = unsafe {std::ffi::CStr::from_ptr(data.message)}.to_string_lossy();

    if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::ERROR {error!("({kind:?}) {msg}")}
    else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::WARNING {warn!("({kind:?}) {msg}")}
    else if severity >= vk::DebugUtilsMessageSeverityFlagsEXT::INFO {info!("({kind:?}) {msg}")}
    else {debug!("({kind:?}) {msg}")}

    vk::FALSE
}


fn create_instance(window: &Window, entry: &Entry, debug_info: &mut Option<vk::DebugUtilsMessengerCreateInfoEXTBuilder>) -> V39Result<Instance>
{
    let mut skip_validation_layer = false;

    let layers = unsafe {entry.enumerate_instance_layer_properties()}?
        .iter()
        .map(|l|l.layer_name)
        .collect::<HashSet<_>>();

    if VALIDATION_ENABLED && !layers.contains(&VALIDATION_LAYER)
    {
        skip_validation_layer = true;
        error!("Validation Layer {VALIDATION_LAYER:?} requested despite not being available. Skipping Layer"); 
    }

    let app_info = vk::ApplicationInfo::builder()
        .application_name(b"v39 App")
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(b"v39")
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0));

    let mut extensions = vk_window::get_required_instance_extensions(window)
        .iter()
        .map(|e|e.as_ptr())
        .collect::<Vec<_>>();

    if VALIDATION_ENABLED && !skip_validation_layer
    {
        extensions.push(vk::EXT_DEBUG_UTILS_EXTENSION.name.as_ptr());
    }

    let active_layers = {
        if VALIDATION_ENABLED && !skip_validation_layer
        {
            vec![VALIDATION_LAYER.as_ptr()]
        }
        else
        {
            vec![]
        }
    };

    let mut create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_layer_names(&active_layers)
        .enabled_extension_names(&extensions);

    if let Some(ref mut debug_info) = debug_info
    {
        create_info = create_info.push_next(debug_info);
    }

    let instance = unsafe {entry.create_instance(&create_info, None)?};

    Ok(instance)
}
