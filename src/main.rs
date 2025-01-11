mod back;
mod chest;
mod directives;
mod hat;
mod image;
mod pants;
mod template;

fn main() {
    let generate_type = std::env::args().nth(1).expect("No type provided");
    match generate_type.as_str() {
        "pants" => {
            let path = std::env::args().nth(2).expect("No path provided");
            let hide_body = std::env::args()
                .nth(3)
                .expect("No hide_body provided")
                .parse()
                .expect("Failed to parse hide_body");
            let res = pants::generate_pants(path, hide_body);
            print!("{}", res);
        }
        "chest" => {
            let chest_path = std::env::args().nth(2).expect("No chest_path provided");
            let front_sleeve_path = std::env::args()
                .nth(3)
                .expect("No front_sleeve_path provided");
            let back_sleeve_path = std::env::args()
                .nth(4)
                .expect("No back_sleeve_path provided");
            let res = chest::generate_chest(chest_path, front_sleeve_path, back_sleeve_path);
            print!("{}", res);
        }
        "back" => {
            let back_path = std::env::args().nth(2).expect("No back_path provided");
            let res = back::generate_back(back_path);
            print!("{}", res);
        }
        "hat" => {
            let hat_path = std::env::args().nth(2).expect("No hat_path provided");
            let res = hat::generate_hat(hat_path);
            print!("{}", res);
        }
        _ => {
            eprintln!("Unknown type: {}", generate_type);
            std::process::exit(1);
        }
    }
}
