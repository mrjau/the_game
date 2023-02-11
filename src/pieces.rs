
use crate::point::Point;
use crate::piece::Piece;
use crate::piece::PieceDefault;
use crate::piece::Rotates;

pub struct Pieces {
    // pub array:[char;3],
    pub vecs:Vec<Piece>,
}
pub fn create_pieces_old() -> Pieces{
    let mut pieces:Vec<Piece> = Vec::new();
    // Quadrat
    let mut piece0:Piece = Piece::new(String::from("Quader 2x2"),4,2,2,vec![Rotates::N,Rotates::X],'Q');
    piece0.addpoint(Point{x:0,y:0,k:2,f:0,l:'Q'});
    piece0.addpoint(Point{x:1,y:0,k:2,f:1,l:'Q'});
    piece0.addpoint(Point{x:0,y:1,k:2,f:1,l:'Q'});
    piece0.addpoint(Point{x:1,y:1,k:2,f:0,l:'Q'});
    //piece0.neighbors_update(0,0);
    pieces.push(piece0);
    // Corner
    let mut piece1:Piece = Piece::new(String::from("Corner"),5,3,3,vec![Rotates::N,Rotates::X,Rotates::Y,Rotates::R],'C');
    piece1.addpoint(Point{x:0,y:0,k:2,f:0,l:'C'});
    piece1.addpoint(Point{x:1,y:0,k:2,f:1,l:'C'});
    piece1.addpoint(Point{x:2,y:0,k:2,f:0,l:'C'});
    piece1.addpoint(Point{x:0,y:1,k:2,f:1,l:'C'});
    piece1.addpoint(Point{x:0,y:2,k:2,f:0,l:'C'});
    //piece1.neighbors_update(0,0);
    pieces.push(piece1);
    // Corner
    let mut piece2:Piece = Piece::new(String::from("Long"),5,5,1,vec![Rotates::N,Rotates::R],'L');
    piece2.addpoint(Point{x:0,y:0,k:1,f:1,l:'L'});
    piece2.addpoint(Point{x:1,y:0,k:2,f:0,l:'L'});
    piece2.addpoint(Point{x:2,y:0,k:2,f:1,l:'L'});
    piece2.addpoint(Point{x:3,y:0,k:2,f:0,l:'L'});
    piece2.addpoint(Point{x:4,y:0,k:1,f:1,l:'L'});
    // piece2.neighbors_update(0,0);
    pieces.push(piece2);
    //return Pieces{array:['Q','C','L'],vecs:pieces}
    return Pieces{vecs:pieces}
}

pub fn create_pieces3() -> Pieces{
    let mut pieces:Vec<Piece> = Vec::new();
    // Quadrat
    let mut piece0:Piece = Piece::new(String::from("Quader 2x2"),4,1,1,vec![Rotates::N,Rotates::X],'Q');
    piece0.addpoint(Point{x:0,y:0,k:2,f:0,l:'Q'});
    piece0.addpoint(Point{x:1,y:0,k:2,f:1,l:'Q'});
    piece0.addpoint(Point{x:0,y:1,k:2,f:1,l:'Q'});
    piece0.addpoint(Point{x:1,y:1,k:2,f:0,l:'Q'});
    //piece0.neighbors_update(0,0);
    pieces.push(piece0);
    // Corner
    let mut piece1:Piece = Piece::new(String::from("Corner"),5,2,2,vec![Rotates::N,Rotates::X,Rotates::Y,Rotates::XY],'C');
    piece1.addpoint(Point{x:0,y:0,k:2,f:0,l:'C'});
    piece1.addpoint(Point{x:1,y:0,k:2,f:1,l:'C'});
    piece1.addpoint(Point{x:2,y:0,k:2,f:0,l:'C'});
    piece1.addpoint(Point{x:0,y:1,k:2,f:1,l:'C'});
    piece1.addpoint(Point{x:0,y:2,k:2,f:0,l:'C'});
    //piece1.neighbors_update(0,0);
    pieces.push(piece1);
    return Pieces{vecs:pieces}
}

