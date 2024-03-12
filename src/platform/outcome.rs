pub struct OutCome {
    msg: String,
    tip: String,
}

impl OutCome {
    pub fn new(msg: &str, tip: &str) -> OutCome {
        OutCome {
            msg: msg.into(),
            tip: tip.into(),
        }
    }

    pub fn msg(&self) -> String {
        self.msg.clone()
    }

    pub fn tip(&self) -> String {
        self.tip.clone()
    }
}

