pub fn reply(question: String) -> String {

    let str = String::new();

    if question.ends_with("?") {
        str + "Sure."
    } else {
        str + "Whatever."
    }
}
