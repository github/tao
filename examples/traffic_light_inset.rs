// Copyright 2026 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0

#[cfg(target_os = "macos")]
fn main() {
  use objc2_app_kit::{NSView, NSWindow, NSWindowButton};
  use tao::{
    dpi::{LogicalPosition, LogicalSize},
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    platform::{
      macos::{WindowBuilderExtMacOS, WindowExtMacOS},
      run_return::EventLoopExtRunReturn,
    },
    window::WindowBuilder,
  };

  let mut event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_visible(false)
    .with_titlebar_transparent(true)
    .with_title_hidden(true)
    .with_fullsize_content_view(true)
    .with_traffic_light_inset(LogicalPosition::new(19.0, 24.0))
    .build(&event_loop)
    .unwrap();
  let ns_window = unsafe { &*window.ns_window().cast::<NSWindow>() };
  let measure = || {
    [
      NSWindowButton::CloseButton,
      NSWindowButton::MiniaturizeButton,
      NSWindowButton::ZoomButton,
    ]
    .map(|kind| {
      let button = ns_window.standardWindowButton(kind).unwrap();
      let rect = button.convertRect_toView(NSView::bounds(&button), None);
      (
        rect.origin.x,
        ns_window.frame().size.height - rect.origin.y - rect.size.height,
        rect.size.width,
        rect.size.height,
      )
    })
  };

  println!("initial: {:?}", measure());
  assert_eq!(measure()[0].0, 19.0, "builder must apply before first show");
  window.set_traffic_light_inset(LogicalPosition::new(31.0, 46.0));
  let expected = measure();
  println!("immediate setter: {expected:?}");
  assert_eq!(expected[0].0, 31.0, "setter must apply before any redraw");

  let mut step = 0;
  event_loop.run_return(|event, _, control_flow| {
    *control_flow = ControlFlow::Poll;
    if !matches!(event, Event::MainEventsCleared) {
      return;
    }
    step += 1;
    match step {
      1 => window.set_title("Changed title"),
      3 => {
        let actual = measure();
        println!("after title: {actual:?}");
        assert_eq!(actual, expected, "title changes must preserve the inset");
        window.set_inner_size(LogicalSize::new(900.0, 700.0));
      }
      5 => {
        let actual = measure();
        println!("after resize: {actual:?}");
        assert_eq!(actual, expected, "resize must preserve the inset");
        *control_flow = ControlFlow::Exit;
      }
      _ => {}
    }
  });
}

#[cfg(not(target_os = "macos"))]
fn main() {}
