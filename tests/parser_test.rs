use auto_domain_blocker::parser::read_section;

#[test]
fn test_read_section() {
    let input = "Normal text

## <!-- auto domain blocker --->
text A
text B

text C
## <!-- auto domain blocker --->

trailer text";

    let want = "text A
text B

text C
";

    assert_eq!(read_section(input), want);
}
