//! The menu catalog preview.

use crate::*;

impl Showcase {
    pub(crate) fn menu_preview(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sign_out = cx.listener(|this, _, _, cx| {
            this.demo_action = "Signed out".into();
            cx.notify();
        });
        let menu = window.use_keyed_state("account-menu", cx, |_, cx| {
            MenuState::new(
                [
                    MenuItem::label("account-heading", "My account"),
                    MenuItem::new("profile", "Profile"),
                    MenuItem::new("notifications", "Notifications").checked(true),
                    MenuItem::separator(),
                    MenuItem::submenu(
                        "invite",
                        "Invite",
                        [
                            MenuItem::new("email", "Email"),
                            MenuItem::new("message", "Message"),
                        ],
                    ),
                    MenuItem::new("billing", "Billing").disabled(true),
                    MenuItem::new("sign-out", "Sign out").on_click(sign_out),
                ],
                cx,
            )
        });
        let teams = window.use_keyed_state("team-menu", cx, |_, cx| {
            let pill = |count: &'static str| {
                move |_: &mut Window, cx: &mut App| {
                    let t = UiTheme::read(cx);
                    div()
                        .px(t.space(1.5))
                        .rounded_full()
                        .bg(t.colors.primary)
                        .text_color(t.colors.primary_foreground)
                        .text_size(t.text(11.))
                        .child(count)
                        .into_any_element()
                }
            };
            MenuState::new(
                [
                    MenuItem::label("teams-heading", "TEAMS ON KARYA.TEAM"),
                    MenuItem::new("tech", "Tech")
                        .icon(LucideIcon::Users)
                        .shortcut("secondary-alt-1")
                        .radio(true),
                    MenuItem::new("gpui", "Gpui")
                        .icon(LucideIcon::Users)
                        .trailing(pill("3"))
                        .shortcut("secondary-alt-2")
                        .radio(false),
                    MenuItem::new("join", "Join another team…").icon(LucideIcon::Plus),
                    MenuItem::separator(),
                    MenuItem::label("servers-heading", "OTHER SERVERS"),
                    MenuItem::new("oss", "Open source")
                        .icon(LucideIcon::Server)
                        .description("mattermost.example")
                        .shortcut("secondary-2"),
                    MenuItem::new("add", "Add server…").icon(LucideIcon::Plus),
                    MenuItem::separator(),
                    MenuItem::new("manage", "Manage servers…")
                        .icon(LucideIcon::Settings)
                        .shortcut("Settings"),
                ],
                cx,
            )
            .width(px(300.))
        });
        div()
            .flex()
            .gap_4()
            .child(Menu::new(&menu, "My account"))
            .child(Menu::new(&teams, "Switch team"))
    }
}

pub(crate) fn preview(view: &Showcase, window: &mut Window, cx: &mut Context<Showcase>) -> gpui_kit::AnyElement {
    view.menu_preview(window, cx).into_any_element()
}
