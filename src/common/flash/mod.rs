use maud::{Markup, html};
use poem::session::Session;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum Flash {
    Success { msg: String },
    Error { msg: String },
    Warning { msg: String },
}

impl Flash {
    pub fn as_html(&self) -> Markup {
        match self {
            Flash::Success { msg } => {
                html! {
                   div .flash-message .flash-message-success {
                       (msg)
                   }
                }
            }
            Flash::Error { msg } => {
                html! {
                   div .flash-message .flash-message-error {
                       (msg)
                   }
                }
            }
            Flash::Warning { msg } => {
                html! {
                   div .flash-message .flash-message-warning {
                       (msg)
                   }
                }
            }
        }
    }
}

pub trait FlashMessageHtml {
    fn flash_message_html(&self) -> Markup;
}

impl FlashMessageHtml for Option<Flash> {
    fn flash_message_html(&self) -> Markup {
        match self {
            None => {
                html! {}
            }
            Some(flash) => flash.as_html(),
        }
    }
}

pub trait FlashMessage {
    fn flash(&self, flash: Flash);

    fn get_flash(&self) -> Option<Flash>;
}

impl FlashMessage for Session {
    fn flash(&self, flash: Flash) {
        self.set("flash", flash)
    }

    fn get_flash(&self) -> Option<Flash> {
        let flash = self.get("flash");
        self.remove("flash");
        flash
    }
}
