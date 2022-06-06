fn main() {

    enum IpAddres {
        v4,
        v6,
    }

    // let four = IpAddresVer::v4;
    // let six = IpAddresVer::v6;

    struct Ip {
        version: IpAddres,
        address: String,
    }

    let home = Ip {
        version: IpAddres::v4,
        address: String::from("127.0.0.1"),
    };



    let m = Message::add_line("test");
    let tupl = (255,155,235);
    let c = Message::add_color(tupl);
    Message::Quit.method(true);
    // dbg!(Message::ChangeColor);
    println!("{:#?}{:#?}", m, c);


}

#[derive(Debug)]
enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
    Line(String),
}


impl Message {
    fn add_line(line: &str) -> Message {
        Message::Line(String::from(line))
    }
    fn add_color(color: (i32, i32, i32)) -> Message {
        Message::ChangeColor(color.0, color.1, color.2)
    }
}

impl Message {
    fn method(&self, quit: bool) -> Message {
        if quit == false {
            println!("{}", quit);
            Message::Quit
        } else {
            println!("{}", quit);
            Message::Quit
        }
    }
}
