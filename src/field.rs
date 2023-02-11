use crate::piece::Piece;
use crate::point::Point;
use crate::point::Points;
use crate::FIELDDIM;
use crate::FIELDLEN;
#[allow(unused_imports)]use crate::point::PointsDefault;
#[allow(unused_imports)]use crate::piece::PieceDefault;

extern crate termion;
use termion::{color, style};
use crate::point::PointDefault;
use std::collections::HashMap;
#[allow(unused_imports)]use std::hash::Hash;

pub struct Field {
    pub field: [Point;FIELDLEN],
    pub piecessethm: HashMap<usize,String>,
    pub solutions: Vec<[Point;FIELDLEN]>,   
}
impl Field {
    pub fn new() -> Field {
        Field { 
            field: Self::create_field(),
            piecessethm: HashMap::new(),
            solutions:Vec::new(),
            }
    }
    pub fn new5() -> Field {
        Field { 
            field: Self::create_field5(),
            piecessethm: HashMap::new(),
            solutions:Vec::new(),
            }
        }
    }
pub trait Default {
    fn create_field()-> [Point;FIELDLEN] {
        let mut points:[Point;FIELDLEN] = [Point {
            x:0, // PosX
            y:0, // Posy
            k:2, // Kanten
            f:2, // Farbe
            l:' ',}; FIELDLEN];    
            for j in 0..FIELDDIM {
                for i in 0..FIELDDIM {
                    let p = i+j*FIELDDIM;
                points[p] = Point {
                    x:i, // PosX
                    y:j, // Posy
                    k:Self::calc_kanten(i,j), // Kanten
                    f:Self::calc_farbe(i,j), // Farbe
                    l:' ',
                    };
            }
        }
    return points

    }
    fn create_field5()-> [Point;FIELDLEN] {
        let mut points:[Point;FIELDLEN] = [Point {
            x:0, // PosX
            y:0, // Posy
            k:2, // Kanten
            f:2, // Farbe
            l:' ',}; FIELDLEN];    
            for j in 0..FIELDDIM {
                for i in 0..FIELDDIM {
                    let p = i+j*FIELDDIM;
                points[p] = Point {
                    x:i, // PosX
                    y:j, // Posy
                    k:Self::calc_kanten(i,j), // Kanten
                    f:Self::calc_farbe_black(i,j), // Farbe
                    l:' ',
                    };
            }
        } // Nutze calc_farbe_black statt calc_farbe
    return points

    }

    fn calc_kanten(i:usize,j:usize) -> u32 {
        if (i==0) && (j==0){return 2}
        else if (i==0) && (j==FIELDDIM-1){return 2}
        else if (i==FIELDDIM-1) && (j==0){return 2}
        else if (i==FIELDDIM-1) && (j==FIELDDIM-1){return 2}
        else if (i==0) || (i==FIELDDIM-1) || (j==0) || (j==FIELDDIM-1){return 3}
        return 4
    }
    fn calc_farbe(i:usize,j:usize) -> i32 {
        let mut color:i32;
        if i%2 == 0 {color = 0}else{color=1}
        if (j%2 == 1) && (color == 1){color=0}
        else if (j%2 == 1) && (color == 0){color=1}
        return color
    }
    fn calc_farbe_black(i:usize,j:usize) -> i32 {
        let mut color:i32;
        if i%2 == 1 {color = 1}else{color=0}
        if (j%2 == 0) && (color == 0){color=1}
        else if (j%2 == 0) && (color == 1){color=0}
        return color
    }

    fn show_brett(&mut self,_title:&str,_n: i32) {}
    fn show_brett_neighbors(&mut self,_title:&str,_n: i32,_neighbors:&Points) {}
    fn show_brett_piece(&mut self,_title:&str,_n: i32,_piece:&Piece) {}
    fn set_piece(&mut self,_piece:&Piece) {}
    fn unset_piece(&mut self,_points:&Vec<Point>){}
    fn calc_k(&mut self,_point:usize) -> u32 {0}
 
}


impl Default for Field {

