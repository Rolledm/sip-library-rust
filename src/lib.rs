// TODO DOCUMENTATION

// TODO divide Message into traits

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
    Subscribe,
    Notify,
    Publish,
}

// TODO Remove pub's, add getters
#[derive(Debug)]
pub struct Message {
    pub mtype: MessageType, // Request/Response
    pub request_uri: String,
    pub domain: String,

    // Mandatory for request headers:
    pub to: String,
    pub from: String,
    pub cseq: String,
    pub call_id: String,
    pub contact: String,
    pub max_forwards: String,
    pub via: String,

    pub event: String,
    pub accept: String,

    pub content_type: String,
    pub body: String,
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
            contact: String::new(),
            max_forwards: String::new(),
            via: String::new(),
            body: String::new(),
            event: String::new(),
            accept: String::new(),
            content_type: String::new(),
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
                    RequestMethod::Subscribe => "SUBSCRIBE",
                    RequestMethod::Notify => "NOTIFY",
                    RequestMethod::Publish => "PUBLISH",
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
            "SUBSCRIBE" => RequestMethod::Subscribe,
            "NOTIFY" => RequestMethod::Notify,
            "PUBLISH" => RequestMethod::Publish,
            _ => return Err("Unknown method name.")
        };
        Ok(method)
    }

    // TODO change implementation: make struct for each header:
    // for each implement value, build string to avoid this spaghetti
    // also, change branch
    pub fn via(&mut self, proto: String, host: String, port: String) -> &mut Message {
        self.via = format!("Via: SIP/2.0/{} {}:{};branch=z9hG4bK\r\n", proto, host, port);
        self
    }

    pub fn to(&mut self, display_name: Option<String>, ext: String) -> &mut Message {
        match display_name {
            None => self.to = format!("To: sip:{}@{}\r\n", ext, self.domain),
            Some(name) => self.to = format!("To: {} <sip:{}@{}>\r\n", name, ext, self.domain)
        };
        self
    }

    pub fn get_to(&self) -> String {
        self.to.chars().skip_while(|c| c != &':').skip(1).skip_while(|c| c != &':').skip(1).take_while(|c| c != &'@').collect()
    }

    pub fn from(&mut self, display_name: Option<String>, ext: String) -> &mut Message {
        match display_name {
            None => self.from = format!("From: sip:{}@{}\r\n", ext, self.domain),
            Some(name) => self.from = format!("From: {} <sip:{}@{}>\r\n", name, ext, self.domain)
        };
        self
    }

    pub fn get_from(&self) -> String {
        self.from.chars().skip_while(|c| c != &':').skip(1).skip_while(|c| c != &':').skip(1).take_while(|c| c != &'@').collect()
    }

    pub fn call_id(&mut self, call_id: String) -> &mut Message {
        self.call_id = format!("Call-ID: {}\r\n", call_id);
        self
    }

    pub fn cseq(&mut self, number: String) -> &mut Message {
        self.cseq = format!("CSeq: {} {}\r\n", number, self.get_method_name().unwrap());
        self
    }

    pub fn event(&mut self, event: String) -> &mut Message {
        self.event = format!("Event: {}\r\n", event);
        self
    }

    pub fn accept(&mut self, accept: String) -> &mut Message {
        self.cseq = format!("Accept: {}\r\n", accept);
        self
    }

    pub fn content_type(&mut self, content_type: String) -> &mut Message {
        self.content_type = format!("Content-Type: {}\r\n", content_type);
        self
    }

    pub fn contact(&mut self, ext: String) -> &mut Message {
        self.to = format!("Contact: <sip:{}@{}>\r\n", ext, self.domain);
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
                format!("SIP/2.0 {}\r\n", response)
            }
        };
        let content_length = match &self.body.is_empty() {
            true => String::from("Content-Length: 0\r\n\r\n"),
            false => format!("Content-Length: {}\r\n\r\n", &self.body.len()),
        };
        format!("{}{}{}{}{}{}{}{}{}{}{}{}{}", start_line, 
                                            self.via, 
                                            self.to, 
                                            self.from, 
                                            self.call_id, 
                                            self.cseq, 
                                            self.max_forwards, 
                                            self.event, 
                                            self.accept, 
                                            self.contact,
                                            self.content_type,
                                            content_length, 
                                            self.body)
    }

    pub fn parse(msg: &str) -> Message {
        let msg_split = msg.split("\r\n").collect::<Vec<_>>();

        let msg_head = msg_split[0].split(" ").collect::<Vec<_>>();

        let message_type = match msg_head[0].starts_with("SIP") {
            true => MessageType::Response(format!("{} {}", msg_head[1], msg_head[2])),
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
                message.via = format!("{}\r\n", msg_split[i]);
            } else if msg_split[i].starts_with("To:") {
                message.to = format!("{}\r\n", msg_split[i]);
            } else if msg_split[i].starts_with("From:") {
                message.from = format!("{}\r\n", msg_split[i]);
            } else if msg_split[i].starts_with("Call-ID:") {
                message.call_id = format!("{}\r\n", msg_split[i]);
            } else if msg_split[i].starts_with("CSeq:") {
                message.cseq = format!("{}\r\n", msg_split[i]);
            } else if msg_split[i].starts_with("Max-Forwards:") {
                message.max_forwards = format!("{}\r\n", msg_split[i]);
            } else if msg_split[i].starts_with("Content-Length:") {
                // do smth
            } else { // body
                message.body = format!("{}\r\n", msg_split[i]);
            }
        }
        message
    }
}

// TODO more tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_method_name() {
        let mut message = Message::new(MessageType::Request(RequestMethod::Register), String::from("my.dom.ru"));
        assert_eq!(message.get_method_name().unwrap(), String::from("REGISTER"));
    }

    #[test]
    fn check_to() {
        let mut message = Message::new(MessageType::Request(RequestMethod::Register), String::from("my.dom.ru"));
        message.to(Some(String::from("name")), String::from("1175"));
        assert_eq!(message.get_to(), String::from("1175"));
    }

    #[test]
    fn check_from() {
        let mut message = Message::new(MessageType::Request(RequestMethod::Register), String::from("my.dom.ru"));
        message.from(Some(String::from("name")), String::from("1176"));
        assert_eq!(message.get_from(), String::from("1176"));
    }

    #[test]
    fn check_to_without_dname() {
        let mut message = Message::new(MessageType::Request(RequestMethod::Register), String::from("my.dom.ru"));
        message.to(None, String::from("1175"));
        assert_eq!(message.get_to(), String::from("1175"));
    }

    #[test]
    fn check_from_without_dname() {
        let mut message = Message::new(MessageType::Request(RequestMethod::Register), String::from("my.dom.ru"));
        message.from(None, String::from("1176"));
        assert_eq!(message.get_from(), String::from("1176"));
    }
}
