use std::fmt;
use crate::point::Point;
use crate::point::PointDefault;
use crate::point::Points;
use crate::point::PointsDefault;
use crate::FIELDDIM;
use crate::FIELDLEN;

#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, Hash)]
pub enum Rotates {
    N,
    X,
    Y,
    R,
    RX,
    RY,
    RXY,
    XY,
    X41,
}

pub struct Piece {
    pub title: String,
    pub index: usize,
    pub maxx: usize,
    pub maxy: usize,
    pub rotate: Vec<Rotates>,
    pub label: char,
    pub points: Points,
}
impl Piece {
    pub fn new(titleinit:String,indexinit:usize,maxxinit:usize,maxyinit:usize,rotateinit:Vec<Rotates>,labelinit:char) -> Piece {
    Piece {
        title:titleinit,
        index:indexinit,
        maxx:maxxinit,
        maxy:maxyinit,        
        rotate:rotateinit,
        label:labelinit,
        points:Points::new(),
        }
    }
}
impl fmt::Debug for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} Max:[{} {}] Length:{}"
            ,self.title
            , self.maxx
            , self.maxy
            ,self.points.len())
    }
}
pub trait PieceDefault {
    fn piece0() -> Piece{
        Piece{
            title:String::from("NULL"),
            index:0,
            maxx:0,
            maxy:0,
            rotate:vec![Rotates::N],
            label:'0',
            points:Points::new(),
        }
    }
    fn addpoint(&mut self,_newpoint:Point) {}
    fn piece_rotate(&self,_rotate:Rotates) -> Piece {Piece::piece0()}
    fn piece_move(&self,_neighbor:&Point,_pref:&Point,_rotate:Rotates,_field:&[Point;FIELDLEN]) -> Piece {Piece::piece0()}
}
impl PieceDefault for Piece {
    fn addpoint(&mut self,newpoint:Point) {
        self.points.push(newpoint);
    }
    fn piece_rotate(&self,rotate:Rotates) -> Piece {
        let mut title = String::from("Rotated(piece:");
        title.push(self.label);
        title.push(')');
        // Erstelle neues Stück, maxx und maxy müssen getauscht werden beim spiegeln
        let mut newpiece:Piece = if rotate==Rotates::R || rotate==Rotates::RX || rotate==Rotates::RY || rotate==Rotates::RXY {
            // Vertausche X und Y
            Piece::new(title,*&self.index
                ,*&self.maxy
                ,*&self.maxx
                ,vec![rotate],*&self.label)
        } else {
            Piece::new(title,*&self.index
                ,*&self.maxx
                ,*&self.maxy
                ,vec![rotate],*&self.label)
        };
        //Rotiere jeden einzelnen Punkt
        for i1 in 0..self.points.len() {
            let changedpoint = self.points.items[i1].pointrotated(rotate, *&self.maxx, *&self.maxy);
            newpiece.addpoint(changedpoint);
        }
        return newpiece;
    } 
    fn piece_move(&self,neighbor:&Point,pref:&Point,rotateinit:Rotates,field:&[Point;FIELDLEN]) -> Piece {
            // Nun muß ich das Piece für jeden Point pref auf den ausgewählten Neighbor legen.
            // Hier werde ich schauen ob
            // Felder schon belegt sind
            // Neighbors keine Kanten frei haben
            // und natürlich ob das Stück aus dem Feld herausragt.

            let mut title = String::from("Moved(");
            title.push(self.label);
            title.push(')');
            
            let mut rotate:Vec<Rotates> = Vec::new();
            rotate.push(rotateinit);

            let mut newpiece:Piece = Piece::new(title,*&self.index,*&self.maxx,*&self.maxy,rotate,*&self.label);
            
            //Verschiebe jedes einzelnen Punkt
            for i1 in 0..self.points.len() {
                //println!("piece.rs.piece_move INPUT{:?}",self.points[i1]);
                let changedpoint = self.points.items[i1].pointmoved(neighbor,pref);
                //println!("piece.rs.piece_move OUTPUT{:?}",changedpoint);
                // Feld ist schon belegt?
                if field[changedpoint.fieldindex(FIELDDIM)].l != ' ' {
                    //println!("piece.rs.piece_move new{:?}; Point{:?} - Besetzt",newpiece,changedpoint);
                    return Piece::piece0()
                }
                // Der Changedpoint ist ein 0 Vektor
                if changedpoint.l == '0' {
                    //println!("piece.rs.piece_move new{:?}; Point{:?} - 0 Vector",newpiece,changedpoint);
                    return Piece::piece0()
                } 

                if changedpoint.x > FIELDDIM || changedpoint.y > FIELDDIM { 
                    //println!("piece.rs.piece_move new{:?}; Point{:?} - ZuGroß",newpiece,changedpoint);
                    return Piece::piece0()
                    }

                if field[changedpoint.fieldindex(FIELDDIM)].f != changedpoint.f {
                    //println!("piece.rs.piece_move new{:?}; Point{:?} - FalscheFarbe",newpiece,changedpoint);
                    return Piece::piece0()
                    } 
                newpiece.addpoint(changedpoint);
            }
        return newpiece;
    }

}



