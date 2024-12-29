use ash::Entry;
use ash::{self, vk};
use crate::platform::linux::vulkan_renderer::ash::Device;
use ash::khr::{surface, swapchain, xlib_surface};
use std::ffi::c_char;
use crate::platform::linux::vulkan_renderer::ash::Instance;
use x11_dl::xlib::Window as XWindow;
use x11_dl::xlib::{self, Display, Xlib};
use std::any::Any;
use ash::ext::debug_utils;

#[derive(Clone)]
pub struct VulkanRenderer {

    instance: ash::Instance,
    surface: vk::SurfaceKHR,
    physical_device: vk::PhysicalDevice,
    swapchain: vk::SwapchainKHR,
    entry: ash::Entry,
    device: ash::Device, // logical device
    surface_loader: surface::Instance,
    swapchain_loader: swapchain::Device,
}

impl VulkanRenderer {
    pub fn new(
    window: XWindow, 
    display: *mut Display,
    width: u32, 
    height: u32
) -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
   let entry = Entry::load()?;
        let app_name = c"storm";
        
        // Get required extensions
        let mut extension_names = vec![
            surface::NAME.as_ptr(),
            xlib_surface::NAME.as_ptr(),
            debug_utils::NAME.as_ptr(),
        ];

            let layer_names = [c"VK_LAYER_KHRONOS_validation"];
            let layers_names_raw: Vec<*const c_char> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

            let xlib_surface_extension = xlib_surface::NAME;
            extension_names.push(debug_utils::NAME.as_ptr());
            extension_names.push(xlib_surface_extension.as_ptr());

            let appinfo = vk::ApplicationInfo::default()
                .application_name(app_name)
                .application_version(0)
                .engine_name(app_name)
                .engine_version(0)
                .api_version(vk::make_api_version(0, 1, 0, 0));

            let create_flags = if cfg!(any(target_os = "macos", target_os = "ios")) {
                vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR
            } else {
                vk::InstanceCreateFlags::default()
            };

            let create_info = vk::InstanceCreateInfo::default()
                .application_info(&appinfo)
                .enabled_layer_names(&layers_names_raw)
                .enabled_extension_names(&extension_names)
                .flags(create_flags);

            let instance = 
                entry
                .create_instance(&create_info, None)
                .expect("Instance creation error");

            let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::default()
                .message_severity(
                    vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                        | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                )
                .message_type(
                    vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
                        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
                )
                .pfn_user_callback(Some(vulkan_debug_callback));

            let debug_utils_loader = debug_utils::Instance::new(&entry, &instance);
            let debug_call_back = debug_utils_loader
                .create_debug_utils_messenger(&debug_info, None)
                .unwrap();
            let pdevices = 
                instance
                .enumerate_physical_devices()
                .expect("Physical device error");
            
        let xlib_surface = xlib_surface::Instance::new(&entry, &instance);
        let surface_create_info = vk::XlibSurfaceCreateInfoKHR::default()
            .dpy(display as *mut std::ffi::c_void)
            .window(window);

            
        let surface_loader = surface::Instance::new(&entry, &instance);

        let surface = xlib_surface
            .create_xlib_surface(&surface_create_info, None)
            .expect("Failed to create surface");

            let (pdevice, queue_family_index) = pdevices
                .iter()
                .find_map(|pdevice| {
                    instance
                        .get_physical_device_queue_family_properties(*pdevice)
                        .iter()
                        .enumerate()
                        .find_map(|(index, info)| {
                            let supports_graphic_and_surface =
                                info.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                                    && 
                                        surface_loader
                                        .get_physical_device_surface_support(
                                            *pdevice,
                                            index as u32,
                                            surface,
                                        )
                                        .unwrap();
                            if supports_graphic_and_surface {
                                let physical_device = *pdevice;
                                Some((*pdevice, index))
                            } else {
                                None
                            }
                        })
                })
                .expect("Couldn't find suitable device.");
            let queue_family_index = queue_family_index as u32;
            let device_extension_names_raw = [
                swapchain::NAME.as_ptr(),
                #[cfg(any(target_os = "macos", target_os = "ios"))]
                ash::khr::portability_subset::NAME.as_ptr(),
            ];
            let features = vk::PhysicalDeviceFeatures {
                shader_clip_distance: 1,
                ..Default::default()
            };
            let priorities = [1.0];

            let queue_info = vk::DeviceQueueCreateInfo::default()
                .queue_family_index(queue_family_index)
                .queue_priorities(&priorities);

            let device_create_info = vk::DeviceCreateInfo::default()
                .queue_create_infos(std::slice::from_ref(&queue_info))
                .enabled_extension_names(&device_extension_names_raw)
                .enabled_features(&features);

            let device = 
                instance
                .create_device(pdevice, &device_create_info, None)
                .unwrap();

let swapchain_loader = swapchain::Instance::new(&entry, &instance);

            let present_queue = device.get_device_queue(queue_family_index, 0);

            let surface_format = 
                surface_loader
                .get_physical_device_surface_formats(pdevice, surface)
                .unwrap()[0];

            let surface_capabilities = 
                surface_loader
                .get_physical_device_surface_capabilities(pdevice, surface)
                .unwrap();
            let mut desired_image_count = surface_capabilities.min_image_count + 1;
            if surface_capabilities.max_image_count > 0
                && desired_image_count > surface_capabilities.max_image_count
            {
                desired_image_count = surface_capabilities.max_image_count;
            }
            let surface_resolution = match surface_capabilities.current_extent.width {
                u32::MAX => vk::Extent2D { width, height },
                _ => surface_capabilities.current_extent,
            };
            let pre_transform = if surface_capabilities
                .supported_transforms
                .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
            {
                vk::SurfaceTransformFlagsKHR::IDENTITY
            } else {
                surface_capabilities.current_transform
            };
            let present_modes = 
                surface_loader
                .get_physical_device_surface_present_modes(pdevice, surface)
                .unwrap();
            let present_mode = present_modes
                .iter()
                .cloned()
                .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
                .unwrap_or(vk::PresentModeKHR::FIFO);
            let swapchain_loader = swapchain::Device::new(&instance, &device);

            let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
                .surface(surface)
                .min_image_count(desired_image_count)
                .image_color_space(surface_format.color_space)
                .image_format(surface_format.format)
                .image_extent(surface_resolution)
                .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                .pre_transform(pre_transform)
                .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                .present_mode(present_mode)
                .clipped(true)
                .image_array_layers(1);

            let swapchain =
                swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .unwrap();

Ok(Self {
    instance,
    surface,
    physical_device: pdevice,
    swapchain,
    entry,
    device,
    surface_loader,
    swapchain_loader,
})
        }
    }
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    vk::FALSE
}
