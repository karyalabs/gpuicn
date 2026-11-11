//! Attachment examples.

use crate::*;

/// Files
///
/// A file chip shows a name, an icon and a line of details.
pub(crate) fn files(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_wrap()
        .justify_center()
        .gap(px(8.))
        .child(
            Attachment::new("files.pdf", "invoice.pdf")
                .icon(LucideIcon::FileText)
                .meta("PDF · 180 KB"),
        )
        .child(
            Attachment::new("files.sheet", "budget.xlsx")
                .icon(LucideIcon::FileSpreadsheet)
                .meta("Spreadsheet · 42 KB"),
        )
        .child(
            Attachment::new("files.zip", "photos.zip")
                .icon(LucideIcon::FileArchive)
                .meta("Archive · 12 MB"),
        )
}

/// Upload states
///
/// An upload shows its progress. A failed one offers a retry.
pub(crate) fn upload_states(
    _view: &Showcase,
    _window: &mut Window,
    _cx: &mut Context<Showcase>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_wrap()
        .justify_center()
        .gap(px(8.))
        .child(
            Attachment::new("upload.running", "demo.mov")
                .icon(LucideIcon::FilePlay)
                .status(AttachmentStatus::Uploading(0.4))
                .on_remove(|_, _, _| {}),
        )
        .child(
            Attachment::new("upload.failed", "notes.txt")
                .status(AttachmentStatus::Failed)
                .on_retry(|_, _, _| {})
                .on_remove(|_, _, _| {}),
        )
}
