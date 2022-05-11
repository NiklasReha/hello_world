use rand::distributions::{Distribution, Uniform};
use std::{thread, time};
use std::io::{Write};
use std::io::stdout;
use crossterm::{QueueableCommand};
use win32console::console::WinConsole;
use std::sync::mpsc;

fn main() {
    let _clean_up = CleanUp;
    let mut _best="y";
    let mut _h=String::new();
    let mut containerarray:Vec<Cells>=Vec::new();
    let mut hoehe:i32 = get_height(); 
    let mut weite:i32 = get_width();
    let iteration:i32= get_iterations();
    let sign:String=get_sign();
    let ten_millis = time::Duration::from_millis(get_speed());
    

    let mut rng = rand::thread_rng();
    let gen = Uniform::from(0..2);
    
    for x in 0..hoehe{
        for d in 0..weite{
            let cell=Cells{neighbors: 0,pos_x:(d as usize) as usize,pos_y:(x as usize) as usize,status:gen.sample(&mut rng),vertical_value:0 };
            containerarray.push(cell);
        }
    }
    weite-=1;
    hoehe-=1;


    let mut drip=containerarray.clone();
    WinConsole::output().clear().expect("Irgendwas lief falsch");
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move|| {
        let mut stdout = stdout();
        stdout.queue(crossterm::cursor::Hide).expect("Irgendwas lief falsch");
        for u in 0..iteration+1{
            let result:Vec<Cells>=receiver.recv().unwrap();
            let mut ausgabe=String::new();
            ausgabe+="   ";
            for _x in 0..weite-1{
                ausgabe+="___";
            }
            ausgabe+=" \n";
            for x in 0..hoehe{
                ausgabe+="  |";
                for d in 0..weite-1{
                    let zelle= Cells{..result[(x*weite+d) as usize]};
                    if zelle.status == 1{
                    ausgabe+=" ";
                    ausgabe+=&sign;
                    ausgabe+=" ";
                    }
                    else{
                        ausgabe+="   ";
                    }
                }
                ausgabe+="|\n";
            }
            ausgabe+="  |";

            for _x in 0..weite-1{
                ausgabe+="___";
            }

            ausgabe+="|\n";
            ausgabe+="  Iterationen: ";
            ausgabe+=&u.to_string();
            ausgabe+="\\";
            ausgabe+=&iteration.to_string();
            ausgabe+="\n";
            stdout.write_all(ausgabe.to_string().as_bytes()).expect("Irgendwas lief falsch");
            thread::sleep(ten_millis);
            }
    });

    let mut stdout = stdout();
    for _u in 0..iteration+1{
        let mut temp:Vec<Cells>=Vec::new();

        for x in 0..hoehe+1{
            for d in 0..weite+1{
                let mut zelle= Cells{..drip[((x*weite)+d)as usize]};                
                zelle.get_vertical_value(drip.clone(),hoehe as usize,weite as usize);
                temp.push(Cells{neighbors : 0, pos_y : x as usize, pos_x : d as usize, ..zelle});
            }
        }

        containerarray=temp.clone();
        temp=Vec::new();
        stdout.queue(crossterm::cursor::MoveTo(0,0)).expect("Irgendwas lief falsch");

        for x in 0..hoehe+1{
            for d in 0..weite+1{
                let mut zelle= Cells{..containerarray[(x*weite+d)as usize]};
                zelle.get_neighbors(containerarray.clone(),weite as usize);
                zelle.update_status();
                temp.push(Cells{neighbors : 0, pos_y : x as usize, pos_x : d as usize, ..zelle});
            }
        }
        sender.send(temp.clone()).unwrap();
        containerarray=temp.clone();
        drip =containerarray.clone();
    }
    stdout.queue(crossterm::cursor::Show).expect("Irgendwas lief falsch");
    println!("Simulation beendet! Drücke ENTER");
    let _b1 = std::io::stdin().read_line(&mut _h).unwrap();
    WinConsole::output().clear().expect("Irgendwas lief falsch");
}

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        let mut stdout=std::io::stdout();
        stdout.queue(crossterm::cursor::Show).expect("Irgendwas lief falsch");
        WinConsole::output().clear().expect("Irgendwas lief falsch");
    }
}

pub fn get_sign()->String{
    loop{
        let mut sign=String::new();
        println!("Darstellungszeichen der Zellen: ");
        let _b1 = std::io::stdin().read_line(&mut sign).unwrap();
        let sign = sign.trim_end();
        if sign.chars().count() == 1{
            return sign.to_string();
        }
        else{
            println!("Bitte gib nur ein Zeichen ein!")
        }
    }
}

pub fn get_speed()->u64{
    loop{
        let mut speed=String::new();
        println!("Darstellungsgeschwindigkeit der Iterationen: ");
        let _b1 = std::io::stdin().read_line(&mut speed).unwrap();
        let speed = speed.trim_end();
        if check_numeric(speed.to_string()){
            return speed.parse::<u64>().unwrap();
        }
        else{
            println!("Bitte gib einen Wert größer als 10 an!")
        }
    }
}