    fn show_brett(&mut self,title:&str,n: i32) {
        println!("{}. {}",n,title);
        for p in 0..FIELDLEN {
            if  p%FIELDDIM == 0 {println!("{}",style::Reset);}
            if self.field[p].f == 0 {print!("{}{}{}{}"
                ,color::Bg(color::White)
                ,color::Fg(color::Black)
                ,self.field[p].k
                ,style::Reset);
            }
            else { print!("{}{}{}{}"
                ,color::Bg(color::Black)
                ,color::Fg(color::White)
                ,self.field[p].k
                ,style::Reset);
                }
        };
        println!("{}",style::Reset);
    } // Darstellung der Kanten des Feldes
    fn show_brett_neighbors(&mut self,title:&str,n: i32,neighbors:&Points) {
        print!(" {}. {} - {:?} - {}",n,title,self.piecessethm,self.solutions.len());
        let mut labels:HashMap<usize,char> = HashMap::new();
        for neighbor in &neighbors.items {
            labels.insert(neighbor.fieldindex(FIELDDIM)
                ,match neighbor.k {
                    0 => '0',
                    1 => '1',
                    2 => '2',
                    3 => '3',
                    4 => '4',
                    5_u32..=u32::MAX => '0',
                }); 
            }
        for p in 0..FIELDLEN {
            if p%FIELDDIM==0 {println!("{}",style::Reset);}
            if labels.contains_key(&p) {
                if self.field[p].f == 0 {print!("{}{}{}{}"
                    ,color::Bg(color::White)
                    ,color::Fg(color::Blue)
                    ,labels[&p]
                    ,style::Reset);}
                else { print!("{}{}{}{}"
                    ,color::Bg(color::Blue)
                    ,color::Fg(color::White)
                    ,labels[&p]
                    ,style::Reset);}
            }else{
                
                if self.field[p].f == 0 {print!("{}{}{}{}"
                    ,color::Bg(color::White)
                    ,color::Fg(color::Black)
                    ,self.field[p].l
                    ,style::Reset);}
                else { print!("{}{}{}{}"
                    ,color::Bg(color::Black)
                    ,color::Fg(color::White)
                    ,self.field[p].l
                    ,style::Reset);}
            };
        };
        println!("{}",style::Reset);
    } // Darstellung der Kanten der Neighbors
    fn show_brett_piece(&mut self,title:&str,n: i32,piece:&Piece) {

        print!(" {}. {} - {:?} - {}",n,title,self.piecessethm,self.solutions.len());
        let mut labels:HashMap<usize,char> = HashMap::new();    
        for i1 in 0..piece.points.len() {
            labels.insert(piece.points.items[i1].fieldindex(FIELDDIM),piece.points.items[i1].l);
            }
        for p in 0..FIELDLEN {
            if p%FIELDDIM==0 {println!("{}",style::Reset);}
            let c = if labels.contains_key(&p) {labels[&p]}else{self.field[p].l};
            if self.field[p].f == 0 {print!("{}{}{}{}"
                ,color::Bg(color::White)
                ,color::Fg(color::Black)
                ,c
                ,style::Reset);}
            else { print!("{}{}{}{}"
                ,color::Bg(color::Black)
                ,color::Fg(color::White)
                ,c
                ,style::Reset);}
        };
        println!("{}",style::Reset);
    } // Darstellung des Pieces, nicht farben treu!

    fn set_piece(&mut self,piece:&Piece){
        // Fülle Label des Feldes mit dem des Pieces und setze Kante auf 0.
        for p in 0..piece.points.len(){
            let key = piece.points.items[p].fieldindex(FIELDDIM);
            self.field[key].l = piece.points.items[p].l;
            //self.field[key].k = 0;
        };
    }

    fn unset_piece(&mut self,points:&Vec<Point>){
        // Nun muss ich die Punkte durchgehen und zurücksetzen.
        for p in 0..points.len(){
            let key = points[p].fieldindex(FIELDDIM);
            self.field[key].l=' ';
            //self.field[key].k=self.calc_k(key);
            }
        }
    fn calc_k(&mut self,key:usize) -> u32 {
            let fx = self.field[key].x;
            let fy = self.field[key].y;
            return (if (fy != 0) && (self.field[key-FIELDDIM].l==' '){1}else{0}) +
            (if (fy != FIELDDIM-1) && (self.field[key+FIELDDIM].l==' ') {1}else{0}) +
            (if ((fx%FIELDDIM) != 0) && (self.field[key-1].l==' '){1}else{0}) +
            (if ((fx%FIELDDIM) != FIELDDIM-1) && (self.field[key+1].l==' '){1}else{0});
        } // Kantenkalkulation
    
}

