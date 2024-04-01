#[cfg(not(target_os = "stax"))]
pub mod main_menu {
    use crate::settings::Settings;
    use crate::ui::menu::{Menu, MenuFeature, MenuItem};
    use crate::ui::multipage_validator::MultipageValidator;
    use crate::ui::single_message::SingleMessage;
    use crate::ui::utils::{BACK_ICON, RADIX_LOGO_ICON};
    use ledger_device_sdk::ui::bagls::{
        CERTIFICATE_ICON, COGGLE_ICON, DASHBOARD_X_ICON, PROCESSING_ICON,
    };
    use ledger_device_sdk::ui::gadgets::clear_screen;
    const APPLICATION: &str = env!("CARGO_PKG_DESCRIPTION");
    const APPLICATION_ABOUT: &str = concat!(
        env!("CARGO_PKG_DESCRIPTION"),
        "\n(c) 2022-23\nRDX Works Ltd."
    );
    const APPLICATION_VERSION: &str = concat!("\n", env!("CARGO_PKG_VERSION"), "\n",);

    fn app_menu_action() -> bool {
        false
    }

    fn version_menu_action() -> bool {
        clear_screen();
        SingleMessage::new(APPLICATION_VERSION).show_and_wait();
        false
    }

    fn get_verbose_mode_state() -> bool {
        Settings::get().verbose_mode
    }

    fn get_blind_signing_state() -> bool {
        Settings::get().blind_signing
    }

    fn settings_menu_action() -> bool {
        clear_screen();

        let menu = [
            MenuItem::new(
                MenuFeature::OnOffState(get_verbose_mode_state),
                "\nVerbose Mode",
                verbose_mode_setting_action,
            ),
            MenuItem::new(
                MenuFeature::OnOffState(get_blind_signing_state),
                "\nBlind Signing",
                blind_signing_setting_action,
            ),
            MenuItem::new(
                MenuFeature::Icon(&BACK_ICON),
                "\nBack",
                back_from_setting_action,
            ),
        ];

        Menu::new(menu).event_loop();

        false
    }

    fn verbose_mode_setting_action() -> bool {
        clear_screen();

        Settings {
            verbose_mode: MultipageValidator::new(
                &["Set Verbose", "Mode"],
                &["Enable"],
                &["Disable"],
            )
            .ask(),
            blind_signing: get_blind_signing_state(),
        }
        .update();

        true
    }

    fn blind_signing_setting_action() -> bool {
        clear_screen();

        Settings {
            verbose_mode: get_verbose_mode_state(),
            blind_signing: MultipageValidator::new(
                &["Set Blind", "Signing"],
                &["Enable"],
                &["Disable"],
            )
            .ask(),
        }
        .update();

        true
    }

    fn back_from_setting_action() -> bool {
        true
    }

    fn about_menu_action() -> bool {
        clear_screen();
        SingleMessage::new(APPLICATION_ABOUT).show_and_wait();
        false
    }

    fn quit_menu_action() -> bool {
        clear_screen();
        ledger_device_sdk::exit_app(0);
    }

    pub fn create() -> Menu<'static, 5> {
        let menu = [
            MenuItem::new(
                MenuFeature::Icon(&RADIX_LOGO_ICON),
                "\nRadix Babylon",
                app_menu_action,
            ),
            MenuItem::new(
                MenuFeature::Icon(&PROCESSING_ICON),
                "\nVersion",
                version_menu_action,
            ),
            MenuItem::new(
                MenuFeature::Icon(&COGGLE_ICON),
                "\nSettings",
                settings_menu_action,
            ),
            MenuItem::new(
                MenuFeature::Icon(&CERTIFICATE_ICON),
                "\nAbout",
                about_menu_action,
            ),
            MenuItem::new(
                MenuFeature::Icon(&DASHBOARD_X_ICON),
                "\nQuit",
                quit_menu_action,
            ),
        ];

        Menu::new(menu)
    }
}
#[cfg(target_os = "stax")]
pub mod main_menu {}
