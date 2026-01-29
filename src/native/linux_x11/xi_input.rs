#![allow(non_upper_case_globals, non_snake_case)]

use super::{
    libx11::{self, Display, Window, _XPrivDisplay},
    xi_input,
};

pub const XIAllDevices: libc::c_int = 0 as libc::c_int;
pub const XIAllMasterDevices: libc::c_int = 1 as libc::c_int;

// XInput2 event types
pub const XI_DeviceChanged: libc::c_int = 1;
pub const XI_KeyPress: libc::c_int = 2;
pub const XI_KeyRelease: libc::c_int = 3;
pub const XI_ButtonPress: libc::c_int = 4;
pub const XI_ButtonRelease: libc::c_int = 5;
pub const XI_Motion: libc::c_int = 6;
pub const XI_Enter: libc::c_int = 7;
pub const XI_Leave: libc::c_int = 8;
pub const XI_FocusIn: libc::c_int = 9;
pub const XI_FocusOut: libc::c_int = 10;
pub const XI_HierarchyChanged: libc::c_int = 11;
pub const XI_PropertyEvent: libc::c_int = 12;
pub const XI_RawKeyPress: libc::c_int = 13;
pub const XI_RawKeyRelease: libc::c_int = 14;
pub const XI_RawButtonPress: libc::c_int = 15;
pub const XI_RawButtonRelease: libc::c_int = 16;
pub const XI_RawMotion: libc::c_int = 17;
pub const XI_TouchBegin: libc::c_int = 18;
pub const XI_TouchUpdate: libc::c_int = 19;
pub const XI_TouchEnd: libc::c_int = 20;
pub const XI_TouchOwnership: libc::c_int = 21;
pub const XI_RawTouchBegin: libc::c_int = 22;
pub const XI_RawTouchUpdate: libc::c_int = 23;
pub const XI_RawTouchEnd: libc::c_int = 24;

// Event masks
pub const XI_RawMotionMask: libc::c_int = (1 as libc::c_int) << XI_RawMotion;
pub const XI_MotionMask: libc::c_int = (1 as libc::c_int) << XI_Motion;
pub const XI_ButtonPressMask: libc::c_int = (1 as libc::c_int) << XI_ButtonPress;
pub const XI_ButtonReleaseMask: libc::c_int = (1 as libc::c_int) << XI_ButtonRelease;
pub const XI_EnterMask: libc::c_int = (1 as libc::c_int) << XI_Enter;
pub const XI_LeaveMask: libc::c_int = (1 as libc::c_int) << XI_Leave;

// Device types
pub const XIMasterPointer: libc::c_int = 1;
pub const XIMasterKeyboard: libc::c_int = 2;
pub const XISlavePointer: libc::c_int = 3;
pub const XISlaveKeyboard: libc::c_int = 4;
pub const XIFloatingSlave: libc::c_int = 5;

// Valuator modes
pub const XIModeRelative: libc::c_int = 0;
pub const XIModeAbsolute: libc::c_int = 1;

