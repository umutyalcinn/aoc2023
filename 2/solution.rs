use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    records: Vec<Record>
}

#[derive(Debug)]
struct Record {
    colors: Vec<Color>
}

#[derive(Debug, Clone)]
enum Color{
    Red(u32),
    Green(u32),
    Blue(u32),
}

fn parse_line(line_str: &str) -> Game{
    let mut read_pos: usize = 5;

    let mut skip_whitespace = move || {
        loop {
            let c = line_str.as_bytes()[read_pos] as char;
            if(c == ' '){
                read_pos += 1;
            }
            else {
                break;
            }
        }
    };

    let mut read_color = move || -> Option<Color> {
        skip_whitespace();
        
        let mut count_string = String::from("");
        loop {
            let c = line_str.as_bytes()[read_pos] as char;
            if(c == ' '){
                break;
            }
            count_string.push(c);
            read_pos += 1;
        }

        println!("{count_string}");
        let count:u32 = count_string.parse().unwrap();

        skip_whitespace();

        let c = line_str.as_bytes()[read_pos] as char;
        match c {
            'r' => {
                read_pos += 3;
                return Some(Color::Red(count));
            }
            'g' => {
                read_pos += 5;
                return Some(Color::Green(count));
            }
            'b' => {
                read_pos += 4;
                return Some(Color::Blue(count));
            }
            _ => {
                return None;
            }
        }
    };

    let mut id_string = String::from(""); 
    loop {
        // satır id bul
        let c = line_str.as_bytes()[read_pos] as char;
        if(c == ':'){
            read_pos += 1;
            break;
        }
        else {
            id_string.push(c);
            read_pos += 1;
        }
    }

    let id: u32 = id_string.parse().unwrap();

    //determine records

    let mut records: Vec<Record> = Vec::new();
    loop {

        //determine colors
        let mut colors: Vec<Color> = Vec::new();
        loop {
            let color = read_color();
            colors.push(color.unwrap());
            let c = line_str.as_bytes()[read_pos] as char;
            read_pos += 1;
            if(c == ';'){
                break;
            }
        }

        records.push(Record { colors: colors.clone() });
    }

    return Game {
        id,
        records,
    }
}

fn main(){
    let input = fs::read_to_string("input.txt").unwrap();

    for line in input.split("\n") {
        let game = parse_line(line);
        println!("{:?}", game);
    }
}
