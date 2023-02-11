#[allow(unused_imports)] use std::env;
#[allow(unused_imports)] use std::process;
#[allow(unused_imports)] use std::collections::HashMap;
use std::time::{Instant}; // Duration z.B. für let five_seconds = Duration::new(5, 0);
//use std::thread; // z.B. für thread::sleep(Duration::from_secs(1));

// Während der Iteration werden die belegten Felder nur angezeigt,
const VIEWSTART:i32=1;  //wenn die Iteration < View ist.
const VIEWEND:i32=14;  //wenn die Iteration > View ist.
#[allow(dead_code)] const MYEXIT:i32=14;

// Count of Pieces, check crate::pieces::create_pieces
const FIELDDIM:usize=8;
const FIELDLEN:usize=FIELDDIM*FIELDDIM;

pub mod field;
pub mod piece;
pub mod point;
pub mod pieces;

use crate::field::Field;
use crate::field::Default;
use crate::piece::Piece;
use crate::piece::PieceDefault;
use crate::point::Point;
use crate::point::PointDefault;
use crate::pieces::create_pieces;
use crate::pieces::Pieces;
#[allow(unused_imports)] use crate::point::PointsDefault;
#[allow(unused_imports)] use crate::point::Points;
#[allow(unused_imports)] use crate::piece::Rotates;

pub fn recursive_function(n: i32,field: &mut Field,pieces:&Pieces,neighbors:&Points) {

    let mut neighbornumber:usize=usize::MAX;
    'firstfor: for i0 in 0..neighbors.len(){
        if neighbors.items[i0].k==1 {
            neighbornumber = i0;
            break 'firstfor;
        }    
    }
    if neighbornumber == usize::MAX {
        'secondfor: for i0 in 0..neighbors.len(){
            if neighbors.items[i0].k==2 {
                neighbornumber = i0;
                break 'secondfor;
            }        
        }
        if neighbornumber == usize::MAX {neighbornumber = 0;}
    }
        //for i0 in 0..field.neighbors.len(){
        let neighbor = neighbors.items[neighbornumber]; // TBD: Nimm den Neighbor mit der geringsten Kantenzahl
        // Versuche nun ein Teil auf das Brett zu legen, wobei es den ausgewählten neighbor berührt.  
        for i1 in 0..pieces.vecs.len() {
            // Zuerst Schleife über alle Pieces
            if !field.piecessethm.contains_key(&i1) {                    
                // Natürlich nur, wenn das Stück nicht schon auf dem Brett liegt.
                let pvec = &pieces.vecs[i1];
                for i2 in 0..pvec.rotate.len() {
                    // Das Stück kann in verschieden Positionen gedreht werden, 
                    // Start mit Rotate:N - Standard Positiion
                    let rotate = pvec.rotate[i2];
                    // Das rotieren passiert noch Feldunabhängig
                    let pvecrot:Piece = pvec.piece_rotate(rotate);

                    //println!("rF[1]: Neighbor{:?}; PiecesIndex:{}; rotate:{:?}; pvecrot{:?}",neighbor,i1,rotate,pvecrot);
                    for pref in &pvecrot.points.items {
                        // Nun muß ich das Piece für jeden Point pref auf den ausgewählten Neighbor legen.
                        // Dabei werde ich schauen ob:
                        // Felder schon belegtself.points sind, falsche Farben auf einander liegen
                        // und natürlich ob das Stück aus dem Feld herausragt.
                        //println!("rF[2]: Neighbor{:?}; PiecesIndex:{}; rotate:{:?}; pvecrot{:?}; prev{:?}",neighbor,i1,rotate,pvecrot,pref);
                        if (n==13) && (rotate == Rotates::X41) {
                            //println!("Solution-1? {} pvecrot:{:?} rotate:{:?} neighbor:{:?} pref:{:?}",neighbornumber,pvecrot,rotate,neighbor,pref);
                            //println!("1Items: {:?}",&pvecrot.points.items);
                            //println!("{} {}",neighbor.k,pref.k);
                            };
                        

                        let pvecmoved = if neighbor.k < pref.k {
                            Piece::piece0()
                        }
                        else {
                            pvecrot.piece_move(&neighbor,pref,rotate,&field.field)
                        };
                        //if (n==13) && (rotate == Rotates::X41) {println!("Solution-2? {} pvecmoved:{:?} neighbor:{:?} pref:{:?}",neighbornumber,pvecmoved,neighbor,pref);std::process::exit(0);};
                        // Zurück gegeben werden mir die belegten Felder oder das Point0, welches an dem 0 Label zu erkennen ist.
                        if pvecmoved.label != '0' {
                            // Nun kalkuliere ich die Nachbarn neu durch, wobei das neue Stück mit einbezogen wird.
                            // println!("rF[3]: Neighbor{:?}; PiecesIndex:{}; rotate:{:?}; pvecrot{:?}; prev{:?}; pvecmoved{:?}",neighbor,i1,rotate,pvecrot,pref,pvecmoved);
                            let newneighbors:Points = neighbors_update(&pvecmoved,neighbors,&field.field);

                            // Dabei gebe ich false zurück, wenn ich einen einsamen Nachbarn finde
                            // Auch alle indirekten alten Neighbors werde ich hier einfügen.
                            if (newneighbors.len()!=0) || (field.piecessethm.len()+1 == pieces.vecs.len()) {
                                // Das Stück kann auf dem Feld plaziert werden.
                                field.set_piece(&pvecmoved);
                                field.piecessethm.insert(i1,pvecmoved.title); // Stück ist plaziert

                                if n>VIEWEND {
                                    field.show_brett_neighbors("End",n,&newneighbors);
                                    println!("Piecessethm {:?} {:?}",field.piecessethm.len(),pieces.vecs.len());
                                } else if n<VIEWSTART {
                                    field.show_brett_neighbors("Start",n,&newneighbors);
                                    //println!("Piecessethm{:?} {:?}",field.piecessethm,field.piecessethm.contains_key(&i1));
                                }                            

                                if field.piecessethm.len() == pieces.vecs.len(){ 
                                    // Lösung erreicht
                                    //field.set_piece(&pvecmoved);
                                    field.show_brett_neighbors("Solution",n,&newneighbors);
                                    field.solutions.push(field.field.clone());
                                    println!("SOLUTION {}",field.solutions.len());
                                    println!("");
                                    field.unset_piece(&pvecmoved.points.items);
                                    field.piecessethm.remove(&i1);
                                    //std::process::exit(0);
                                } else if field.piecessethm.len() < pieces.vecs.len(){ 
                                    // Iterate
                                    recursive_function(n+1,field,pieces,&newneighbors);
                                    // remove pieces
                                    field.unset_piece(&pvecmoved.points.items);
                                    field.piecessethm.remove(&i1);
                                }
                                
                            }
                        }
                    }
                }
            }
        }
    
}

