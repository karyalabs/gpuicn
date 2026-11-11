//! Form examples.

use crate::*;

/// Sign in
///
/// A form stacks its fields and its submit button with even spacing.
pub(crate) fn sign_in(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let email = demo_input("signin.email", "", "you@example.com", window, cx);
    let password = demo_input("signin.password", "", "Password", window, cx);
    form("signin", cx)
        .w_full()
        .max_w(px(320.))
        .aria_label("Sign in")
        .child(Field::new("signin-email", &email).label("Email"))
        .child(Field::new("signin-password", &password).label("Password"))
        .child(Button::new("signin-submit").label("Sign in"))
}

/// With fieldsets
///
/// Group related fields in a fieldset inside the form.
pub(crate) fn with_fieldsets(
    _view: &Showcase,
    window: &mut Window,
    cx: &mut Context<Showcase>,
) -> impl IntoElement {
    let name = demo_input("profile.name", "Ada Lovelace", "Full name", window, cx);
    let email = demo_input("profile.email", "ada@example.com", "Email", window, cx);
    let city = demo_input("profile.city", "", "London", window, cx);
    form("profile", cx)
        .w_full()
        .max_w(px(320.))
        .aria_label("Profile")
        .child(
            fieldset("profile.contact", cx)
                .aria_label("Contact")
                .child(fieldset_legend(FieldsetLegendVariant::Legend, cx).child("Contact"))
                .child(Field::new("profile-name", &name).label("Full name"))
                .child(Field::new("profile-email", &email).label("Email")),
        )
        .child(
            fieldset("profile.address", cx)
                .aria_label("Address")
                .child(fieldset_legend(FieldsetLegendVariant::Legend, cx).child("Address"))
                .child(Field::new("profile-city", &city).label("City")),
        )
        .child(Button::new("profile-submit").label("Save profile"))
}
