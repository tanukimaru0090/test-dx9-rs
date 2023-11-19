
use std::{ptr, time::Instant};

use core::ops::ControlFlow;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::{BOOL, HWND};
use windows::Win32::Graphics::Direct3D9::{
    Direct3DCreate9, IDirect3D9, IDirect3DDevice9, D3DADAPTER_DEFAULT,
    D3DCREATE_SOFTWARE_VERTEXPROCESSING, D3DDEVTYPE_HAL, D3DFMT_R5G6B5, D3DMULTISAMPLE_NONE,
    D3DPRESENT_INTERVAL_DEFAULT, D3DPRESENT_PARAMETERS, D3DPRESENT_RATE_DEFAULT,
    D3DSWAPEFFECT_DISCARD, D3D_SDK_VERSION,
};

use d3d9::*;
use windows::Win32::{
    System::SystemServices::D3DCLEAR_TARGET, System::SystemServices::D3DCOLOR_RGB,
};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;
unsafe fn setup_dx_context(hwnd: HWND) -> (IDirect3D9, IDirect3DDevice9) {
    let d9_option = Direct3DCreate9(D3D_SDK_VERSION);

    match d9_option {
        Some(d9) => {
            let mut present_params = D3DPRESENT_PARAMETERS {
                BackBufferCount: 1,
                MultiSampleType: D3DMULTISAMPLE_NONE,
                MultiSampleQuality: 0,
                SwapEffect: D3DSWAPEFFECT_DISCARD,
                hDeviceWindow: hwnd,
                Flags: 0,
                FullScreen_RefreshRateInHz: D3DPRESENT_RATE_DEFAULT,
                PresentationInterval: D3DPRESENT_INTERVAL_DEFAULT as u32,
                BackBufferFormat: D3DFMT_R5G6B5,
                EnableAutoDepthStencil: BOOL(0),
                Windowed: BOOL(1),
                BackBufferWidth: WINDOW_WIDTH as _,
                BackBufferHeight: WINDOW_HEIGHT as _,
                ..core::mem::zeroed()
            };
            let mut device: Option<IDirect3DDevice9> = None;
            match d9.CreateDevice(
                D3DADAPTER_DEFAULT,
                D3DDEVTYPE_HAL,
                hwnd,
                D3DCREATE_SOFTWARE_VERTEXPROCESSING as u32,
                &mut present_params,
                &mut device,
            ) {
                Ok(_) => (d9, device.unwrap()),
                _ => panic!("CreateDevice failed"),
            }
        }
        None => panic!("Direct3DCreate9 failed"),
    }
}
fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("test")
        .with_resizable(false)
        .with_inner_size(LogicalSize {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
        })
        .build(&event_loop)
        .unwrap();
    let hwnd = if let Ok(RawWindowHandle::Win32(handle)) = window.raw_window_handle() {
        HWND(isize::from(handle.hwnd))
    } else {
        unreachable!()
    };
    let (_d9, device) = unsafe { setup_dx_context(hwnd) };
    event_loop.run(move |event, control_flow| {
        //let control_flow:&mut winit::event_loop::ControlFlow = control_flow;
        control_flow.set_control_flow(winit::event_loop::ControlFlow::Poll);
        match event {
            Event::NewEvents(_) => {}
            Event::UserEvent(_) => {
                window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => unsafe {
                device.Clear(
                    0,
                    std::ptr::null_mut(),
                    D3DCLEAR_TARGET as u32,
                    0x00000,
                    1.0,
                    0,
                );
                device.BeginScene();
                device.EndScene();
                device.Present(
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    None,
                    std::ptr::null_mut(),
                );
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.exit(),
            _ => {}
        }
    });

    println!("Hello, world!");
}