pub fn neighbor_check_old(x:usize,y:usize,field:&[Point;FIELDLEN],piece:&Piece,oldneighbors:&Points) -> Point {
    let retval = Point {x:x,y:y,k:0,f:0,l:'_'};
    let key = retval.fieldindex(FIELDDIM);
    let free = !piece.points.hm.contains_key(&key);
    if free && oldneighbors.hm.contains_key(&key){
        // ein alter Bekannter
        let mut oldneighbor = oldneighbors.items[oldneighbors.hm[&key]].clone();
        oldneighbor.decrease();
        return oldneighbor
    } else if free && (field[key].l == ' ') {
        let mut newneighbor = field[key].clone();
        newneighbor.decrease();
        return newneighbor
    } else {return Point::point0()}
}
pub fn neighbor_check(x:usize,y:usize,field:&[Point;FIELDLEN],piece:&Piece,oldneighbors:&Points) -> Point {
    let retval = Point {x:x,y:y,k:0,f:0,l:'_'};
    let key = retval.fieldindex(FIELDDIM);
    let free = !piece.points.hm.contains_key(&key);
    if free && oldneighbors.hm.contains_key(&key){
        // ein alter Bekannter
        let mut oldneighbor = oldneighbors.items[oldneighbors.hm[&key]].clone();
        oldneighbor.decrease();
        return oldneighbor
    } else if free && (field[key].l == ' ') {
        let mut newneighbor = field[key].clone();
        //println!("Field:{:?}",newneighbor);
        newneighbor.decrease();
        return newneighbor
    } else {return Point::point0()}
}
pub fn neighbors_update_old(piece:&Piece,oldneighbors:&Points,field:&[Point;FIELDLEN]) -> Points {

    // Berechne Nachbarn für jeden Punkt des pieces
    let mut newneighbors:Points = Points::new(); 
    // Attach Neighbors of new point
    for i in 0..piece.points.len() {
        let p:Point = piece.points.items[i];
        //println!("main.rs.neighbors_update[1]{:?}",p);
        let mut retval:[Point;4]=[Point::point0();4];
        retval[0] = if p.x > 0  {neighbor_check( p.x-1, p.y,&field,piece,&oldneighbors)}else{Point::point0()};
        retval[1] = if p.y > 0  {neighbor_check( p.x, p.y-1  ,&field,piece,&oldneighbors)}else{Point::point0()};
        retval[2] = if p.x < FIELDDIM-1  {neighbor_check( p.x+1, p.y,&field,piece,&oldneighbors)}else{Point::point0()};
        retval[3] = if p.y < FIELDDIM-1  {neighbor_check( p.x, p.y+1,&field,piece,&oldneighbors)}else{Point::point0()};
        for i in 0..retval.len(){
            if retval[i].l != '0' {
                println!("main.rs.neighbors_update[2]{:?}",retval[i]);
                if retval[i].k == 0 {return Points::points0()}
                else if !newneighbors.contains(retval[i]){newneighbors.push(retval[i]);}
                else{
                    let retboolval = newneighbors.decrease(retval[i]);
                    if !retboolval {return Points::points0()}
                }
            }
        }
    }
    // Attach Old Neighbors
    for i in 0..oldneighbors.len() {
        if !piece.points.contains(oldneighbors.items[i]) {
           if !newneighbors.contains(oldneighbors.items[i]){
              newneighbors.push(oldneighbors.items[i].clone());
           }
        }
    };
return newneighbors;
}

