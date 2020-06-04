// TODO DOCUMENTATION

#[derive(Debug)]
pub enum MessageType {
    Request(RequestMethod),
    Response(String),
}

#[derive(Debug)]
pub enum RequestMethod {
    Register,
    Invite,
    ACK,
    Cancel,
    Bye,
    Options,
}

// Remove pub's, add getters
#[derive(Debug)]
pub struct Message {
    pub mtype: MessageType, // Request/Response
    pub request_uri: String,

    // Mandatory for request headers:
    pub to: String,
    pub from: String,
    pub cseq: String,
    pub call_id: String,
    pub max_forwards: String,
    pub via: String,

    pub body: String,

    pub domain: String,
}

impl Message {
    pub fn new(mtype: MessageType, domain: String) -> Message {
        Message {
            mtype: mtype,
            request_uri: String::new(),
            to: String::new(),
            from: String::new(),
            cseq: String::new(),
            call_id: String::new(),
            max_forwards: String::new(),
            via: String::new(),
            body: String::new(),
            domain: domain,
        }
    }

    fn get_method_name(&mut self) -> Result<String, &'static str> {
        match &self.mtype {
            MessageType::Request(method) => {
                let method_str = match method {
                    RequestMethod::ACK => "ACK",
                    RequestMethod::Bye => "BYE",
                    RequestMethod::Cancel => "CANCEL",
                    RequestMethod::Invite => "INVITE",
                    RequestMethod::Options => "OPTIONS",
                    RequestMethod::Register => "REGISTER",
                };
                Ok(method_str.to_string())
            },
            MessageType::Response(_) => {
                Err("Incorrect message type.")
            }
        }
    }

    fn get_request_method_from_name(name: &str) -> Result<RequestMethod, &'static str> {
        let method = match name {
            "ACK" => RequestMethod::ACK,
            "BYE" => RequestMethod::Bye,
            "CANCEL" => RequestMethod::Cancel,
            "INVITE" => RequestMethod::Invite,
            "OPTIONS" => RequestMethod::Options,
            "REGISTER" => RequestMethod::Register,
            _ => return Err("Unknown method name.")
        };
        Ok(method)
    }

    // need to change implementation: make struct for each header:
    // for each implement value, build string to avoid this spaghetti
    // also, change branch
    pub fn via(&mut self, proto: String, host: String, port: String) -> &mut Message {
        self.via = format!("Via: SIP/2.0/{} {}:{};branch=z9hG4bK\r\n", proto, host, port);
        self
    }

    // add tag
    pub fn to(&mut self, display_name: String, ext: String) -> &mut Message {
        if display_name.is_empty() {
            self.to = format!("To: sip:{}@{}\r\n", ext, self.domain);
        } else {
            self.to = format!("To: {} <sip:{}@{}>\r\n", display_name, ext, self.domain);
        }
        self
    }

    // add tag
    pub fn from(&mut self, display_name: String, ext: String) -> &mut Message {
        if display_name.is_empty() {
            self.from = format!("From: sip:{}@{}\r\n", ext, self.domain);
        } else {
            self.from = format!("From: {} <sip:{}@{}>\r\n", display_name, ext, self.domain);
        }
        self
    }

    pub fn call_id(&mut self, call_id: String) -> &mut Message {
        self.call_id = format!("Call-ID: {}\r\n", call_id);
        self
    }

    pub fn cseq(&mut self, number: String) -> &mut Message {
        self.cseq = format!("CSeq: {} {}\r\n", number, self.get_method_name().unwrap());
        self
    }

    pub fn max_forwards(&mut self, number: String) -> &mut Message {
        self.max_forwards = format!("Max-Forwards: {}\r\n", number);
        self
    }

    pub fn request_uri(&mut self, uri: String) -> &mut Message {
        self.request_uri = uri;
        self
    }

    pub fn build_message(&mut self) -> String {
        let start_line = match &self.mtype {
            MessageType::Request(_) => {
                let method_name = self.get_method_name().unwrap();
                format!("{} sip:{}@{} SIP/2.0\r\n", method_name, self.request_uri, self.domain)
            },
            MessageType::Response(response) => {
                format!("SIP/2.0 {}", response)
            }
        };
        let content_length = match &self.body.is_empty() {
            true => String::from("Content-Length: 0\r\n\r\n"),
            false => format!("Content-Length: {}\r\n\r\n", &self.body.len()),
        };
        format!("{}{}{}{}{}{}{}{}{}", start_line, self.via, self.to, self.from, self.call_id, self.cseq, self.max_forwards, content_length, self.body)
    }

    pub fn parse(msg: String) -> Message {
        let msg_split = msg.split("\r\n").collect::<Vec<_>>();

        let msg_head = msg_split[0].split(" ").collect::<Vec<_>>();

        let message_type = match msg_head[0].starts_with("SIP") {
            true => MessageType::Response(format!("{} {}", msg_head[0], msg_head[1])),
            false => MessageType::Request(Message::get_request_method_from_name(msg_head[0]).unwrap())
        };

        let mut message = Message::new(message_type, String::new());

        match message.mtype {
            MessageType::Request(_) => message.request_uri = msg_head[1].chars().skip_while(|c| c != &':').skip(1).take_while(|c| c != &'@').collect(),
            _ => ()
        };

        // Need to refactor
        for i in 1..msg_split.len() {
            if msg_split[i].starts_with("Via:") {
                message.via = String::from(msg_split[i]);
            } else if msg_split[i].starts_with("To:") {
                message.to = String::from(msg_split[i]);
            } else if msg_split[i].starts_with("From:") {
                message.from = String::from(msg_split[i]);
            } else if msg_split[i].starts_with("Call-ID:") {
                message.call_id = String::from(msg_split[i]);
            } else if msg_split[i].starts_with("CSeq:") {
                message.cseq = String::from(msg_split[i]);
            } else if msg_split[i].starts_with("Max-Forwards:") {
                message.max_forwards = String::from(msg_split[i]);
            } else if msg_split[i].starts_with("Content-Length:") {
                // do smth
            } else { // body
                message.body = String::from(msg_split[i]);
            }
        }
        message
    }
}

// TODO TESTS!!!
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
