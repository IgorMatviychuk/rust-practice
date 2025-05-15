struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

fn calculate_area(rect: &Rectangle) -> i32 {
    let width = (rect.b.x - rect.a.x).abs();
    let height = (rect.a.y - rect.b.y).abs();
    width * height
}

fn get_overlap(rect1: &Rectangle, rect2: &Rectangle) -> Option<(i32, i32, i32, i32)> {
    let x1 = rect1.a.x.max(rect2.a.x);
    let y1 = rect1.b.y.max(rect2.b.y);
    let x2 = rect1.b.x.min(rect2.b.x);
    let y2 = rect1.a.y.min(rect2.a.y);

    if x1 < x2 && y1 < y2 {
        Some((x1, y1, x2, y2))
    } else {
        None
    }
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;
    let mut overlap_area = 0;

    for rect in xs {
        total_area += calculate_area(rect);
    }

    for i in 0..xs.len() {
        for j in (i + 1)..xs.len() {
            if let Some((x1, y1, x2, y2)) = get_overlap(&xs[i], &xs[j]) {
                let overlap_width = x2 - x1;
                let overlap_height = y2 - y1;
                overlap_area += overlap_width * overlap_height;
            }
        }
    }

    total_area - overlap_area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

#[test]
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}
