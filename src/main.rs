mod lib;

// Example
fn main() {
    let mut m = lib::Message::new(lib::MessageType::Request(lib::RequestMethod::Register), String::from("my.dom.ru"));

    m.to(Some("Vladislav".to_string()), "1175".to_string())
    .from(Some("Vladislav".to_string()), "1175".to_string())
    .via("TCP".to_string(), "localhost".to_string(), "5060".to_string())
    .max_forwards("70".to_string())
    .cseq("1".to_string())
    .call_id("test".to_string())
    .request_uri("1175".to_string());


    println!("{}", m.build_message());
    let mes = lib::Message::parse(&m.build_message());
    println!("\n{:?}", mes);
}