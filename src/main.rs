use rand::distributions::{Distribution, Uniform};
use std::{thread, time};
use std::io::{Write};
use std::io::stdout;
use crossterm::{QueueableCommand};
use win32console::console::WinConsole;
use term_size;

fn main() {
    let mut stdout = stdout();
    let mut _best="y";
    let mut _h=String::new();
    let mut containerarray:Vec<Vec<Cells>>=Vec::new();
    let hoehe:i32 = get_height(); 
    let weite:i32 = get_width();
    let iteration:i32= get_iterations();
    let sign:String=get_sign();
    let ten_millis = time::Duration::from_millis(get_speed());
    

    let mut rng = rand::thread_rng();
    let gen = Uniform::from(0..2);
    
    for x in 0..hoehe{
        containerarray.push(Vec::new());
        for d in 0..weite{
            let cell=Cells{neighbors: 0,pos_x:d as usize,pos_y:x as usize,status:gen.sample(&mut rng),vertical_value:0 };
            containerarray[x as usize].push(cell);
        }
    }

    let mut drip:Vec<Vec<Cells>>=containerarray.clone();
    WinConsole::output().clear().expect("Irgendwas lief falsch");
    stdout.queue(crossterm::cursor::Hide).expect("Irgendwas lief falsch");
    for u in 0..iteration+1{
        let mut temp:Vec<Vec<Cells>>=Vec::new();

        for x in 0..hoehe{
            temp.push(Vec::new());
            for d in 0..weite{
                let mut zelle= Cells{..drip[x as usize][d as usize]};                
                zelle.get_vertical_value(drip.clone());
                temp[x as usize].push(Cells{neighbors : 0, pos_y : x as usize, pos_x : d as usize, ..zelle});
            }
        }

        containerarray=temp.clone();
        temp=Vec::new();
        let mut ausgabe=String::new();
        stdout.queue(crossterm::cursor::MoveTo(0,0)).expect("Irgendwas lief falsch");
        ausgabe+=&"   ".to_string();
        for _x in 0..weite{
        ausgabe+=&"___".to_string();
        }

        ausgabe+=&" \n".to_string();

        for x in 0..hoehe{
            temp.push(Vec::new());
            ausgabe+=&"  |".to_string();
            for d in 0..weite{
                let mut zelle= Cells{..containerarray[x as usize][d as usize]};
                zelle.get_neighbors(containerarray.clone());
                zelle.update_status();
                if zelle.status == 1{
                    ausgabe+=&" ".to_string();
                    ausgabe+=&sign;
                    ausgabe+=&" ".to_string();
                }
                else{
                    ausgabe+=&"   ".to_string();
                }
                temp[x as usize].push(Cells{neighbors : 0, pos_y : x as usize, pos_x : d as usize, ..zelle});
            }
            ausgabe+=&"|\n".to_string();
        }

        ausgabe+=&"  |".to_string();

        for _x in 0..weite{
            ausgabe+=&"___".to_string();
        }

        ausgabe+=&"|\n".to_string();
        ausgabe+=&"  Iterationen: ".to_string();
        ausgabe+=&u.to_string();
        ausgabe+=&"\\".to_string();
        ausgabe+=&iteration.to_string();
        ausgabe+=&"\n".to_string();
        stdout.write_all(format!("{}", ausgabe).as_bytes()).expect("Irgendwas lief falsch");
        containerarray=temp.clone();
        drip =containerarray.clone();
        thread::sleep(ten_millis);
    }
    stdout.queue(crossterm::cursor::Show).expect("Irgendwas lief falsch");
    println!("Simulation beendet! Drücke ENTER");
    let _b1 = std::io::stdin().read_line(&mut _h).unwrap();
    WinConsole::output().clear().expect("Irgendwas lief falsch");
}

