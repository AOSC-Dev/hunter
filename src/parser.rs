use nom::{
    bytes::complete::{tag, take_until, take_while},
    character::complete::{char, space0},
    combinator::{map, verify},
    multi::{many0, many1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

type SinglePackageResult<'a> = IResult<&'a [u8], Vec<(&'a [u8], &'a [u8])>>;

#[inline]
fn single_line(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take_until("\n")(input)
}

// drop deb811 multi line info (e.g: Conffiles)
fn drop_multi_line(input: &[u8]) -> IResult<&[u8], ()> {
    map(
        many1(terminated(
            tag(" "),
            tuple((take_until("\n"), take_while(|c| c == b'\n'))),
        )),
        |_| {},
    )(input)
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
fn hr(input: &[u8]) -> IResult<&[u8], ()> {
    map(many0(drop_multi_line), |_| ())(input)
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
fn single_package(input: &[u8]) -> SinglePackageResult {
    many1(preceded(hr, terminated(key_value, tag("\n"))))(input)
}

#[inline]
pub fn all_packages(input: &[u8]) -> IResult<&[u8], Vec<PackageCtx>> {
    many1(terminated(extract_package, tag("\n")))(input)
}

fn extract_name(input: &[u8]) -> IResult<&[u8], &[u8]> {
    let info = single_package(input)?;

    let name = info.1.iter().find(|(x, _)| x == b"Package").map(|(_, y)| y);

    Ok((info.0, name.unwrap()))
}

pub struct PackageCtx<'a> {
    pub package: &'a [u8],
    pub status: &'a [u8],
    pub version: &'a [u8],
    pub desc: &'a [u8],
}

fn extract_package(input: &[u8]) -> IResult<&[u8], PackageCtx> {
    let info = single_package(input)?;

    let name = info.1.iter().find(|(x, _)| x == b"Package").map(|(_, y)| y);
    let status = info.1.iter().find(|(x, _)| x == b"Status").map(|(_, y)| y);
    let version = info.1.iter().find(|(x, _)| x == b"Version").map(|(_, y)| y);
    let desc = info
        .1
        .iter()
        .find(|(x, _)| x == b"Description")
        .map(|(_, y)| y);

    Ok((
        info.0,
        PackageCtx {
            package: name.unwrap(),
            status: status.unwrap(),
            version: version.unwrap(),
            desc: desc.unwrap(),
        },
    ))
}

#[inline]
pub fn extract_all_names(input: &[u8]) -> IResult<&[u8], Vec<&[u8]>> {
    many1(terminated(extract_name, tag("\n")))(input)
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

#[test]
fn test_multi_package() {
    let test =
        &b"Package: zsync\nStatus: b\n\nPackage: rsync\nStatus: install ok installed\n\n"[..];

    dbg!(extract_all_names(test).unwrap().1);

    assert_eq!(
        extract_all_names(test),
        Ok((&b""[..], vec![&b"zsync"[..], &b"rsync"[..]]))
    );
}
