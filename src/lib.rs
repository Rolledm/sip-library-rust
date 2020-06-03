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

#[derive(Debug)]
pub struct Message {
    mtype: MessageType, // Request/Response
    request_uri: String,

    // Mandatory for request headers:
    to: String,
    from: String,
    cseq: String,
    call_id: String,
    max_forwards: String,
    via: String,

    body: String,

    domain: String,
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
        println!("{:?}", msg_split);

        let message_type = match msg_split[0].starts_with("SIP") {
            true => MessageType::Response()
        }

        //let message = Message::new(mtype: MessageType, domain: String);
        Message::new(MessageType::Response("200 OK".to_string()), String::new())
    }
}


/*
"BYE sip:alice@client.atlanta.example.com SIP/2.0\r
Via: SIP/2.0/TCP client.chicago.example.com:5060;branch=z9hG4bKfgaw2\r
Max-Forwards: 70\r
Route: <sip:ss3.chicago.example.com;lr>\r
From: Bob <sip:bob@biloxi.example.com>;tag=314159\r
To: Alice <sip:alice@atlanta.example.com>;tag=9fxced76sl\r
Call-ID: 2xTb9vxSit55XU7p8@atlanta.example.com\r
CSeq: 1 BYE\r
Content-Length: 0\r\n\r\n";
*/

// TODO TESTS!!!
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