// Device classes
pub const XIKeyClass: libc::c_int = 0;
pub const XIButtonClass: libc::c_int = 1;
pub const XIValuatorClass: libc::c_int = 2;
pub const XIScrollClass: libc::c_int = 3;
pub const XITouchClass: libc::c_int = 8;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIEventMask {
    pub deviceid: libc::c_int,
    pub mask_len: libc::c_int,
    pub mask: *mut libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIButtonState {
    pub mask_len: libc::c_int,
    pub mask: *mut libc::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIModifierState {
    pub base: libc::c_int,
    pub latched: libc::c_int,
    pub locked: libc::c_int,
    pub effective: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIGroupState {
    pub base: libc::c_int,
    pub latched: libc::c_int,
    pub locked: libc::c_int,
    pub effective: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIValuatorState {
    pub mask_len: libc::c_int,
    pub mask: *mut libc::c_uchar,
    pub values: *mut libc::c_double,
}

pub type Time = libc::c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIRawEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub time: Time,
    pub deviceid: libc::c_int,
    pub sourceid: libc::c_int,
    pub detail: libc::c_int,
    pub flags: libc::c_int,
    pub valuators: XIValuatorState,
    pub raw_values: *mut libc::c_double,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIDeviceEvent {
    pub type_0: libc::c_int,
    pub serial: libc::c_ulong,
    pub send_event: libc::c_int,
    pub display: *mut Display,
    pub extension: libc::c_int,
    pub evtype: libc::c_int,
    pub time: Time,
    pub deviceid: libc::c_int,
    pub sourceid: libc::c_int,
    pub detail: libc::c_int,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: libc::c_double,
    pub root_y: libc::c_double,
    pub event_x: libc::c_double,
    pub event_y: libc::c_double,
    pub flags: libc::c_int,
    pub buttons: XIButtonState,
    pub valuators: XIValuatorState,
    pub mods: XIModifierState,
    pub group: XIGroupState,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIDeviceInfo {
    pub deviceid: libc::c_int,
    pub name: *mut libc::c_char,
    pub use_: libc::c_int,
    pub attachment: libc::c_int,
    pub enabled: libc::c_int,
    pub num_classes: libc::c_int,
    pub classes: *mut *mut XIAnyClassInfo,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIAnyClassInfo {
    pub type_: libc::c_int,
    pub sourceid: libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XIValuatorClassInfo {
    pub type_: libc::c_int,
    pub sourceid: libc::c_int,
    pub number: libc::c_int,
    pub label: libc::c_ulong,
    pub min: libc::c_double,
    pub max: libc::c_double,
    pub value: libc::c_double,
    pub resolution: libc::c_int,
    pub mode: libc::c_int,
}

use core::ffi::{c_char, c_int};
crate::declare_module!(
    LibXi,
    "libXi.so",
    "libXi.so.6",
    ...
    ...
    pub fn XQueryExtension(*mut Display, *const c_char, *mut c_int, *mut c_int, *mut c_int) -> c_int,
    pub fn XIQueryVersion(*mut Display, *mut c_int, *mut c_int) -> c_int,
    pub fn XISelectEvents(*mut Display, Window, *mut XIEventMask, c_int),
    pub fn XIQueryDevice(*mut Display, c_int, *mut c_int) -> *mut XIDeviceInfo,
    pub fn XIFreeDeviceInfo(*mut XIDeviceInfo),
    pub fn XGetEventData(*mut Display, *mut libx11::XGenericEventCookie) -> c_int,
    pub fn XFreeEventData(*mut Display, *mut libx11::XGenericEventCookie),
    ...
    ...
    pub xi_extension_opcode: Option<i32>,
);

impl LibXi {
    pub unsafe fn query_xi_extension(
        &mut self,
        libx11: &mut libx11::LibX11,
        display: *mut Display,
        window: Window,
    ) -> Vec<TabletDevice> {
        let mut ev = 0;
        let mut err = 0;
        let mut xi_opcode = 0;

        if (libx11.XQueryExtension)(
            display,
            b"XInputExtension\x00" as *const u8 as *const libc::c_char,
            &mut xi_opcode,
            &mut ev,
            &mut err,
        ) == 0
        {
            eprintln!("XInput extension not available");
            return Vec::new();
        }

        // check the version of XInput
        let mut major = 2;
        let mut minor = 3;
        if (self.XIQueryVersion)(display, &mut major, &mut minor) != 0 {
            eprintln!("XInput version query failed");
            return Vec::new();
        }

        eprintln!("XInput {}.{} initialized", major, minor);

        // Create a proper bit mask for XI events
        // XInput2 uses a byte array where each bit represents an event type
        // We need enough bytes to cover event type 24 (XI_RawTouchEnd)
        let mut mask_bytes: [libc::c_uchar; 4] = [0; 4];

        // Helper macro to set a bit in the mask
        macro_rules! xi_set_mask {
            ($mask:expr, $event:expr) => {
                $mask[($event >> 3) as usize] |= 1 << ($event & 7);
            };
        }

        // Select events on the window for tablet input
        xi_set_mask!(mask_bytes, XI_Motion);
        xi_set_mask!(mask_bytes, XI_ButtonPress);
        xi_set_mask!(mask_bytes, XI_ButtonRelease);
        xi_set_mask!(mask_bytes, XI_Enter);
        xi_set_mask!(mask_bytes, XI_Leave);

        let mut masks = XIEventMask {
            deviceid: XIAllDevices,
            mask_len: mask_bytes.len() as _,
            mask: mask_bytes.as_mut_ptr(),
        };

        (self.XISelectEvents)(display, window, &mut masks, 1 as libc::c_int);

        // Also select raw motion on root window
        let mut raw_mask_bytes: [libc::c_uchar; 4] = [0; 4];
        xi_set_mask!(raw_mask_bytes, XI_RawMotion);

        let mut raw_masks = XIEventMask {
            deviceid: XIAllDevices,
            mask_len: raw_mask_bytes.len() as _,
            mask: raw_mask_bytes.as_mut_ptr(),
        };

        (self.XISelectEvents)(
            display,
            // this weird pointers is macro expansion of DefaultRootWindow(display)
            (*(*(display as _XPrivDisplay))
                .screens
                .offset((*(display as _XPrivDisplay)).default_screen as isize))
            .root,
            &mut raw_masks,
            1 as libc::c_int,
        );
        self.xi_extension_opcode = Some(xi_opcode);

        // Query and return tablet devices
        let devices = self.query_devices(libx11, display);
        eprintln!("Found {} tablet device(s)", devices.len());
        for device in &devices {
            eprintln!(
                "  Tablet: {} (ID: {}) - Pressure: {}, Tilt: {}, Rotation: {}, Distance: {}",
                device.name,
                device.device_id,
                device.has_pressure,
                device.has_tilt,
                device.has_rotation,
                device.has_distance
            );
        }
        devices
    }

    /// Get mouse delta from XI_RawMotion's event XGenericEventCookie data
    pub unsafe fn read_cookie(
        &mut self,
        xcookie: &mut libx11::XGenericEventCookie,
        display: *mut Display,
    ) -> (f64, f64) {
        assert!(xcookie.evtype == xi_input::XI_RawMotion);

        (self.XGetEventData)(display, xcookie);

        let raw_event = xcookie.data as *mut xi_input::XIRawEvent;

        let dx = *(*raw_event).raw_values;
        let dy = *(*raw_event).raw_values.offset(1);

        (self.XFreeEventData)(display, &mut (*xcookie) as *mut _);

        (dx, dy)
    }

    /// Read XIDeviceEvent from cookie and extract valuator data
    pub unsafe fn read_device_event(
        &self,
        xcookie: &mut libx11::XGenericEventCookie,
        display: *mut Display,
    ) -> Option<XIDeviceEvent> {
        (self.XGetEventData)(display, xcookie);

        if xcookie.data.is_null() {
            return None;
        }

        let device_event = *(xcookie.data as *mut XIDeviceEvent);

        (self.XFreeEventData)(display, xcookie);

        Some(device_event)
    }

    /// Get valuator value by index from XIDeviceEvent
    pub unsafe fn get_valuator_value(event: &XIDeviceEvent, valuator_number: i32) -> Option<f64> {
        let valuators = &event.valuators;
        let mask = valuators.mask;
        let values = valuators.values;

        if mask.is_null() || values.is_null() {
            return None;
        }

        let mut value_index = 0;
        for i in 0..valuator_number {
            if Self::xi_mask_is_set(mask, i) {
                value_index += 1;
            }
        }

        if Self::xi_mask_is_set(mask, valuator_number) {
            Some(*values.offset(value_index as isize))
        } else {
            None
        }
    }

    /// Check if a bit is set in the XI mask
    unsafe fn xi_mask_is_set(mask: *const libc::c_uchar, bit: i32) -> bool {
        let byte = bit / 8;
        let bit_in_byte = bit % 8;
        (*mask.offset(byte as isize) & (1 << bit_in_byte)) != 0
    }

    /// Query all XI devices and find tablet devices
    pub unsafe fn query_devices(
        &self,
        libx11: &libx11::LibX11,
        display: *mut Display,
    ) -> Vec<TabletDevice> {
        let mut num_devices = 0;
        let devices = (self.XIQueryDevice)(display, XIAllDevices, &mut num_devices);

        if devices.is_null() {
            return Vec::new();
        }

        let mut tablet_devices = Vec::new();

        eprintln!("Querying {} XI devices...", num_devices);

        for i in 0..num_devices {
            let device = &*devices.offset(i as isize);

            let device_name = if !device.name.is_null() {
                std::ffi::CStr::from_ptr(device.name)
                    .to_string_lossy()
                    .to_string()
            } else {
                String::new()
            };

            eprintln!(
                "  Device {}: '{}' (ID: {}, use: {}, enabled: {})",
                i, device_name, device.deviceid, device.use_, device.enabled
            );

            // Check if this is a slave pointer or floating slave device (actual hardware)
            // XISlavePointer = 2, XIFloatingSlave = 4
            if device.use_ != XISlavePointer && device.use_ != XIFloatingSlave {
                eprintln!(
                    "    Skipping: not a slave/floating pointer (use={})",
                    device.use_
                );
                continue;
            }

            let device_name_lower = device_name.to_lowercase();

            // Look for tablet-related keywords in device name
            let is_tablet = device_name_lower.contains("pen")
                || device_name_lower.contains("stylus")
                || device_name_lower.contains("eraser")
                || device_name_lower.contains("wacom")
                || device_name_lower.contains("tablet");

            // Also check for devices that have pressure capability
            let mut has_pressure_axis = false;
            for j in 0..device.num_classes {
                let class = *device.classes.offset(j as isize);
                let class_type = (*class).type_;

                if class_type == XIValuatorClass {
                    let valuator = class as *mut XIValuatorClassInfo;
                    let label_atom = (*valuator).label;

                    if label_atom != 0 {
                        let label_name = (libx11.XGetAtomName)(display, label_atom);
                        if !label_name.is_null() {
                            let label = std::ffi::CStr::from_ptr(label_name)
                                .to_string_lossy()
                                .to_lowercase();

                            if label.contains("pressure") {
                                has_pressure_axis = true;
                            }

                            (libx11.XFree)(label_name as *mut _);
                        }
                    }
                }
            }

            if !is_tablet && !has_pressure_axis {
                eprintln!(
                    "    Skipping: not a tablet device (no pressure axis or tablet keywords)"
                );
                continue;
            }

            eprintln!("    ✓ Detected as tablet device!");

            let mut tablet_device = TabletDevice {
                device_id: device.deviceid,
                name: device_name.clone(),
                has_pressure: false,
                has_tilt: false,
                has_rotation: false,
                has_distance: false,
                pressure_axis: -1,
                tilt_x_axis: -1,
                tilt_y_axis: -1,
                rotation_axis: -1,
                distance_axis: -1,
                pressure_min: 0.0,
                pressure_max: 1.0,
                tilt_x_min: -90.0,
                tilt_x_max: 90.0,
                tilt_y_min: -90.0,
                tilt_y_max: 90.0,
                rotation_min: 0.0,
                rotation_max: 360.0,
                distance_min: 0.0,
                distance_max: 1.0,
            };

            // Examine device classes to find valuator information
            for j in 0..device.num_classes {
                let class = *device.classes.offset(j as isize);
                let class_type = (*class).type_;

                if class_type == XIValuatorClass {
                    let valuator = class as *mut XIValuatorClassInfo;
                    let label_atom = (*valuator).label;

                    if label_atom != 0 {
                        let label_name = (libx11.XGetAtomName)(display, label_atom);
                        if !label_name.is_null() {
                            let label = std::ffi::CStr::from_ptr(label_name)
                                .to_string_lossy()
                                .to_lowercase();

                            // Map common axis labels to our tablet device structure
                            if label.contains("pressure") {
                                tablet_device.has_pressure = true;
                                tablet_device.pressure_axis = (*valuator).number;
                                tablet_device.pressure_min = (*valuator).min;
                                tablet_device.pressure_max = (*valuator).max;
                            } else if label.contains("tilt") && label.contains("x") {
                                tablet_device.has_tilt = true;
                                tablet_device.tilt_x_axis = (*valuator).number;
                                tablet_device.tilt_x_min = (*valuator).min;
                                tablet_device.tilt_x_max = (*valuator).max;
                            } else if label.contains("tilt") && label.contains("y") {
                                tablet_device.has_tilt = true;
                                tablet_device.tilt_y_axis = (*valuator).number;
                                tablet_device.tilt_y_min = (*valuator).min;
                                tablet_device.tilt_y_max = (*valuator).max;
                            } else if label.contains("rotation") || label.contains("twist") {
                                tablet_device.has_rotation = true;
                                tablet_device.rotation_axis = (*valuator).number;
                                tablet_device.rotation_min = (*valuator).min;
                                tablet_device.rotation_max = (*valuator).max;
                            } else if label.contains("distance") {
                                tablet_device.has_distance = true;
                                tablet_device.distance_axis = (*valuator).number;
                                tablet_device.distance_min = (*valuator).min;
                                tablet_device.distance_max = (*valuator).max;
                            }

                            (libx11.XFree)(label_name as *mut _);
                        }
                    }
                }
            }

            tablet_devices.push(tablet_device);
        }

        (self.XIFreeDeviceInfo)(devices);

        tablet_devices
    }
}

/// Information about a detected tablet device
#[derive(Clone, Debug)]
pub struct TabletDevice {
    pub device_id: i32,
    pub name: String,
    pub has_pressure: bool,
    pub has_tilt: bool,
    pub has_rotation: bool,
    pub has_distance: bool,
    pub pressure_axis: i32,
    pub tilt_x_axis: i32,
    pub tilt_y_axis: i32,
    pub rotation_axis: i32,
    pub distance_axis: i32,
    pub pressure_min: f64,
    pub pressure_max: f64,
    pub tilt_x_min: f64,
    pub tilt_x_max: f64,
    pub tilt_y_min: f64,
    pub tilt_y_max: f64,
    pub rotation_min: f64,
    pub rotation_max: f64,
    pub distance_min: f64,
    pub distance_max: f64,
}

impl TabletDevice {
    /// Extract pen input data from an XIDeviceEvent
    pub unsafe fn extract_pen_data(&self, event: &XIDeviceEvent) -> crate::event::PenInput {
        let mut pen_data = crate::event::PenInput {
            x: event.event_x as f32,
            y: event.event_y as f32,
            pressure: 0.5, // Default pressure if not available
            tilt_x: 0.0,
            tilt_y: 0.0,
            distance: 0.0,
            rotation: 0.0,
        };

        // Extract pressure
        if self.has_pressure && self.pressure_axis >= 0 {
            if let Some(value) = LibXi::get_valuator_value(event, self.pressure_axis) {
                let normalized =
                    (value - self.pressure_min) / (self.pressure_max - self.pressure_min);
                pen_data.pressure = normalized.clamp(0.0, 1.0) as f32;
            }
        }

        // Extract tilt X
        if self.has_tilt && self.tilt_x_axis >= 0 {
            if let Some(value) = LibXi::get_valuator_value(event, self.tilt_x_axis) {
                pen_data.tilt_x = value as f32;
            }
        }

        // Extract tilt Y
        if self.has_tilt && self.tilt_y_axis >= 0 {
            if let Some(value) = LibXi::get_valuator_value(event, self.tilt_y_axis) {
                pen_data.tilt_y = value as f32;
            }
        }

        // Extract rotation
        if self.has_rotation && self.rotation_axis >= 0 {
            if let Some(value) = LibXi::get_valuator_value(event, self.rotation_axis) {
                let normalized =
                    (value - self.rotation_min) / (self.rotation_max - self.rotation_min);
                pen_data.rotation = (normalized * 360.0) as f32;
            }
        }

        // Extract distance
        if self.has_distance && self.distance_axis >= 0 {
            if let Some(value) = LibXi::get_valuator_value(event, self.distance_axis) {
                let normalized =
                    (value - self.distance_min) / (self.distance_max - self.distance_min);
                pen_data.distance = normalized as f32;
            }
        }

        pen_data
    }

    /// Determine the tool type from device name
    pub fn get_tool_type(&self) -> crate::event::PenToolType {
        let name = self.name.to_lowercase();
        if name.contains("eraser") {
            crate::event::PenToolType::Eraser
        } else if name.contains("pen") || name.contains("stylus") {
            crate::event::PenToolType::Pen
        } else if name.contains("brush") {
            crate::event::PenToolType::Brush
        } else if name.contains("airbrush") {
            crate::event::PenToolType::Airbrush
        } else {
            crate::event::PenToolType::Pen // Default to pen
        }
    }
}
