use winit::{
    event::{Event, WindowEvent, StartCause},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use winit::platform::ios::WindowExtIOS;
use uikit_sys::{
    id,
    UIView,
    UIColor,
    IUIColor,
    UIView_UIViewRendering,
};

#[no_mangle]
pub extern fn run_app() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    let root_view: UIView = UIView(window.ui_view() as id);
    unsafe {
        root_view.setBackgroundColor_(UIColor::yellowColor());
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                add_label("THIS IS SOME TEXT".to_string(), root_view);
                println!("The app has started!");
            },
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}

fn add_label(label_text: String, root_view: UIView) {
    use uikit_sys::{
        UILabel,
        IUILabel,
        NSString,
        INSObject,
        UIView_UIViewHierarchy,
        UIView_UIViewGeometry,
        NSString_NSStringExtensionMethods,
        NSUTF8StringEncoding,
        CGRect,
        CGPoint,
        CGSize,
    };

    use std::{
        ffi::CString,
        convert::TryInto,
    };

    let text = CString::new(label_text.as_str()).expect("CString::new failed");
    let text_ptr = text.as_ptr();
    let text_length = label_text.len().try_into().unwrap();
    unsafe {
        let label = UILabel::alloc();
        label.init();

        let text = NSString(
            NSString::alloc().initWithBytes_length_encoding_(
                text_ptr as *mut std::ffi::c_void,
                text_length,
                NSUTF8StringEncoding,
            ),
        );

        label.setFrame_(CGRect {
            origin: CGPoint {
                x: 20.0,
                y: 20.0,
            },
            size: CGSize {
                width: 200.0,
                height: 40.0,
            },
        });
        label.setText_(text);

        root_view.addSubview_(UIView(label.0));
    }
}
