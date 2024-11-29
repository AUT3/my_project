use std::cell::RefCell;

thread_local! {
    static CHAT_MSGS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[ic_cdk::update]
fn save_msg(m: String) {
    CHAT_MSGS.with(|msg| msg.borrow_mut().push(m));
}

#[ic_cdk::query]
fn get_chat() -> Vec<String> {
    CHAT_MSGS.with(|msgs| msgs.borrow().clone())
}

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
