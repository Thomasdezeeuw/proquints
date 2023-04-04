use proquints::proquints;

#[test]
fn simple() {
    // Test taken from the examples from spec.
    let tests = [
        ([127, 0, 0, 1], "lusab-babad"),
        ([63, 84, 220, 193], "gutih-tugad"),
        ([63, 118, 7, 35], "gutuk-bisog"),
        ([140, 98, 193, 141], "mudof-sakat"),
        ([64, 255, 6, 200], "haguz-biram"),
        ([128, 30, 52, 45], "mabiv-gibot"),
        ([147, 67, 119, 2], "natag-lisaf"),
        ([212, 58, 253, 68], "tibup-zujah"),
        ([216, 35, 68, 215], "tobog-higil"),
        ([216, 68, 232, 21], "todah-vobij"),
        ([198, 81, 129, 136], "sinid-makam"),
        ([12, 110, 110, 204], "budov-kuras"),
    ];

    for (input, expected) in tests {
        let got = proquints(input.as_slice());
        assert_eq!(got, expected, "input: {input:?}");
    }
}
