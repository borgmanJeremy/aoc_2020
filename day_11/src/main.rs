enum SeatType {
    FLOOR,
    EMPTY,
    FULL,
}

fn main() {
    println!("Hello, world!");
}

/*

    If a seat is empty (L) and there are no occupied seats adjacent to it, the seat becomes occupied.
    If a seat is occupied (#) and four or more seats adjacent to it are also occupied, the seat becomes empty.
    Otherwise, the seat's state does not change.

Floor (.) never changes; seats don't move, and nobody sits on the floor.
 */