pub fn neighbors_update(piece:&Piece,oldneighbors:&Points,field:&[Point;FIELDLEN]) -> Points {

    // Berechne Nachbarn für jeden Punkt des pieces
    let mut newneighbors:Points = Points::new(); 
    let mut retval:Vec<Point>=Vec::new();

    // Attach Neighbors of new point
    for i in 0..piece.points.len() {
        let p:Point = piece.points.items[i];
        //println!("main.rs.neighbors_update[1]{:?}",p);Rotates
        retval.push(if p.x > 0  {neighbor_check( p.x-1, p.y,&field,piece,&oldneighbors)}else{Point::point0()});
        retval.push(if p.y > 0  {neighbor_check( p.x, p.y-1  ,&field,piece,&oldneighbors)}else{Point::point0()});
        retval.push(if p.x < FIELDDIM-1  {neighbor_check( p.x+1, p.y,&field,piece,&oldneighbors)}else{Point::point0()});
        retval.push(if p.y < FIELDDIM-1  {neighbor_check( p.x, p.y+1,&field,piece,&oldneighbors)}else{Point::point0()});
    }

    for i in 0..retval.len(){
        if retval[i].l != '0' {
            //println!("main.rs.neighbors_update[2]{:?}",retval[i]);
            if retval[i].k == 0 {return Points::points0()}
            else if !newneighbors.contains(retval[i]){
                if retval[i].k == 0 {return Points::points0()}
                else {
                    if retval[i].k == 0 {return Points::points0()}
                    newneighbors.push(retval[i]);
                    }
                }
            else{
                let retboolval = newneighbors.decrease(retval[i]);
                if !retboolval {return Points::points0()}
            }
        }
    }

    // Attach Old Neighbors
    for i in 0..oldneighbors.len() {
        if !piece.points.contains(oldneighbors.items[i]) {
           if !newneighbors.contains(oldneighbors.items[i]){
              newneighbors.push(oldneighbors.items[i].clone());
           }
        }
    };
return newneighbors;
}

fn main() {

    let args: Vec<String> = env::args().collect();

    let pieces = create_pieces();  

    let mut field:Field= if FIELDDIM == 5 {Field::new5()}
    else {Field::new()};
    let mut neighbors:Points = Points::new();

    match args.len() {
        // no arguments passed, just run
        1 => {
            let start = Instant::now();
            println!();
            println!("Start recursive Function:");
            neighbors.push(Point {x:0,y:0,k:2,f:0,l:'_'});
            field.show_brett("Start", 1);
            recursive_function(1,&mut field,&pieces,&neighbors);
            let duration = start.elapsed();
            println!("Time elapsed in recursive_function() is: {:?}", duration);
            },
        // one argument passed
        2 => {
            let number:usize = match args[1].parse() {
                Ok(n) => {n},
                Err(_) => {eprintln!("error: argument is not an integer");return;},
            };
            for i in 0..pieces.vecs[number].rotate.len() {
                let piecerot = pieces.vecs[number].piece_rotate(pieces.vecs[number].rotate[i]);
                field.show_brett_piece("Test Piece",99,&piecerot);
                println!("Rotated {:?}",pieces.vecs[number].rotate[i]  );
            }
            },
        _ => {
            // show a help message
            println!("no help message available");
            }
    }    
    std::process::exit(0);
}
