use std::string::String;

pub fn apply_rule(token: &str) -> String {
    let len_token = token.len();
    let mut ret = String::new();

    if token.ends_with("s") {
        if token.ends_with("ies") && len_token > 7 {
            if token.ends_with("cies") {
                let slice = &token[..len_token - 4];
                ret = slice.to_owned() + "cy";

                return ret
            }
            if token.ends_with("ries") {
                let slice = &token[..len_token - 4];
                ret = slice.to_owned() + "ry";

                return ret
            }
            if token.ends_with("ties") {
                let slice = &token[..len_token - 4];
                ret = slice.to_owned() + "ty";

                return ret
            }
        }
        if token.ends_with("doms") {
            let slice = &token[..len_token - 4];
            ret = slice.to_owned() + "dom";

            return ret
        }
        if token.ends_with("esses") {
            let slice = &token[..len_token - 5];
            ret = slice.to_owned() + "ess";

            return ret
        }
        if token.ends_with("isms") {
            let slice = &token[..len_token - 4];
            ret = slice.to_owned() + "ism";

            return ret
        }
        if token.ends_with("ists") {
            let slice = &token[..len_token - 4];
            ret = slice.to_owned() + "ist";

            return ret
        }
        if token.ends_with("ments") {
            let slice = &token[..len_token - 5];
            ret = slice.to_owned() + "ments";

            return ret
        }
        if token.ends_with("nces") {
            let slice = &token[..len_token - 4];
            ret = slice.to_owned() + "nce";

            return ret
        }
        if token.ends_with("ships") {
            let slice = &token[..len_token - 5];
            ret = slice.to_owned() + "ship";

            return ret
        }
        if token.ends_with("tions") {
            let slice = &token[..len_token - 5];
            ret = slice.to_owned() + "tion";

            return ret
        }
        if token.ends_with("tions") {
            let slice = &token[..len_token - 5];
            ret = slice.to_owned() + "tion";

            return ret
        }
    }

    if token.ends_with("ed") {
        if token.ends_with("ated") {
            let slice = &token[..len_token - 4];
            ret = slice.to_owned() + "ate";

            return ret
        }
        if token.ends_with("ened") {
            let slice = &token[..len_token - 4];
            ret = slice.to_owned() + "en";

            return ret
        }
        if token.ends_with("fied") {
            let slice = &token[..len_token - 4];
            ret = slice.to_owned() + "fy";

            return ret
        }
        if token.ends_with("ized") {
            let slice = &token[..len_token - 4];
            ret = slice.to_owned() + "ize";

            return ret
        }
    }

    let slice = &token[..len_token - 1];

    return slice.to_owned()
}