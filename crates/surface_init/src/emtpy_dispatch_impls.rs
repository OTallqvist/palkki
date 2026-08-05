use super::*;
// Required Dispatch impls (empty unless needed)
impl<D: Drawer> Dispatch<wl_compositor::WlCompositor, ()> for State<D> {
    fn event(
        _: &mut Self,
        _: &wl_compositor::WlCompositor,
        _: wl_compositor::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        println!("dispatched");
    }
}

impl<D: Drawer> Dispatch<wl_shm::WlShm, ()> for State<D> {
    fn event(
        _: &mut Self,
        _: &wl_shm::WlShm,
        _: wl_shm::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        println!("dispatched");
    }
}

impl<D: Drawer> Dispatch<zwlr_layer_shell_v1::ZwlrLayerShellV1, ()> for State<D> {
    fn event(
        _: &mut Self,
        _: &zwlr_layer_shell_v1::ZwlrLayerShellV1,
        _: zwlr_layer_shell_v1::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        println!("dispatched");
    }
}

impl<D: Drawer> Dispatch<wl_surface::WlSurface, ()> for State<D> {
    fn event(
        _: &mut Self,
        _: &wl_surface::WlSurface,
        _: wl_surface::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        println!("dispatched");
    }
}

impl<D: Drawer> Dispatch<wl_shm_pool::WlShmPool, ()> for State<D> {
    fn event(
        _: &mut Self,
        _: &wl_shm_pool::WlShmPool,
        _: wl_shm_pool::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        println!("dispatched");
    }
}

impl<D: Drawer> Dispatch<wayland_client::protocol::wl_buffer::WlBuffer, ()> for State<D> {
    fn event(
        _: &mut Self,
        _: &wayland_client::protocol::wl_buffer::WlBuffer,
        _: <wayland_client::protocol::wl_buffer::WlBuffer as wayland_client::Proxy>::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        println!("dispatched");
    }
}

impl<D: Drawer> Dispatch<WlRegistry, GlobalListContents> for State<D> {
    fn event(
        _: &mut Self,
        _: &WlRegistry,
        _: <WlRegistry as wayland_client::Proxy>::Event,
        _: &GlobalListContents,
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        println!("dispatched");
    }
}
