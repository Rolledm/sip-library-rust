
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
    cceq: String,
    call_id: String,
    max_forwards: String,
    via: String,

    body: String,
}

impl Message {
    pub fn new(mtype: MessageType) -> Message {
        Message {
            mtype: mtype,
            request_uri: String::new(),
            to: String::new(),
            from: String::new(),
            cceq: String::new(),
            call_id: String::new(),
            max_forwards: String::new(),
            via: String::new(),
            body: String::new(),
        }
    }

    pub fn to(&mut self, to: String) -> &mut Message {
        self.to = to;
        self
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
                Ok(format!("{} sip:{}@atlanta.example.com SIP/2.0\r\n", method_str, self.to))
            },
            MessageType::Response(_) => {
                Err("Incorrect message type.")
            }
        }
    }

    pub fn build_message(&mut self) -> String {
        let start_line = match &self.mtype {
            MessageType::Request(method) => {
                let method_name = self.get_method_name().unwrap();
                format!("{} sip:{}@atlanta.example.com SIP/2.0\r\n", method_name, self.to)
            },
            MessageType::Response(response) => {
                format!("SIP/2.0 {}", response)
            }
        };
        format!("{}Via: SIP/2.0/TCP client.chicago.example.com:5060;branch=z9hG4bKfgaw2\r
Max-Forwards: 70\r
Route: <sip:ss3.chicago.example.com;lr>\r
From: Bob <sip:bob@biloxi.example.com>;tag=314159\r
To: Alice <sip:alice@atlanta.example.com>;tag=9fxced76sl\r
Call-ID: 2xTb9vxSit55XU7p8@atlanta.example.com\r
CSeq: 1 BYE\r
Content-Length: 0\r\n\r\n", start_line)
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
