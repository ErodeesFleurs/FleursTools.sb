// static toReplace(palette: { [index: string]: string }, omitReplace = false): string {
//     let s = omitReplace ? '' : '?replace';

//     Object.keys(palette).forEach(key => {
//       s += `;${key}=${palette[key]}`;
//     });

//     return s;
//   }

use std::collections::HashMap;

pub fn to_replace(palette: HashMap<String, String>, omit_replace: bool) -> String {
    let mut s = if omit_replace {
        "".to_string()
    } else {
        "?replace".to_string()
    };

    for (key, value) in palette {
        s += &format!(";{}={}", key, value);
    }

    s
}
