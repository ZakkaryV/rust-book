use lib::content::{NewsArticle, Tweet};

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, pt: Point<V, W>) -> Point<T, W> {
        Point { x: self.x, y: pt.y }
    }
}

// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

fn main() {
    // let list_i32 = vec![100, 242, 435, 852, 120, 529, 129];
    // println!("Largest number: {}", largest(&list_i32));

    // let list_char = vec!['a', 'z', 'b', 'y', 'c'];
    // println!("Largest character: {}", largest(&list_char));

    // let pt1 = Point { x: 10, y: 'c' };
    // let pt2 = Point {
    //     x: "Hi!!!!",
    //     y: 6.66,
    // };

    // let pt3 = pt1.mixup(pt2);
    // println!("pt3: {:?}", pt3);
}
