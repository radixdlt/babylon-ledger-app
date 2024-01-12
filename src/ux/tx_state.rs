
mod stax {

}
mod non_stax {
    use crate::ui::multiline_scroller::MultilineMessageScroller;
    use crate::ui::multipage_validator::MultipageValidator;
    use crate::ui::single_message::SingleMessage;
    use crate::ui::utils;

    pub fn display_max_fee(text: &[u8]) {
        MultilineMessageScroller::with_title(
            "Max TX Fee:",
            core::str::from_utf8(text).unwrap(),
            true,
        )
            .event_loop();
    }
}