pub fn get_sign()->String{
    loop{
        let mut sign=String::new();
        println!("Darstellungszeichen der Zellen: ");
        let _b1 = std::io::stdin().read_line(&mut sign).unwrap();
        let sign = sign.trim_end();
        if sign.len() == 1{
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
            if result<he-3 && result>2{
            return result
            }
        }
        else{
            println!("Bitte gib nur ganze Zahlen als Wert ein!");
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
            if result<we/3 && result>2{
            return result
            }
        }
        else{
            println!("Bitte gib nur ganze Zahlen als Wert ein!");
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
    return true;
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

    pub fn get_vertical_value(&mut self,containerarray:Vec<Vec<Cells>>){
        if containerarray.len()>1{
            if self.pos_y==0_usize {
                self.vertical_value = containerarray[self.pos_y + 1][self.pos_x].status + self.status;
            }
            else if self.pos_y == (containerarray.len() - 1) as usize{
                self.vertical_value = containerarray[self.pos_y - 1][self.pos_x].status + self.status;
            }
            else{
                self.vertical_value = self.status + containerarray[self.pos_y - 1][self.pos_x].status + containerarray[self.pos_y + 1][self.pos_x].status;
            }
        }
        else{
            self.vertical_value=self.status;
        }
    }

    pub fn get_neighbors(&mut self,containerarray:Vec<Vec<Cells>>){
        let mut gesamt=self.vertical_value-self.status;
        if containerarray[0].len()>1{
            if gesamt < 0{
                gesamt=0;
            }

            if self.pos_x == 0_usize {
                self.neighbors = gesamt + containerarray[self.pos_y ][self.pos_x+1].vertical_value;
            }
            else if self.pos_x == (containerarray[0].len()-1 ) as usize{
                self.neighbors =gesamt + containerarray[self.pos_y][self.pos_x-1].vertical_value ;
            }
            else{

                self.neighbors= gesamt + containerarray[self.pos_y][self.pos_x-1].vertical_value + containerarray[self.pos_y][self.pos_x+1].vertical_value;
            }
        }
        else{
            self.neighbors=0;
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

#[cfg(test)]
mod eval {

    use crate::Cells;
    use crate::check_numeric;

    fn set_up()->Vec<Vec<Cells>>{
        return vec![vec![Cells{pos_x:0,pos_y:0,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:0,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:0,status:0,vertical_value:0,neighbors:0}],
   vec![Cells{pos_x:0,pos_y:1,status:0,vertical_value:2,neighbors:0},
        Cells{pos_x:1,pos_y:1,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:1,status:0,vertical_value:3,neighbors:0}
],vec![ Cells{pos_x:0,pos_y:2,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:2,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:2,status:0,vertical_value:0,neighbors:0}
]];
    }

    fn set_up1()->Vec<Vec<Cells>>{
        return vec![vec![Cells{pos_x:0,pos_y:0,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:0,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:0,status:0,vertical_value:0,neighbors:0}],
   vec![Cells{pos_x:0,pos_y:1,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:1,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:1,status:0,vertical_value:0,neighbors:0}
],vec![ Cells{pos_x:0,pos_y:2,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:2,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:2,status:0,vertical_value:0,neighbors:0}
]];
    }

    fn set_up2()->Vec<Vec<Cells>>{
        return vec![vec![Cells{pos_x:0,pos_y:0,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:0,status:0,vertical_value:3,neighbors:0},
        Cells{pos_x:2,pos_y:0,status:0,vertical_value:0,neighbors:0}],
   vec![Cells{pos_x:0,pos_y:1,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:1,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:1,status:0,vertical_value:0,neighbors:0}
],vec![ Cells{pos_x:0,pos_y:2,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:2,status:0,vertical_value:3,neighbors:0},
        Cells{pos_x:2,pos_y:2,status:1,vertical_value:3,neighbors:0}
]];
    }

    fn set_up3()->Vec<Vec<Cells>>{
        return vec![vec![Cells{pos_x:0,pos_y:0,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:0,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:0,status:0,vertical_value:0,neighbors:0}],
   vec![Cells{pos_x:0,pos_y:1,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:1,status:1,vertical_value:3,neighbors:0},
        Cells{pos_x:2,pos_y:1,status:0,vertical_value:0,neighbors:0}
],vec![ Cells{pos_x:0,pos_y:2,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:2,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:2,status:0,vertical_value:0,neighbors:0}
]];
    }

    fn set_up4()->Vec<Vec<Cells>>{
        return vec![vec![Cells{pos_x:0,pos_y:0,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:0,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:0,status:1,vertical_value:0,neighbors:0}],
   vec![Cells{pos_x:0,pos_y:1,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:1,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:1,status:1,vertical_value:0,neighbors:0}
],vec![ Cells{pos_x:0,pos_y:2,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:2,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:2,status:1,vertical_value:0,neighbors:0}
]];
    }
    fn set_up5()->Vec<Vec<Cells>>{
        return vec![vec![Cells{pos_x:0,pos_y:0,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:0,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:0,status:1,vertical_value:0,neighbors:0}],
   vec![Cells{pos_x:0,pos_y:1,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:1,status:0,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:1,status:1,vertical_value:0,neighbors:0}
],vec![ Cells{pos_x:0,pos_y:2,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:1,pos_y:2,status:1,vertical_value:0,neighbors:0},
        Cells{pos_x:2,pos_y:2,status:1,vertical_value:0,neighbors:0}
]];
    }

    fn set_up6()->Vec<Vec<Cells>>{
        return vec![vec![Cells{pos_x:0,pos_y:0,status:1,vertical_value:0,neighbors:0}]];
    }
    fn set_up7()->Vec<Vec<Cells>>{
        return vec![vec![Cells{pos_x:0,pos_y:0,status:1,vertical_value:1,neighbors:0},Cells{pos_x:1,pos_y:0,status:1,vertical_value:1,neighbors:0},Cells{pos_x:2,pos_y:0,status:1,vertical_value:1,neighbors:0}]];
    }

    #[test]
    fn vertical_value_one() {
        let test=set_up();
        let mut zelle:Cells=Cells{..test[1][1]};
        zelle.get_vertical_value(test);
        assert_eq!(zelle.vertical_value,1);
    }

    #[test]
    fn vertical_value_three() {
        let test=set_up1();
        let mut zelle:Cells=Cells{..test[1][1]};
        zelle.get_vertical_value(test);
        assert_eq!(zelle.vertical_value,3);
    }
    #[test]
    fn vertical_value_two() {
        let test=set_up3();
        let mut zelle:Cells=Cells{..test[1][1]};
        zelle.get_vertical_value(test);
        assert_eq!(zelle.vertical_value,2);
    }

    #[test]
    fn vertical_value_zero() {
        let test=set_up2();
        let mut zelle:Cells=Cells{..test[1][1]};
        zelle.get_vertical_value(test);
        assert_eq!(zelle.vertical_value,0);
    }
    #[test]
    fn neighbors_both_sides() {
        let test=set_up();
        let mut zelle:Cells=Cells{..test[1][1]};
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,5);
    }
    #[test]
    fn neighbors_zero() {
        let test=set_up1();
        let mut zelle:Cells=Cells{..test[1][1]};
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,0);
    }
    #[test]
    fn neighbors_both_left() {
        let test=set_up2();
        let mut zelle:Cells=Cells{..test[0][0]};
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,3);
    }
    #[test]
    fn neighbors_left() {
        let test=set_up3();
        let mut zelle:Cells=Cells{..test[1][0]};
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,3);
    }
    #[test]
    fn neighbors_bottom_right() {
        let test=set_up2();
        let mut zelle:Cells=Cells{..test[2][2]};
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,5);
    }
    #[test]
    fn all_cells() {
        let mut test=set_up4();
        let mut zelle0:Cells=Cells{..test[1][0]};
        let mut zelle:Cells=Cells{..test[1][1]};
        let mut zelle1:Cells=Cells{..test[1][2]};
        zelle0.get_vertical_value(test.clone());
        zelle.get_vertical_value(test.clone());
        zelle1.get_vertical_value(test.clone());
        test[1][0]=zelle0;
        test[1][1]=zelle;
        test[1][2]=zelle1;
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,8);
    }
    #[test]
    fn all_cells_minus_self() {
        let mut test=set_up5();
        let mut zelle0:Cells=Cells{..test[1][0]};
        let mut zelle:Cells=Cells{..test[1][1]};
        let mut zelle1:Cells=Cells{..test[1][2]};
        zelle0.get_vertical_value(test.clone());
        zelle.get_vertical_value(test.clone());
        zelle1.get_vertical_value(test.clone());
        test[1][0]=zelle0;
        test[1][1]=zelle;
        test[1][2]=zelle1;
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,8);
    }
    #[test]
    fn all_cells_bottom_left() {
        let mut test=set_up4();
        let mut zelle:Cells=Cells{..test[2][0]};
        let mut zelle1:Cells=Cells{..test[2][1]};
        zelle.get_vertical_value(test.clone());
        zelle1.get_vertical_value(test.clone());
        test[2][0]=zelle;
        test[2][1]=zelle1;
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,3);
    }
    #[test]
    fn all_cells_bottom_right() {
        let mut test=set_up4();
        let mut zelle:Cells=Cells{..test[2][2]};
        let mut zelle1:Cells=Cells{..test[2][1]};
        zelle.get_vertical_value(test.clone());
        zelle1.get_vertical_value(test.clone());
        test[2][2]=zelle;
        test[2][1]=zelle1;
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,3);
    }
    #[test]
    fn all_cells_top_left() {
        let mut test=set_up4();
        let mut zelle:Cells=Cells{..test[0][0]};
        let mut zelle1:Cells=Cells{..test[0][1]};
        zelle.get_vertical_value(test.clone());
        zelle1.get_vertical_value(test.clone());
        test[0][0]=zelle;
        test[0][1]=zelle1;
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,3);
    }
    #[test]
    fn all_cells_top_right() {
        let mut test=set_up4();
        let mut zelle:Cells=Cells{..test[0][2]};
        let mut zelle1:Cells=Cells{..test[0][1]};
        zelle.get_vertical_value(test.clone());
        zelle1.get_vertical_value(test.clone());
        test[0][2]=zelle;
        test[0][1]=zelle1;
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,3);
    }
    #[test]
    fn all_cells_bottom_middle() {
        let mut test=set_up4();
        let mut zelle0:Cells=Cells{..test[2][0]};
        let mut zelle:Cells=Cells{..test[2][1]};
        let mut zelle1:Cells=Cells{..test[2][2]};
        zelle0.get_vertical_value(test.clone());
        zelle.get_vertical_value(test.clone());
        zelle1.get_vertical_value(test.clone());
        test[2][0]=zelle0;
        test[2][1]=zelle;
        test[2][2]=zelle1;
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,5);
    }
    #[test]
    fn all_cells_top_middle() {
        let mut test=set_up4();
        let mut zelle0:Cells=Cells{..test[0][0]};
        let mut zelle:Cells=Cells{..test[0][1]};
        let mut zelle1:Cells=Cells{..test[0][2]};
        zelle0.get_vertical_value(test.clone());
        zelle.get_vertical_value(test.clone());
        zelle1.get_vertical_value(test.clone());
        test[0][0]=zelle0;
        test[0][1]=zelle;
        test[0][2]=zelle1;
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,5);
    }
    #[test]
    fn single_cell_get_vertical(){
        let test=set_up6();
        let mut zelle:Cells=Cells{..test[0][0]};
        zelle.get_vertical_value(test);
        assert_eq!(zelle.vertical_value,1);
    }
    #[test]
    fn single_cell_get_neighbors(){
        let test=set_up6();
        let mut zelle:Cells=Cells{..test[0][0]};
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,0);
    }
    #[test]
    fn single_cell_row_get_neighbors(){
        let test=set_up7();
        let mut zelle:Cells=Cells{..test[0][1]};
        zelle.get_neighbors(test);
        assert_eq!(zelle.neighbors,2);
    }
    #[test]
    fn check_numeric_false(){
        assert_eq!(check_numeric("ezrtis123".to_string()),false);
    }
    #[test]
    fn check_numeric_true(){
        assert_eq!(check_numeric("123".to_string()),true);
    }

}
