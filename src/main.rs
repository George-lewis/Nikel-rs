mod client;

use std::io::Write;

fn main() {
    let client = client::NikelAPI::new();
    loop {
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input == "\n" {
            continue;
        }
        let split = input.split(" ").collect::<Vec<&str>>();
        let mut map = std::collections::HashMap::<&str, &str>::new();
        for kv in split[1].split(",") {
            let spl = kv.split(":").collect::<Vec<&str>>();
            let k = spl[0];
            let v = spl[1];
            map.insert(k, v);
        }
        let mut out: String;
        match split[0] {
            "courses" | "classes" => {
                let x = client.courses(map).unwrap();
                out = x.response.iter().map(|e| format!("{}|{}: {}", e.code.as_ref().unwrap(), e.campus.as_ref().unwrap(), e.description.as_ref().unwrap())).collect::<Vec<String>>().join("\n-----\n");
            },
            "textbooks" | "tb" => {
                let x = client.textbooks(map).unwrap();
                out = x.response.iter().map(|e| format!("{}|{}: ${}", e.title.as_ref().unwrap(), e.author.as_ref().unwrap(), e.price.as_ref().unwrap())).collect::<Vec<String>>().join("\n-----\n");
            }
            _ => unreachable!()
        }
        // let out = x.response.iter().map(|e| format!("{}|{}: {}", e.code.as_ref().unwrap(), e.campus.as_ref().unwrap(), e.description.as_ref().unwrap())).collect::<Vec<String>>().join("\n-----\n");
        println!("==========\n{}\n==========", out);
    }
}
