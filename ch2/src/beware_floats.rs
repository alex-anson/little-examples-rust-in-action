pub fn beware_floats() {
    let thirty_two: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let sixty_four: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("thirty_two");
    println!("0.1 + 0.2: {:x}", (thirty_two.0 + thirty_two.1).to_bits());
    println!("      0.3: {:x}", thirty_two.2.to_bits());
    println!();

    println!("sixty_four");
    println!("0.1 + 0.2: {:x}", (sixty_four.0 + sixty_four.1).to_bits());
    println!("      0.3: {:x}", sixty_four.2.to_bits());
    println!();

    assert!(thirty_two.0 + thirty_two.1 == thirty_two.2);
    // assert!(sixty_four.0 + sixty_four.1 == sixty_four.2); // crashes the program with panic

    // safer to test whether a float falls within an acceptable margin of its
    // true mathematical result. this margin = the epsilon.
    let result: f64 = 0.1 + 0.2;
    let desired: f64 = 0.3;
    let absolute_difference = (desired - result).abs();
    assert!(absolute_difference <= f64::EPSILON);
}
