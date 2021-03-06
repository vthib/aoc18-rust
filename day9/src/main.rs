use std::error::Error;
use linked_list::{Cursor, LinkedList};

type Result<T> = std::result::Result<T, Box<Error>>;

fn main() -> Result<()> {
    const FINAL_MARBLE: usize = 71250;
    const NB_PLAYERS: usize = 452;

    println!("day9, part1: high score is {}", play_game(FINAL_MARBLE, NB_PLAYERS));
    println!("day9, part2: high score is {}", play_game(FINAL_MARBLE * 100, NB_PLAYERS));
    Ok(())
}

fn play_game(final_marble: usize, nb_players: usize) -> usize {
    let mut list = LinkedList::new();
    list.push_front(0);
    let mut cursor = list.cursor();

    let mut next_marble = 1;
    let mut current_player = 0;

    let mut scores = vec![0; nb_players];

    while next_marble <= final_marble {
        if next_marble % 23 == 0 {
            go_backward(&mut cursor, 7);
            let removed_marble = cursor.remove().unwrap();
            // let prev = cursor.peek_prev();
            // println!("removing {} between {:?}", removed_marble, prev);
            // let next = cursor.peek_next();
            // println!("and {:?}", next);
            scores[current_player] += next_marble + removed_marble;
        } else {
            go_forward(&mut cursor, 2);
            // let prev = cursor.peek_prev();
            // println!("adding {} between {:?}", next_marble, prev);
            // let next = cursor.peek_next();
            // println!("and {:?}", next);
            cursor.insert(next_marble);
        }
        next_marble += 1;
        current_player = (current_player + 1) % nb_players;
    }

    *scores.iter().max().unwrap()
}

/* The linked_list crate provides a cursor over a linked list, however the cursor does not handle
 * the looping seamlessly: when looping back to the "start" of the list, next/prev will return
 * None, and need to be called again to return the right node.
 * Those functions handle this case transparently.
 */

fn go_forward<'a>(cursor: &mut Cursor<'a, usize>, n: usize) {
    let mut i = n;

    while i > 0 {
        if !cursor.next().is_none() {
            i -= 1;
        }
    }
}

fn go_backward<'a>(cursor: &mut Cursor<'a, usize>, n: usize) {
    let mut i = n;

    while i > 0 {
        if !cursor.prev().is_none() {
            i -= 1;
        }
    }
}
