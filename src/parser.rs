use nom::{
    bytes::complete::{take_until, tag},
    character::complete::{space0, char},
    combinator::{map, verify},
    sequence::{separated_pair, tuple, terminated},
    IResult, multi::many1,
};

#[inline]
fn single_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until("\n")(input)
}

#[inline]
fn key_name(input: &[u8]) -> IResult<&[u8], &[u8]> {
    verify(take_until(":"), |input: &[u8]| {
        if !input.is_empty() {
            input[0] != b'\n'
        } else {
            false
        }
    })(input)
}

#[inline]
fn separator(input: &[u8]) -> IResult<&[u8], ()> {
    map(tuple((char(':'), space0)), |_| ())(input)
}

#[inline]
fn key_value(input: &[u8]) -> IResult<&[u8], (&[u8], &[u8])> {
    separated_pair(key_name, separator, single_line)(input)
}

#[inline]
pub fn single_package(input: &[u8]) -> IResult<&[u8], Vec<(&[u8], &[u8])>> {
    many1(terminated(key_value, tag("\n")))(input)
}

#[test]
fn test_package() {
    let test = &b"Package: zsync\nVersion: 0.6.2-1\nStatus: install ok installed\nArchitecture: amd64\nInstalled-Size: 256\n\n"[..];
    assert_eq!(
        single_package(test),
        Ok((
            &b"\n"[..],
            vec![
                (&b"Package"[..], &b"zsync"[..]),
                (&b"Version"[..], &b"0.6.2-1"[..]),
                (&b"Status"[..], &b"install ok installed"[..]),
                (&b"Architecture"[..], &b"amd64"[..]),
                (&b"Installed-Size"[..], &b"256"[..])
            ]
        ))
    );
}
