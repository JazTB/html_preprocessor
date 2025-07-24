pub fn parse_data<F>(
    data: &str,
    start_sym: &str,
    end_sym: &str,
    mut cb: F
) -> String
where F: FnMut(&Vec<String>) -> String {
    let mut current_data = data;
    let mut idxes: Vec<[usize;2]> = Vec::new();
    // Find the position/size of all <!--@ ... @-->
    loop {
        if current_data.is_empty() { break; }
        match current_data.starts_with(start_sym) {
            false => {
                current_data = &current_data[1..];
                continue;
            }
            true => {
                let backup = current_data;
                let start = data.len() - current_data.len();

                if !loop {
                    if current_data.is_empty() {
                        current_data = backup;
                        break false;
                    }
                    if current_data.starts_with(end_sym) {
                        current_data = &current_data[end_sym.len()..];
                        break true;
                    }
                    current_data = &current_data[1..];
                } {
                    break;
                };

                let len = (data.len() - current_data.len()) - start;
                idxes.push([start, len]);
            }
        }
    };
    // Grab all the comments, parse them
    let mut parsed_comments: Vec<String> = Vec::new();
    for [start, len] in &idxes {
        let real_start = start + start_sym.len();
        let real_len = (len - start_sym.len()) - end_sym.len();
        let strings: Vec<String> = data[real_start..real_start+real_len]
            .trim()
            .split_whitespace()
            .map(|v| v.to_string())
            .collect();
        parsed_comments.push(cb(&strings));
    };
    parsed_comments.reverse();
    // Push all the sections (interleave normal/ppx) to a String
    let mut out_string = String::new();
    let mut idx: usize = 0;
    for [start, len] in &idxes {
        out_string.push_str(&data[idx..*start]);
        idx = start + len;
        let com = &parsed_comments.pop()
            .expect("Couldn't pop parsed comment (THIS SHOULDN'T HAPPEN)");
        out_string.push_str(com);
    };
    out_string.push_str(&data[idx..]);
    // Return it :)
    return out_string;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const DATA: &'static str =
        "<html>
            <body>
                <!--@ This should get processed @-->
                <p> Something here! </p>
                <!--@ This should too! @-->
                <!--@ This shouldn't get touched -->
                <!-- This is a comment! -->
            </body>
        </html>";

    #[test]
    fn parser() {
        let stuff = parse_data(DATA, "<!--@", "@-->", |strs| {
            format!("<!-- Contained: {} -->", strs.join(" "))
        });
        eprintln!("{}", stuff)
    }
}