pub fn create_pieces() -> Pieces{
    let mut pieces:Vec<Piece> = Vec::new();
    // Long 5x1
    let mut piece0:Piece = Piece::new(String::from("Long 5x1")
        ,5,4,1
        ,vec![Rotates::N,Rotates::R]
        ,'L');
    piece0.addpoint(Point{x:0,y:0,k:1,f:1,l:'L'});
    piece0.addpoint(Point{x:1,y:0,k:2,f:0,l:'L'});
    piece0.addpoint(Point{x:2,y:0,k:2,f:1,l:'L'});
    piece0.addpoint(Point{x:3,y:0,k:1,f:0,l:'L'});
    piece0.addpoint(Point{x:4,y:0,k:1,f:1,l:'L'});
    //piece0.neighbors_update(0,0);
    pieces.push(piece0);
    // T-Piece
    let mut piece1:Piece = Piece::new(String::from("T-Piece")
        ,5,2,2
        ,vec![Rotates::N,Rotates::Y,Rotates::R,Rotates::RX]
        ,'T');
    piece1.addpoint(Point{x:0,y:0,k:1,f:1,l:'T'});
    piece1.addpoint(Point{x:1,y:0,k:2,f:0,l:'T'});
    piece1.addpoint(Point{x:2,y:0,k:1,f:1,l:'T'});
    piece1.addpoint(Point{x:1,y:1,k:2,f:1,l:'T'});
    piece1.addpoint(Point{x:1,y:2,k:1,f:0,l:'T'});
    pieces.push(piece1);
    // Corner
    let rotates_all=vec![Rotates::N,Rotates::X,Rotates::Y,Rotates::XY,Rotates::R,Rotates::RX,Rotates::RY,Rotates::RXY];
    let mut piece2:Piece = Piece::new(String::from("Corner")
        ,5,2,2
        ,rotates_all
        ,'C');
    piece2.addpoint(Point{x:0,y:0,k:1,f:0,l:'C'});
    piece2.addpoint(Point{x:1,y:0,k:2,f:1,l:'C'});
    piece2.addpoint(Point{x:2,y:0,k:2,f:0,l:'C'});
    piece2.addpoint(Point{x:0,y:1,k:2,f:1,l:'C'});
    piece2.addpoint(Point{x:0,y:2,k:1,f:0,l:'C'});
    pieces.push(piece2);
    // Haken
    let rotates_all=vec![Rotates::N,Rotates::X,Rotates::Y,Rotates::XY,Rotates::R,Rotates::RX,Rotates::RY,Rotates::RXY];
    let mut piece3:Piece = Piece::new(String::from("Haken")
        ,5,1,3
        ,rotates_all
        ,'H');
    piece3.addpoint(Point{x:0,y:0,k:2,f:0,l:'H'});
    piece3.addpoint(Point{x:1,y:0,k:1,f:1,l:'H'});
    piece3.addpoint(Point{x:0,y:1,k:2,f:1,l:'H'});
    piece3.addpoint(Point{x:0,y:2,k:2,f:0,l:'H'});
    piece3.addpoint(Point{x:0,y:3,k:1,f:1,l:'H'});
    pieces.push(piece3);
    // Rectangle
    let rotates_all=vec![Rotates::N,Rotates::X,Rotates::Y,Rotates::XY,Rotates::R,Rotates::RX,Rotates::RY,Rotates::RXY];
    let mut piece4:Piece = Piece::new(String::from("Rectangle")
        ,5,1,2
        ,rotates_all
        ,'R');
    piece4.addpoint(Point{x:0,y:0,k:2,f:0,l:'R'});
    piece4.addpoint(Point{x:1,y:0,k:2,f:1,l:'R'});
    piece4.addpoint(Point{x:0,y:1,k:2,f:1,l:'R'});
    piece4.addpoint(Point{x:1,y:1,k:2,f:0,l:'R'});
    piece4.addpoint(Point{x:0,y:2,k:1,f:0,l:'R'});
    pieces.push(piece4);
    // Snake
    let mut piece5:Piece = Piece::new(String::from("Snake")
        ,5,2,2
        ,vec![Rotates::N,Rotates::X,Rotates::R,Rotates::RX],'S');
    piece5.addpoint(Point{x:0,y:0,k:1,f:0,l:'S'});
    piece5.addpoint(Point{x:1,y:0,k:2,f:1,l:'S'});
    piece5.addpoint(Point{x:1,y:1,k:2,f:0,l:'S'});
    piece5.addpoint(Point{x:1,y:2,k:2,f:1,l:'S'});
    piece5.addpoint(Point{x:2,y:2,k:1,f:0,l:'S'});
    pieces.push(piece5);
    // Star
    let mut piece6:Piece = Piece::new(String::from("Star")
        ,5,2,2
        ,vec![Rotates::N],'A');
    piece6.addpoint(Point{x:1,y:0,k:1,f:0,l:'A'});
    piece6.addpoint(Point{x:0,y:1,k:1,f:0,l:'A'});
    piece6.addpoint(Point{x:1,y:1,k:4,f:1,l:'A'});
    piece6.addpoint(Point{x:2,y:1,k:1,f:0,l:'A'});
    piece6.addpoint(Point{x:1,y:2,k:1,f:0,l:'A'});
    pieces.push(piece6);
    // 4x1
    let mut piece7:Piece = Piece::new(String::from("4x1")
        ,4,1,1
        ,vec![Rotates::N,Rotates::X41],'X');
    piece7.addpoint(Point{x:0,y:0,k:2,f:0,l:'X'});
    piece7.addpoint(Point{x:1,y:0,k:2,f:1,l:'X'});
    piece7.addpoint(Point{x:0,y:1,k:2,f:1,l:'X'});
    piece7.addpoint(Point{x:1,y:1,k:2,f:0,l:'X'});
    pieces.push(piece7);
    // Tor
    let rotates_all=vec![Rotates::N,Rotates::X,Rotates::Y,Rotates::XY,Rotates::R,Rotates::RX,Rotates::RY,Rotates::RXY];
    let mut piece8:Piece = Piece::new(String::from("Tor")
        ,5,2,1
        ,rotates_all
        ,'O');
    piece8.addpoint(Point{x:0,y:0,k:2,f:0,l:'O'});
    piece8.addpoint(Point{x:1,y:0,k:2,f:1,l:'O'});
    piece8.addpoint(Point{x:2,y:0,k:2,f:0,l:'O'});
    piece8.addpoint(Point{x:0,y:1,k:1,f:1,l:'O'});
    piece8.addpoint(Point{x:2,y:1,k:1,f:1,l:'O'});
    pieces.push(piece8);
    // Men
    let rotates_all=vec![Rotates::N,Rotates::X,Rotates::Y,Rotates::XY,Rotates::R,Rotates::RX,Rotates::RY,Rotates::RXY];
    let mut piece9:Piece = Piece::new(String::from("Men")
        ,5,1,3
        ,rotates_all
        ,'M');
    piece9.addpoint(Point{x:0,y:0,k:1,f:1,l:'M'});
    piece9.addpoint(Point{x:0,y:1,k:3,f:0,l:'M'});
    piece9.addpoint(Point{x:0,y:2,k:2,f:1,l:'M'});
    piece9.addpoint(Point{x:0,y:3,k:1,f:0,l:'M'});
    piece9.addpoint(Point{x:1,y:1,k:1,f:1,l:'M'});
    pieces.push(piece9);
    // ZickZack
    let rotates_all=vec![Rotates::N,Rotates::X,Rotates::Y,Rotates::XY,Rotates::R,Rotates::RX,Rotates::RY,Rotates::RXY];
    let mut piece10:Piece = Piece::new(String::from("ZickZack")
        ,5,1,3
        ,rotates_all
        ,'Z');
    piece10.addpoint(Point{x:0,y:0,k:1,f:0,l:'Z'});
    piece10.addpoint(Point{x:0,y:1,k:2,f:1,l:'Z'});
    piece10.addpoint(Point{x:1,y:1,k:2,f:0,l:'Z'});
    piece10.addpoint(Point{x:1,y:2,k:2,f:1,l:'Z'});
    piece10.addpoint(Point{x:1,y:3,k:1,f:0,l:'Z'});
    pieces.push(piece10);
    // Staicase
    let rotates_all=vec![Rotates::N,Rotates::X,Rotates::Y,Rotates::XY,Rotates::R,Rotates::RX,Rotates::RY,Rotates::RXY];
    let mut piece11:Piece = Piece::new(String::from("Staircase")
        ,5,2,2
        ,rotates_all
        ,'i');
    piece11.addpoint(Point{x:0,y:0,k:1,f:1,l:'i'});
    piece11.addpoint(Point{x:1,y:1,k:2,f:1,l:'i'});
    piece11.addpoint(Point{x:2,y:2,k:1,f:1,l:'i'});
    piece11.addpoint(Point{x:0,y:1,k:2,f:0,l:'i'});
    piece11.addpoint(Point{x:1,y:2,k:2,f:0,l:'i'});
    pieces.push(piece11);
    // Lastone
    let rotates_all=vec![Rotates::N,Rotates::X,Rotates::Y,Rotates::XY,Rotates::R,Rotates::RX,Rotates::RY,Rotates::RXY];
    let mut piece12:Piece = Piece::new(String::from("Lastone")
        ,5,2,2
        ,rotates_all
        ,'E');
    piece12.addpoint(Point{x:0,y:0,k:1,f:0,l:'E'});
    piece12.addpoint(Point{x:1,y:0,k:2,f:1,l:'E'});
    piece12.addpoint(Point{x:1,y:1,k:3,f:0,l:'E'});
    piece12.addpoint(Point{x:1,y:2,k:1,f:1,l:'E'});
    piece12.addpoint(Point{x:2,y:1,k:1,f:1,l:'E'});
    pieces.push(piece12);
    
    return Pieces{vecs:pieces}
}