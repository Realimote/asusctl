//! Demo mode (`--demo`): runs the UI with representative fake data and no
//! dbus/system daemon at all. Useful for UI development, screenshots and
//! trying out the control center on non-ASUS hardware. All controls are
//! interactive and update the demo state only.

use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;

use log::info;
use slint::{Brush, Color, ComponentHandle, Model, SharedString, Timer, TimerMode};

use crate::{
    AttrMinMax, AuraDevType, AuraEffect, AuraPageData, FanPageData, FanType, GPUPageData,
    MainWindow, Profile, SlashPageData, SystemPageData,
};

fn demo_err<E: std::fmt::Display>(e: E) -> crate::error::Error {
    crate::error::Error::Io(std::io::Error::other(e.to_string()))
}

pub fn run() -> crate::error::Result<()> {
    info!("Starting in demo mode (no dbus, fake data)");
    slint::set_xdg_app_id(crate::APP_ID)
        .map_err(demo_err)
        .ok();
    slint::init_translations!(env!("ROGCC_TRANSLATIONS_DIR"));

    let ui = MainWindow::new().map_err(demo_err)?;
    ui.set_device_name("ROG Zephyrus G14 GA403UV (demo)".into());
    ui.set_sidebar_items_available([
        true, true, true, true, true, true, true, true, true, true,
    ].into());

    let weak = ui.as_weak();

    // ─── SystemPageData ────────────────────────────────────────────────
    {
        let data = ui.global::<SystemPageData>();
        data.set_asus_armoury_loaded(true);

        data.set_platform_profile_choices(
            ["Balanced", "Performance", "Quiet"]
                .iter()
                .map(|s| SharedString::from(*s))
                .collect::<Vec<_>>()
                .as_slice()
                .into(),
        );
        data.set_platform_profile_indexes([0i32, 1, 2].into());
        data.set_platform_profile(1);

        data.set_charge_control_enabled(true);
        data.set_charge_control_end_threshold(80.0);
        data.set_battery_health(96);
        data.set_battery_power_consumption(14.2);
        data.set_battery_status("Discharging".into());
        data.set_battery_time_estimate("4:37".into());

        data.set_cpu_name("Ryzen 9 7940HS".into());
        data.set_igpu_name("Radeon 680M".into());
        data.set_dgpu_name("GeForce RTX 4060 Mobile".into());
        data.set_has_dgpu(true);
        data.set_has_igpu(true);
        data.set_dgpu_suspended(false);
        data.set_cpu_temp_val(47.0);
        data.set_gpu_temp_val(44.0);
        data.set_igpu_temp_val(45.0);
        data.set_cpu_usage_val(18.0);
        data.set_gpu_usage_val(3.0);
        data.set_igpu_usage_val(2.0);
        data.set_ram_usage_val(41.0);
        data.set_cpu_freq_mhz(3800.0);
        data.set_cpu_fan_rpm(1900);
        data.set_gpu_fan_rpm(2100);
        data.set_mid_fan_rpm(-1);

        data.set_panel_overdrive(1);
        data.set_boot_sound(-1);
        data.set_screen_auto_brightness(-1);
        data.set_mcu_powersave(-1);
        data.set_mini_led_mode(-1);
        data.set_screenpad_brightness(-1);

        data.set_ppt_pl1_spl(AttrMinMax {
            min: 25,
            max: 65,
            current: 45.0,
        });
        data.set_ppt_pl2_sppt(AttrMinMax {
            min: 25,
            max: 80,
            current: 65.0,
        });
        data.set_ppt_pl3_fppt(AttrMinMax {
            min: 0,
            max: 0,
            current: -1.0,
        });
        data.set_ppt_fppt(AttrMinMax {
            min: 0,
            max: 0,
            current: -1.0,
        });
        data.set_ppt_apu_sppt(AttrMinMax {
            min: 0,
            max: 0,
            current: -1.0,
        });
        data.set_ppt_platform_sppt(AttrMinMax {
            min: 0,
            max: 0,
            current: -1.0,
        });
        data.set_nv_dynamic_boost(AttrMinMax {
            min: 5,
            max: 25,
            current: 15.0,
        });
        data.set_nv_tgp(AttrMinMax {
            min: 60,
            max: 140,
            current: 115.0,
        });
        data.set_nv_temp_target(AttrMinMax {
            min: 50,
            max: 87,
            current: 87.0,
        });
        data.set_ppt_enabled_available(false);
        data.set_enable_ppt_group(false);

        let weak_cb = weak.clone();
        data.on_cb_platform_profile(move |index| {
            if let Some(ui) = weak_cb.upgrade() {
                ui.global::<SystemPageData>().set_platform_profile(index);
            }
        });
        let weak_cb = weak.clone();
        data.on_cb_charge_control_end_threshold(move |value| {
            if let Some(ui) = weak_cb.upgrade() {
                ui.global::<SystemPageData>()
                    .set_charge_control_end_threshold(value as f32);
            }
        });
        let weak_cb = weak.clone();
        data.on_cb_panel_overdrive(move |value| {
            if let Some(ui) = weak_cb.upgrade() {
                ui.global::<SystemPageData>().set_panel_overdrive(value);
            }
        });
    }

    // ─── FanPageData ───────────────────────────────────────────────────
    {
        let data = ui.global::<FanPageData>();
        // Spread demo curve nodes so adjacent point labels don't overlap
        let demo_curve = [
            crate::Node { x: 10.0, y: 15.0 },
            crate::Node { x: 25.0, y: 22.0 },
            crate::Node { x: 40.0, y: 33.0 },
            crate::Node { x: 52.0, y: 45.0 },
            crate::Node { x: 63.0, y: 58.0 },
            crate::Node { x: 73.0, y: 70.0 },
            crate::Node { x: 84.0, y: 84.0 },
            crate::Node { x: 95.0, y: 100.0 },
        ];
        data.set_balanced_cpu(demo_curve.as_slice().into());
        data.set_balanced_gpu(demo_curve.as_slice().into());
        data.set_performance_cpu(demo_curve.as_slice().into());
        data.set_performance_gpu(demo_curve.as_slice().into());
        data.set_quiet_cpu(demo_curve.as_slice().into());
        data.set_quiet_gpu(demo_curve.as_slice().into());
        data.set_available_profiles(
            [
                Profile::Balanced,
                Profile::Performance,
                Profile::Quiet,
            ]
            .as_slice()
            .into(),
        );
        data.set_available_fans([FanType::CPU, FanType::GPU].as_slice().into());
        data.set_mid_fan_available(false);
        data.set_quiet_available(true);
    }

    // ─── GPUPageData ───────────────────────────────────────────────────
    {
        let data = ui.global::<GPUPageData>();
        data.set_gpu_modes(
            ["Integrated", "Hybrid", "Ultimate"]
                .iter()
                .map(|s| SharedString::from(*s))
                .collect::<Vec<_>>()
                .as_slice()
                .into(),
        );
        data.set_gpu_mode_index(2);
        data.set_gpu_switchable(true);
        data.set_gpu_dropdown_enabled(true);

        let weak_cb = weak.clone();
        data.on_cb_set_gpu_mode(move |index| {
            if let Some(ui) = weak_cb.upgrade() {
                ui.global::<GPUPageData>().set_gpu_mode_index(index);
            }
        });
    }

    // ─── AuraPageData ──────────────────────────────────────────────────
    {
        let data = ui.global::<AuraPageData>();
        data.set_device_type(AuraDevType::New);
        data.set_supported_basic_modes([0i32, 1, 2].into());
        data.set_available_mode_names(
            ["Static", "Breathe", "Strobe"]
                .iter()
                .map(|s| SharedString::from(*s))
                .collect::<Vec<_>>()
                .as_slice()
                .into(),
        );
        data.set_current_available_mode(0);
        data.set_brightness(3);

        let effect = AuraEffect {
            mode: 0,
            zone: 0,
            colour1: Color::from_rgb_u8(0xff, 0x00, 0x00),
            colour2: Color::from_rgb_u8(0x00, 0xc8, 0xff),
            speed: 1,
            direction: 0,
        };
        data.set_color1(effect.colour1);
        data.set_color2(effect.colour2);
        data.set_colorbox1(Brush::from(effect.colour1));
        data.set_colorbox2(Brush::from(effect.colour2));
        data.set_led_mode_data(effect);

        // apply_effect has a slint-side implementation which updates
        // led_mode_data then calls apply_led_mode_data: mirror the new state
        // into the swatches and the mode dropdown index.
        let weak_cb = weak.clone();
        data.on_apply_led_mode_data(move || {
            let Some(ui) = weak_cb.upgrade() else {
                return;
            };
            let data = ui.global::<AuraPageData>();
            let effect = data.get_led_mode_data();
            data.set_color1(effect.colour1);
            data.set_color2(effect.colour2);
            data.set_colorbox1(Brush::from(effect.colour1));
            data.set_colorbox2(Brush::from(effect.colour2));
            let supported = data.get_supported_basic_modes();
            for i in 0..supported.row_count() {
                if supported.row_data(i) == Some(effect.mode) {
                    data.set_current_available_mode(i as i32);
                    break;
                }
            }
        });
    }

    // ─── SlashPageData ─────────────────────────────────────────────────
    {
        let data = ui.global::<SlashPageData>();
        data.set_enabled(true);
        data.set_mode_choices(
            ["Static", "Bounce", "Flash", "Laser", "Rainbow", "Scan"]
                .iter()
                .map(|s| SharedString::from(*s))
                .collect::<Vec<_>>()
                .as_slice()
                .into(),
        );
        data.set_mode(1);
        data.set_brightness(220);
        data.set_interval(1);

        let weak_cb = weak.clone();
        data.on_cb_enabled(move |value| {
            if let Some(ui) = weak_cb.upgrade() {
                ui.global::<SlashPageData>().set_enabled(value);
            }
        });
        let weak_cb = weak.clone();
        data.on_cb_mode(move |value| {
            if let Some(ui) = weak_cb.upgrade() {
                ui.global::<SlashPageData>().set_mode(value);
            }
        });
        let weak_cb = weak.clone();
        data.on_cb_brightness(move |value| {
            if let Some(ui) = weak_cb.upgrade() {
                ui.global::<SlashPageData>().set_brightness(value);
            }
        });
    }

    // Optional page override for screenshots/tests: ROGCC_DEMO_PAGE=<index>
    if let Ok(page) = std::env::var("ROGCC_DEMO_PAGE")
        .map_err(|_| ())
        .and_then(|page| page.parse::<i32>().map_err(|_| ()))
    {
        ui.global::<crate::NavState>().set_page(page);
    }

    ui.on_exit_app(|| {
        if let Err(e) = slint::quit_event_loop() {
            log::warn!("Failed to quit event loop: {e:?}");
        }
    });

    // Live-ish sensor values so the dashboard feels alive in the demo.
    let timer = Timer::default();
    let tick = Rc::new(Cell::new(0.0f64));
    let weak_tick = weak.clone();
    timer.start(TimerMode::Repeated, Duration::from_secs(2), move || {
        tick.set(tick.get() + 1.0);
        let t = tick.get();
        let Some(ui) = weak_tick.upgrade() else {
            return;
        };
        let data = ui.global::<SystemPageData>();
        let sin = |x: f64| x.sin() as f32;
        data.set_cpu_temp_val(47.0 + 4.0 * sin(0.35 * t));
        data.set_gpu_temp_val(44.0 + 3.0 * sin(0.3 * t + 1.3));
        data.set_igpu_temp_val(45.0 + 2.0 * sin(0.25 * t + 0.4));
        data.set_cpu_fan_rpm((1900.0 + 260.0 * sin(0.5 * t)) as i32);
        data.set_gpu_fan_rpm((2100.0 + 300.0 * sin(0.45 * t + 0.7)) as i32);
        data.set_cpu_usage_val((18.0 + 14.0 * sin(0.8 * t)).max(2.0));
        data.set_gpu_usage_val((4.0 + 3.0 * sin(0.6 * t + 2.0)).max(0.0));
    });

    ui.window().show().map_err(demo_err)?;
    if let Err(e) = slint::run_event_loop_until_quit() {
        log::error!("Slint event loop error: {e:?}");
    }
    Ok(())
}
