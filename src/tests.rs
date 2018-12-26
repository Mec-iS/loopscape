use loopscope::shapes::Shape;

#[test]
fn test_shape_build() {
    let name = String::from("test");
    let pattern = String::from("-.P");
    let sides: u32 = 4;

    let shp: Shape = Shape::build(
        name,
        pattern,
        sides
    );

    assert_eq!(shp.name, String::from("test"));
}