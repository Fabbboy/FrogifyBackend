use regex::Regex;

pub(crate) fn isPwdValid(pwd: &String) -> bool {
    if pwd.len() < 8 {
        return false;
    }
    if pwd.len() > 32 {
        return false;
    }
    if pwd.contains(" ") {
        return false;
    }
    if pwd.contains("\t") {
        return false;
    }
    if pwd.contains("\r") {
        return false;
    }
    if pwd.contains("\n") {
        return false;
    }

    true
}

pub(crate) fn isMailValid(mail: &String) -> bool {
    let re = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    if ! re.is_match(& mail) {
        return false;
    }

    //now check if the email is a sbl email if not reject. it must start with e or et then 6 numbers then @sbl.ch or @edu.sbl.ch. for example: e256261@sbl.ch
    let re = Regex::new(r"^[eE][tT]?[0-9]{6}@(sbl\.ch|edu\.sbl\.ch)$").unwrap();
    if ! re.is_match(& mail) {
        return false;
    }

    true
}