pub fn get_height()->i32{

    loop{
        let mut height=String::new();
        println!("Hoehe des Feldes: ");
        let _b1 = std::io::stdin().read_line(&mut height).unwrap();
        let height = height.trim_end();
        let mut he=0;
        if let Some((_w, h)) = term_size::dimensions() {
            he=h as i32;
        } else {
            println!("Unable to get term size :(")
        }
        if check_numeric(height.to_string()){
            let  result=height.parse::<i32>().unwrap();
            if result<he-6 && result>2{
            return result
            }
            else{
                println!("Bitte gib nur ganze Zahlen als Wert ein,und maximal {} als größten Wert",he-7);
            }
        }
        else{
            println!("Bitte gib nur ganze Zahlen als Wert ein,und maximal {} als größten Wert",he-7);
            println!();
        }
    }
}

pub fn get_iterations()->i32{

    loop{
        let mut height=String::new();
        println!("Anzahl der Iteration: ");
        let _b1 = std::io::stdin().read_line(&mut height).unwrap();
        let height = height.trim_end();
        if height.len()<10{
            if check_numeric(height.to_string()){
                return height.parse::<i32>().unwrap();
            }
            else{
                println!("Bitte gib nur ganze Zahlen als Wert ein!");
                println!();
            }
        }
        else{
            println!("Der Input ist zu lang!")
        }
    }
}

pub fn get_width()->i32{

    loop{
        let mut width=String::new();
        println!("Weite des Feldes: ");
        let _b1 = std::io::stdin().read_line(&mut width).unwrap();
        let width = width.trim_end();
        let mut we=0;
        if let Some((w, _h)) = term_size::dimensions() {
            we=w as i32;
        } else {
            println!("Unable to get term size :(")
        }
        if check_numeric(width.to_string()){
            let result=width.parse::<i32>().unwrap();
            if result<we/3-2 && result>2{
            return result
            }
            else{
                println!("Bitte gib nur ganze Zahlen als Wert ein,und maximal {} als größten Wert",we/3-3);
            }
        }
        else{
            println!("Bitte gib nur ganze Zahlen als Wert ein,und maximal {} als größten Wert",we/3-3);
            println!();
        }
    }
    }


pub fn check_numeric(s:String)->bool{
    for c in s.chars(){
        if !c.is_numeric(){
            return false;
        }
    }
    true
}

#[derive(Clone, Copy)]
pub struct Cells{
    vertical_value: i32,
    status: i32,
    pos_x: usize,
    pos_y: usize,
    neighbors: i32,
}

impl Cells{
    pub fn add(&mut self,pos_x: i32,pos_y:i32,status:i32){
        self.pos_x = pos_x as usize;
        self.pos_y =pos_y as usize;
        self.status=status;
    }

    pub fn get_vertical_value(&mut self,containerarray:Vec<Cells>,hoehe:usize,weite:usize){
        if containerarray.len()>1{
            if self.pos_y==0_usize {
                self.vertical_value = containerarray[(self.pos_y + 1)*weite+self.pos_x].status + self.status+containerarray[(hoehe-1*weite)+self.pos_x].status;
            }
            else if self.pos_y == (containerarray.len() - 1) as usize{
                self.vertical_value = containerarray[(self.pos_y - 1)*weite+self.pos_x].status + self.status+containerarray[self.pos_x].status;
            }
            else{
                self.vertical_value = self.status + containerarray[(self.pos_y - 1*weite)+self.pos_x].status + containerarray[(self.pos_y + 1)*weite+self.pos_x].status;
            }
        }
        else{
            self.vertical_value=self.status;
        }
    }

    pub fn get_neighbors(&mut self,containerarray:Vec<Cells>,weite:usize){
        let mut gesamt=self.vertical_value-self.status;
            if gesamt < 0{
                gesamt=0;
            }

            if self.pos_x == 0_usize {
                self.neighbors = gesamt + containerarray[self.pos_y*weite +self.pos_x+1].vertical_value+ containerarray[self.pos_y*weite +weite-2 ].vertical_value;
            }
            else if self.pos_x == weite-1{
                self.neighbors =gesamt + containerarray[self.pos_y*weite +self.pos_x-1].vertical_value+containerarray[self.pos_y*weite ].vertical_value ;
            }
            else{

                self.neighbors= gesamt + containerarray[self.pos_y*weite+self.pos_x-1].vertical_value + containerarray[self.pos_y*weite+self.pos_x+1].vertical_value;
            }
    }

    pub fn update_status(&mut self){
        if self.neighbors>3 || self.neighbors<2{
            self.status = 0;
        }
        else if self.neighbors == 3{
            self.status = 1;
        }
    }
}


