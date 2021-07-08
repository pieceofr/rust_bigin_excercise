fn main() {
    println!("Hello, world!");
    let origin_point = Point { x: 0, y: 0 };
    println!("origin point is ({},{})", origin_point.x, origin_point.y);

    let mut mov_point = Point { x: 0, y: 0 };
    mov_point.x = 5;
    mov_point.y = 10;
    println!("move to ({},{})", mov_point.x, mov_point.y);
    mov_point.x = 3;
    // make y field immutable again
    //let mov_point = mov_point; // rebind mov_point make it immutable again so y can not be changed
    //mov_point.y = 7; // Error, because we make mov_point immutable.

    // Use Point Reference struc

    let mut my_point = Point { x: 0, y: 0 };
    {
        let r = PointRef {
            x: &mut my_point.x,
            y: &mut my_point.y,
        };
        *r.x = 5;
        *r.y = 6;
    }
    assert_eq!(5, my_point.x);
    assert_eq!(6, my_point.y);
    // Update value
    let mut point3d = Point3D { x: 0, y: 0, z: 0 };
    // Change only y, use .. to copy value of x and z
    point3d = Point3D { y: 1, ..point3d };
    // Also can change value by a new variable
    let pt3change = Point3D {
        z: 1,
        y: 2,
        ..point3d
    };
    println!(
        "pt3change (x,y,z)=({},{},{})",
        pt3change.x, pt3change.y, pt3change.z,
    );
}

struct Point {
    x: i32,
    y: i32,
}

struct PointRef<'a> {
    x: &'a mut i32,
    y: &'a mut i32,
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}
