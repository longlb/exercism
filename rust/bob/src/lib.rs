pub fn reply(message: &str) -> &str {
    let message_n = message.trim();
    let mut final_ans = message_n;
    println!("{}", message_n);
    if message_n != "" {
        if message_n.to_ascii_uppercase() == message_n
            && message_n.chars().any(|x| x.is_alphabetic())
        {
            final_ans = "CAPS";
        }
        if message_n.ends_with("?") {
            final_ans = match final_ans {
                "CAPS" => "?CAPS",
                _ => "?",
            };
        }
    }
    println!("{}", message_n);

    match final_ans {
        "?" => "Sure.",
        "CAPS" => "Whoa, chill out!",
        "?CAPS" => "Calm down, I know what I'm doing!",
        "" => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
