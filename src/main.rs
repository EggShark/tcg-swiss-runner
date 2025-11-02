use player::Player;

mod player;
mod swiss;

fn main() {
    let p1 = Player::new("Bob".to_string(), 1);
    let p2 = Player::new("Alice".to_string(), 2);
    let p3 = Player::new("Carol".to_string(), 3);
    let p4 = Player::new("Carlos".to_string(), 4);

    let mut players = vec![p1, p2, p3, p4];
}   
