use std::fmt;
use std::collections::HashMap;
use crate::piece::Rotates;
use crate::FIELDDIM;
use termion::{color, style};

#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct Point {
    pub x:usize, //Posx
    pub y:usize, //Posy
    pub k:u32, //Kanten
    pub f:i32, //Farbe
    pub l:char //Label
}


impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.f==0 {
            if self.l == '0' {write!(f, "Null-Point")}
            else{write!(f,"{}{}({},{}/{}){}",color::Bg(color::White),color::Fg(color::Black),self.x,self.y,self.k,style::Reset)}
        } else {
            if self.l == '0' {write!(f, "Null-Point")}
            else{write!(f,"{}{}({},{}/{}){}",color::Bg(color::Black),color::Fg(color::White),self.x,self.y,self.k,style::Reset)}
        }
    }
}

pub trait PointDefault {
    fn point0() -> Point {
        Point {
            x:0,
            y:0,
            k:2,
            f:0,
            l:'0',
        }
    } // Error Point
    fn ispoint0(&self) -> bool {true}
    fn compare(&self,_x:usize,_y:usize) -> bool {true} 
    fn fieldindex(&self,_fielddim:usize) -> usize {usize::MAX} 
    fn fieldindexmoved_old(&self,_fielddim:usize,_f:&Point,_p:&Point) -> usize {usize::MAX}
    fn pointmoved(&self,_f:&Point,_p:&Point) -> Point {Point::point0()}
    fn pointrotated(&self,_rotate:Rotates,_maxx:usize,_maxy:usize) -> Point {Point::point0()}
    fn decrease(&mut self) -> bool {false}
}
impl PointDefault for Point {
    fn ispoint0(&self) -> bool {if self.l == '0' {true}else{false}} // Check if Error Point
    fn compare(&self,x:usize,y:usize) -> bool {
        self.x == x && self.y == y
    } // compare x,y to point coordinates
    fn fieldindex(&self,fielddim:usize) -> usize {
        let retval:usize = self.x + fielddim*self.y;
        return retval
    } // return fieldindex of 2Dim (NxN) Array
    fn pointmoved(&self,n:&Point,p:&Point) -> Point {

        let dist1 = distance(n.x,p.x);
        let prx = if p.x < n.x { self.x+dist1} else {
            if dist1>self.x {return Point::point0()}
            else{self.x-dist1}
        };
        if prx>FIELDDIM-1{return Point::point0()}

        let dist1 = distance(n.y,p.y);
        let pry = if p.y < n.y { self.y+dist1} else {
            if dist1>self.y {return Point::point0()}
            else{self.y-dist1}
        };
        if pry>FIELDDIM-1{return Point::point0()}

        let r = Point {
            x:prx,
            y:pry,
            k:self.k,
            f:self.f,
            l:self.l,
        };
        //println!("point.rs.pointmoved {:?};N{:?}P{:?}=R{:?}",self,n,p,r);
        return r;
    } // return new point, if point self is moved in relation to point p is located on point f. Reference 2Dim (NxN) Array 
    fn pointrotated(&self,rotate:Rotates,maxx:usize,maxy:usize) -> Point {  
        match rotate {
            Rotates::N => {
                Point {
                    x:self.x,
                    y:self.y,
                    k:self.k,
                    f:self.f,
                    l:self.l,
            }},
            Rotates::X => {
                let r = calc(self.x as i32,maxx as i32,self.l);
                Point {
                    x:r.1,
                    y:self.y,
                    k:self.k,
                    f:self.f,
                    l:r.0,
                    }     
                },
            Rotates::Y => {
                let r = calc(self.y as i32,maxy as i32,self.l);
                Point {
                    x:self.x,
                    y:r.1,
                    k:self.k,
                    f:self.f,
                    l:r.0,
                    } 
                },
            Rotates::XY => {
                    let rx = calc(self.x as i32,maxx as i32,self.l);
                    let ry = calc(self.y as i32,maxy as i32,self.l);
                    Point {
                        x:rx.1,
                        y:ry.1,
                        k:self.k,
                        f:self.f,
                        l:rx.0,
                        } 
                },
            Rotates::R => {
                let r = calc(self.y as i32,maxy as i32,self.l);
                Point {
                    x:self.y,
                    y:self.x,
                    k:self.k,
                    f:self.f,
                    l:r.0,
                    } 
                },
            Rotates::RX => {
                let r = calc(self.y as i32,maxy as i32,self.l);
                Point {
                    x:r.1,
                    y:self.x,
                    k:self.k,
                    f:self.f,
                    l:r.0,
                    } 
                },
            Rotates::RXY => {
                    let ry = calc(self.y as i32,maxy as i32,self.l);
                    let rx = calc(self.x as i32,maxx as i32,self.l);
                    Point {
                        x:ry.1,
                        y:rx.1,
                        k:self.k,
                        f:self.f,
                        l:rx.0,
                        } 
                },
            Rotates::RY => {
                let r = calc(self.x as i32,maxx as i32,self.l);
                Point {
                    x:self.y,
                    y:r.1,
                    k:self.k,
                    f:self.f,
                    l:r.0,
                    }
                },
                Rotates::X41=> {
                    let r = calcx41(self.x as i32,maxx as i32,self.l);
                    Point {
                        x:r.1,
                        y:self.y,
                        k:self.k,
                        f:self.f,
                        l:r.0,
                        }
                    },
                }
    } // Rotate Point
    fn decrease(&mut self) -> bool {
        if self.k == 0 {return false}
        else{
            self.k = self.k -1;
            if self.k == 0 {return false}
            return true
        }
    }
}

