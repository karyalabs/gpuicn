//! The GIF picker catalog preview.

use crate::*;

/// A few sample pictures; an application asks a GIF service instead.
fn sample_gifs() -> Vec<Gif> {
    [
        ("Mountain lake", 10, 300, 200),
        ("City lights", 20, 200, 300),
        ("Forest path", 28, 300, 300),
        ("Ocean waves", 40, 300, 180),
        ("Desert road", 54, 200, 260),
        ("Snowy peaks", 64, 300, 220),
        ("Morning coffee", 75, 240, 300),
        ("Old bridge", 84, 300, 200),
    ]
    .into_iter()
    .map(|(title, picture, width, height)| {
        Gif::new(format!("{picture}"), title, width as f32, height as f32).image(format!(
            "https://picsum.photos/id/{picture}/{width}/{height}"
        ))
    })
    .collect()
}

impl Showcase {
    pub(crate) fn gif_picker_preview(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.entity().downgrade();
        let state = window
            .use_keyed_state("gif-picker.state", cx, |window, cx| {
                let state = cx.new(|cx| {
                    let mut state = GifPickerState::new(window, cx);
                    state.set_attribution(Some("Sample pictures".into()), cx);
                    state.set_gifs(sample_gifs(), false, cx);
                    state
                });
                let subscription = cx.subscribe(&state, move |_, state, event, cx| {
                    let action = match event {
                        // The application searches its GIF service; here the
                        // samples are filtered by title.
                        GifPickerEvent::Search(query) => {
                            let query = query.to_lowercase();
                            let found = sample_gifs()
                                .into_iter()
                                .filter(|gif| gif.title.to_lowercase().contains(&query));
                            state.update(cx, |state, cx| state.set_gifs(found, false, cx));
                            return;
                        }
                        GifPickerEvent::LoadMore => return,
                        GifPickerEvent::Pick(id) => format!("Picked GIF {id}"),
                        GifPickerEvent::Cancel => "Closed".into(),
                    };
                    let _ = owner.update(cx, |this, cx| {
                        this.demo_action = action;
                        cx.notify();
                    });
                });
                (state, subscription)
            })
            .read(cx)
            .0
            .clone();
        GifPicker::new(&state)
    }
}

pub(crate) fn preview(view: &Showcase, window: &mut Window, cx: &mut Context<Showcase>) -> gpui_kit::AnyElement {
    view.gif_picker_preview(window, cx).into_any_element()
}
