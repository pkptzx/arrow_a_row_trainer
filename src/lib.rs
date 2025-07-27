use hudhook::tracing::*;
use hudhook::*;
use hudhook::{
    imgui::{Condition, Ui},
    ImguiRenderLoop,
};
use imgui::internal::RawCast;
use imgui::sys::{ImFontAtlas_AddFontFromFileTTF, ImFontAtlas_GetGlyphRangesChineseFull};
use imgui::Key;
use windows::{
    core::h,
    Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
};

use std::mem::size_of;
use std::time::Instant;

pub static mut IS_SHOW_UI: bool = true;
struct RenderLoop {
    start_time: Instant,
    full_hp: bool,
    arrow_attack: f32,
    arrow_speed: f32,
    arrow_distance: f32,
    arrow_count: i32,
    arrow_rate: f32,
    arrow_addition: f32,
    ui_visible: bool,
}

impl RenderLoop {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            full_hp: false,
            arrow_attack: 33.0,
            arrow_speed: 5.0,
            arrow_distance: 10.0,
            arrow_count: 1,
            arrow_rate: 1.1,
            arrow_addition: 0.0,
            ui_visible: true,
        }
    }
}

impl ImguiRenderLoop for RenderLoop {
    fn initialize<'a>(
        &'a mut self,
        _ctx: &mut hudhook::imgui::Context,
        _render_context: &'a mut dyn hudhook::RenderContext,
    ) {
        _ctx.set_ini_filename(None);

        unsafe {
            ImFontAtlas_AddFontFromFileTTF(
                _ctx.fonts().raw_mut(),
                "C:\\windows\\fonts\\simhei.ttf\0".as_ptr().cast(),
                26.0,
                std::ptr::null(),
                ImFontAtlas_GetGlyphRangesChineseFull(_ctx.fonts().raw_mut()),
            )
        };

        _ctx.style_mut().use_light_colors();
    }

    fn render(&mut self, ui: &mut Ui) {
        if ui.is_key_released(Key::Tab) {
            self.ui_visible = !self.ui_visible;
        }
        if self.ui_visible {
            ui.window("箭箭剑修改器 v1.0 ##arrowarow")
            .size([400., 800.], Condition::FirstUseEver)
            .bg_alpha(0.7)
            .build(|| {
                ui.text("[TAB]键显示或隐藏此界面");
                // ui.show_demo_window(&mut self.ui_visible);
                // ui.show_metrics_window(&mut self.ui_visible);
                if ui.button("金币9999") {}
                if ui.checkbox("满血", &mut self.full_hp) {}

                if ui.slider("弓箭攻击", 0.0f32, 9999.0f32, &mut self.arrow_attack) {}
                if ui.slider("弓箭数量", 1, 9999, &mut self.arrow_count) {
                    // [[[["GameAssembly.dll"+01927E78]+B8]+20]+18]+28+20
                    unsafe {
                        let mi = vcheat::internal::get_mod_info("GameAssembly.dll").unwrap();
                        let proc = vcheat::internal::get_proc_handle();
                        let offsets = [0xB8usize, 0x20usize, 0x18usize, 0x28usize+0x20];
                        let base0 = mi.addr.add(0x1927E78);
                        let (addr,_val) = read_mem_with_offsets3::<i32>(proc, base0, offsets.as_slice()).unwrap();
                        // if val < self.arrow_count {
                            let _ = vcheat::write_mem_t(proc, addr as *mut ::core::ffi::c_void, &self.arrow_count, 4);
                        // }
                    }
                }
                if ui.slider("弓箭攻速", 0.0f32, 9999.0f32, &mut self.arrow_speed) {}
                if ui.slider("弓箭距离", 0.0f32, 9999.0f32, &mut self.arrow_distance) {}
                if ui.slider("弓箭频率", 0.0f32, 9999.0f32, &mut self.arrow_rate) {}

                ui.text("弓箭加成: ");

                ui.text(format!("Elapsed: {:?}", self.start_time.elapsed()));
                
                if ui.button("隐藏界面") {
                    self.ui_visible = false;
                }
                ui.same_line_with_spacing(0., 50.);
                if ui.button("卸载") {
                    eject();
                }
            });
        }
        }
        
}

// use hudhook::hooks::dx11::ImguiDx11Hooks;
// hudhook::hudhook!(ImguiDx11Hooks, HelloHud::new());

#[no_mangle]
pub extern "system" fn DllMain(h_module: isize, reason: u32, _: *const u8) -> u32 {
    if reason == 1 {
        println!("==> xinput1_3.dll loaded");

        //unsafe { MessageBoxW(None, h!("xinput1_3.dll loaded"), h!("tips"), MB_OK) };
        trace!("DllMain()");
        std::thread::spawn(move || {
            use hudhook::hooks::dx11::ImguiDx11Hooks;
            if let Err(e) = Hudhook::builder()
                .with::<ImguiDx11Hooks>(RenderLoop::new())
                .with_hmodule(hudhook::windows::Win32::Foundation::HINSTANCE(h_module))
                .build()
                .apply()
            {
                error!("Couldn't apply hooks: {e:?}");
                eject();
            }
        });
    }
    1
}
fn read_mem_with_offsets3<T:Default>(
    proc: isize,
    addr: *mut ::core::ffi::c_void,
    offsets: &[usize],
) -> Option<(*mut ::core::ffi::c_void,T)> {
    let size: usize = (usize::BITS / 8).try_into().unwrap();
    let mut addr = addr;
    let mut buffer = T::default();
    unsafe {
        for offset in offsets {
            let mut buffer = 0usize;
            let _ = vcheat::read_mem_t(proc, addr, &mut buffer, size).unwrap();
            println!("addr: {:?} buf: {:x}", addr, buffer);
            addr = buffer as *mut ::core::ffi::c_void;
            addr = addr.add(*offset);
        }
        
        let _ = vcheat::read_mem_t(proc, addr, &mut buffer, size_of::<T>()).unwrap();
    }

    Some((addr,buffer))
}