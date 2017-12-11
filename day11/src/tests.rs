use get_shortest_dist;

#[test]
fn test_get_shortest_dist1() {
    let input = "ne,ne,ne";
    assert_eq!(Some((3, 3)), get_shortest_dist(input));
}

#[test]
fn test_get_shortest_dist2() {
    let input = "ne,ne,sw,sw";
    assert_eq!(Some((0, 2)), get_shortest_dist(input));
}

#[test]
fn test_get_shortest_dist3() {
    let input = "ne,ne,s,s";
    assert_eq!(Some((2, 2)), get_shortest_dist(input));
}

#[test]
fn test_get_shortest_dist4() {
    let input = "se,sw,se,sw,sw";
    assert_eq!(Some((3, 3)), get_shortest_dist(input));
}
