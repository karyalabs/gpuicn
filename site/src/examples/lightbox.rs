//! Lightbox examples.

use crate::*;

/// Single image
///
/// With one image and no actions the viewer shows only zoom and close.
pub(crate) fn single_image(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let image = || {
        LightboxImage::new(
            "https://raw.githubusercontent.com/karyalabs/gpuicn/0f4ecbfad9ccd443af0925137c88d87996e999f7/docs/images/catalog-hero.png",
            "catalog-hero.png",
        )
    };
    let handle = window
        .use_keyed_state("single.handle", cx, |_, _| DialogHandle::new(false))
        .read(cx)
        .clone();
    let state = window.use_keyed_state("single.state", cx, |_, _| LightboxState::new([image()]));
    let open = handle.clone();
    div()
        .child(
            Button::new("single.open")
                .variant(ButtonVariant::Outline)
                .label("View screenshot")
                .on_click(move |_, window, cx| open.open(window, cx)),
        )
        .child(lightbox(
            "single.viewer",
            &handle,
            &state,
            Vec::<gpui_kit::AnyElement>::new(),
            window,
            cx,
        ))
}
