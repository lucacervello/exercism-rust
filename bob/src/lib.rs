pub fn reply(question: &str) -> &str {

    if question.ends_with("?") {
        "Sure."
    } else if question == "" {
        "Fine. Be that way!"
    } else if question.to_uppercase() == question {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
