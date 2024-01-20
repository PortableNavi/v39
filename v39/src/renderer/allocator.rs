use once_cell::sync::OnceCell;
use vulkanalia::vk::AllocationCallbacks;


static ALLOCATOR: OnceCell<Container> = OnceCell::new();


#[derive(Default)]
struct Container
{
    cb: Option<AllocationCallbacks>,
}


unsafe impl Send for Container {}
unsafe impl Sync for Container {}


pub fn init_allocator()
{
   let _ = ALLOCATOR.set(Container::default());
}


#[inline]
pub fn alloc() -> Option<&'static vulkanalia::vk::AllocationCallbacks>
{
    if let Some(ref allocator) = ALLOCATOR.get().unwrap().cb
    {
        Some(allocator)
    }

    else
    {
        None
    }
}
