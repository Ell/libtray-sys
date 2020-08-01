/* automatically generated by rust-bindgen 0.54.1 */

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tray {
    pub icon: *mut ::std::os::raw::c_char,
    pub menu: *mut tray_menu,
}
#[test]
fn bindgen_test_layout_tray() {
    assert_eq!(
        ::std::mem::size_of::<tray>(),
        16usize,
        concat!("Size of: ", stringify!(tray))
    );
    assert_eq!(
        ::std::mem::align_of::<tray>(),
        8usize,
        concat!("Alignment of ", stringify!(tray))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tray>())).icon as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tray),
            "::",
            stringify!(icon)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tray>())).menu as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tray),
            "::",
            stringify!(menu)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tray_menu {
    pub text: *mut ::std::os::raw::c_char,
    pub disabled: ::std::os::raw::c_int,
    pub checked: ::std::os::raw::c_int,
    pub cb: ::std::option::Option<unsafe extern "C" fn(arg1: *mut tray_menu)>,
    pub context: *mut ::std::os::raw::c_void,
    pub submenu: *mut tray_menu,
}
#[test]
fn bindgen_test_layout_tray_menu() {
    assert_eq!(
        ::std::mem::size_of::<tray_menu>(),
        40usize,
        concat!("Size of: ", stringify!(tray_menu))
    );
    assert_eq!(
        ::std::mem::align_of::<tray_menu>(),
        8usize,
        concat!("Alignment of ", stringify!(tray_menu))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tray_menu>())).text as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tray_menu),
            "::",
            stringify!(text)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tray_menu>())).disabled as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tray_menu),
            "::",
            stringify!(disabled)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tray_menu>())).checked as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tray_menu),
            "::",
            stringify!(checked)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tray_menu>())).cb as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tray_menu),
            "::",
            stringify!(cb)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tray_menu>())).context as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(tray_menu),
            "::",
            stringify!(context)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tray_menu>())).submenu as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tray_menu),
            "::",
            stringify!(submenu)
        )
    );
}