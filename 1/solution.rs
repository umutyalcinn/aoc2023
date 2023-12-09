use std::fs;

struct LineReader<'a> {
    line_str: &'a str,
    current_pos: usize,
    current_read: usize,
}

impl LineReader<'_>{
    fn new(line: &str) -> LineReader {
        LineReader{
            line_str: line,
            current_pos: 0,
            current_read: 0,
        }
    }

    fn readChar(&mut self) -> Option<char> {
        if(self.current_read < str::len(self.line_str)){
            let c = self.line_str.as_bytes()[self.current_read] as char;
            self.current_read += 1;
            return Some(c);
        }
        else{
            return None;
        }
    } 

    pub fn nextNumber(&mut self) -> Option<u32> {
        loop {
            if let Some(c) = self.readChar(){
                if let Some(n) = c.to_digit(10){
                    self.current_pos += 1; 
                    self.current_read = self.current_pos; 
                    return Some(n);
                }
                else{
                    println!("trying to read number");
                    if let Some(n) = self.readNumber(c){
                        println!("found number");
                        self.current_pos += 1;
                        self.current_read = self.current_pos; 
                        return Some(n);
                    }
                    else{
                        self.current_pos += 1;
                        self.current_read = self.current_pos; 
                    }
                }
            }
            else {
                return None;
            }
        }
    }

    fn readNumber(&mut self, char: char) -> Option<u32>{
        match char {
            'o' => {
                if (self.readChar() == Some('n') 
                    && self.readChar() == Some('e')) {
                    return Some(1);
                }
                else {
                    return None;
                }
            }
            't' => {

                match self.readChar() {
                    Some('w') => {
                        if self.readChar() == Some('o'){
                            return Some(2);
                        }
                        else {
                            return None;
                        }
                    }
                    Some('h') => {
                        if (self.readChar() == Some('r')
                        && self.readChar() == Some('e')
                        && self.readChar() == Some('e')){
                            return Some(3);
                        }
                        else {
                            return None;
                        }
                    }
                    _ => {
                        return None;
                    }
                }
            }
            'f' => {
                match self.readChar() {
                    Some('o') => {
                        if (self.readChar() == Some('u')
                            && self.readChar() == Some('r')){
                            return Some(4);
                        }
                        else {
                            return None;
                        }
                    }
                    Some('i') => {
                        if (self.readChar() == Some('v')
                        && self.readChar() == Some('e')){
                            return Some(5);
                        }
                        else {
                            return None;
                        }
                    }
                    _ => {
                        return None;
                    }
                }
            }
            's' => {
                match self.readChar() {
                    Some('i') => {
                        if self.readChar() == Some('x'){
                            return Some(6);
                        }
                        else {
                            return None;
                        }
                    }
                    Some('e') => {
                        if (self.readChar() == Some('v')
                        && self.readChar() == Some('e')
                        && self.readChar() == Some('n')){
                            return Some(7);
                        }
                        else {
                            return None;
                        }
                    }
                    _ => {
                        return None;
                    }
                }
            }
            'e' => {
                if (self.readChar() == Some('i') 
                    && self.readChar() == Some('g')
                    && self.readChar() == Some('h')
                    && self.readChar() == Some('t')) {
                    return Some(8);
                }
                else {
                    return None;
                }
            }
            'n' => {
                if (self.readChar() == Some('i') 
                    && self.readChar() == Some('n')
                    && self.readChar() == Some('e')) {
                    return Some(9);
                }
                return None;
            }
            _ => {
                return None;
            }
        }
    }
}

fn main(){
    let input = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    for line in input.split("\n") {

        let mut lineReader = LineReader::new(line);

        let mut numbers: Vec<u32> = Vec::new();

        loop{
            if let Some(n) = lineReader.nextNumber() {
                numbers.push(n);
            }
            else{
                break;
            }
        }

        println!("{:?}", numbers);

        match numbers.len() {
            1 => {
                println!("line: {}, result: {}", line, numbers[0] + numbers[0] * 10);
                sum += numbers[0] + numbers[0] * 10;
            }
            2.. => {
                println!("line: {}, result: {}", line, numbers[0] * 10+ numbers[numbers.len() - 1]);
                sum += numbers[0] * 10 + numbers[numbers.len() - 1];
            }
            _ => {
                
            }
        }
    }

    println!("{sum}");
}
