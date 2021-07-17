use crate::{Button, MouseButton};
use std::mem::{size_of, transmute_copy};
use winapi::um::winuser::{
    SendInput, INPUT, INPUT_MOUSE, LPINPUT, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP,
    MOUSEEVENTF_MIDDLEDOWN, MOUSEEVENTF_MIDDLEUP, MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP,
    MOUSEEVENTF_XDOWN, MOUSEEVENTF_XUP, MOUSEINPUT, XBUTTON1, XBUTTON2,
};

impl Button for MouseButton {
    fn press(&self) {
        mouse_press(*self)
    }

    fn click(&self) {
        mouse_click(*self);
    }

    fn release(&self) {
        mouse_release(*self);
    }
}

fn mouse_interact_with(interaction: u32, mouse_data: u16) {
    unsafe {
        let mut x = INPUT {
            type_: INPUT_MOUSE,
            u: transmute_copy(&MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: mouse_data.into(),
                time: 0,
                dwFlags: interaction,
                dwExtraInfo: 0,
            }),
        };

        SendInput(1, &mut x as LPINPUT, size_of::<INPUT>() as libc::c_int);
    }
}

pub fn mouse_press(button: MouseButton) {
    let click = button_to_event_down(button) | button_to_event_up(button);
    mouse_interact_with(click, button_to_mouse_data(button))
}

pub fn mouse_release(button: MouseButton) {
    mouse_interact_with(button_to_event_up(button), button_to_mouse_data(button))
}

pub fn mouse_click(button: MouseButton) {
    mouse_interact_with(button_to_event_down(button), button_to_mouse_data(button))
}

fn button_to_mouse_data(button: MouseButton) -> u16 {
    match button {
        MouseButton::Side | MouseButton::DoubleSide => XBUTTON1,
        MouseButton::Extra | MouseButton::DoubleExtra => XBUTTON2,
        _ => 0,
    }
}

fn button_to_event_up(button: MouseButton) -> u32 {
    use MouseButton::*;
    match button {
        Left | DoubleLeft => MOUSEEVENTF_LEFTDOWN,
        Right | DoubleRight => MOUSEEVENTF_RIGHTDOWN,
        Middle | DoubleMiddle => MOUSEEVENTF_MIDDLEDOWN,
        Side | DoubleSide | Extra | DoubleExtra => MOUSEEVENTF_XDOWN,
    }
}

fn button_to_event_down(button: MouseButton) -> u32 {
    use MouseButton::*;
    match button {
        Left | DoubleLeft => MOUSEEVENTF_LEFTUP,
        Right | DoubleRight => MOUSEEVENTF_RIGHTUP,
        Middle | DoubleMiddle => MOUSEEVENTF_MIDDLEUP,
        Side | DoubleSide | Extra | DoubleExtra => MOUSEEVENTF_XUP,
    }
}