pub struct Points {
    pub items : Vec<Point>,
    pub hm : HashMap<usize,usize>,
}
pub trait PointsDefault {
    fn new() -> Points{
        Points {
            items:vec![],
            hm:HashMap::new(),
        }    
    } 
    fn points0() -> Points {
        Points {
            items:vec![],
            hm:HashMap::new(),
        }
    }
    fn len(&self) -> usize {0}
    fn push(&mut self,_new:Point) -> bool {false}
    fn remove(&mut self,_old:Point) -> bool {false}
    fn contains(&self,_old:Point) -> bool {false}
    fn decrease(&mut self,_refo:Point) -> bool {false}
}
impl PointsDefault for Points {
    fn len(&self) -> usize {self.items.len()}
    fn push(&mut self,new:Point) -> bool {
        let key:usize = new.x + FIELDDIM*new.y;
        if ! self.hm.contains_key(&key){
            self.items.push(new.clone());
            self.hm.insert(key,self.items.len()-1);
            return true;
        }
        return false;
    }
    fn remove(&mut self,old:Point) -> bool {
        let key:usize = old.x + FIELDDIM*old.y;
        if ! self.hm.contains_key(&key){
            self.items.remove(self.hm[&key]);
            self.hm.remove(&key);
            return true;
        }
        return false;
    }
    fn contains(&self,new:Point) -> bool {
        let key:usize = new.x + FIELDDIM*new.y;
        self.hm.contains_key(&key)
    }
    fn decrease(&mut self,refo:Point) -> bool {
        let key:usize = refo.x + FIELDDIM*refo.y;
        if self.hm.contains_key(&key){
            if self.items[self.hm[&key]].k==0 {return false;}
            else{
                self.items[self.hm[&key]].k = self.items[self.hm[&key]].k-1;
                if self.items[self.hm[&key]].k==0 {return false;}
                return true;
            }
        }
        return false;
    }
 }

fn calc(v:i32,max:i32,sc:char) -> (char,usize) {
    let result = (v-max)*(-1);
    let resultus = result as usize;
    if resultus == usize::MAX {return ('0',usize::MAX)}else{return (sc,resultus)}
} // Help function for pointrotated to calc usize and return '0' for impossible results

fn calcx41(v:i32,max:i32,sc:char) -> (char,usize) {
    let result = max+((v)*(-1));
    let resultus = result as usize;
    if resultus == usize::MAX {return ('0',usize::MAX)}else{return (sc,resultus)}
} // Help function for pointrotated to calc usize and return '0' for impossible results
fn distance(n:usize,p:usize) -> usize {
    if n<p{p-n}else{n-p}
} // x or y distance between two points n or p.

