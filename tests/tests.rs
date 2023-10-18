use insta::assert_debug_snapshot;

use pep_508::parse;

#[test]
fn basic() {
    assert_debug_snapshot!(parse("requests"));
    assert_debug_snapshot!(parse("beautifulsoup4"));
    assert_debug_snapshot!(parse("Click>=7.0"));
    assert_debug_snapshot!(parse("tomli;python_version<'3.11'"));

    assert!(parse("").is_err());
    assert!(parse("# comment").is_err())
}

// tests from the PEP
#[test]
fn pep() {
    assert_debug_snapshot!(parse("A"));
    assert_debug_snapshot!(parse("A.B-C_D"));
    assert_debug_snapshot!(parse("aa"));
    assert_debug_snapshot!(parse("name"));
    assert_debug_snapshot!(parse("name<=1"));
    assert_debug_snapshot!(parse("name>=4"));
    assert_debug_snapshot!(parse("name>=3,<2"));
    assert_debug_snapshot!(parse("name@http://foo.com"));
    assert_debug_snapshot!(parse(
        "name [fred,bar] @ http://foo.com ; python_version=='3.7'"
    ));
    assert_debug_snapshot!(parse(
        "name[quux, strange];python_version<'3.7' and platform_version=='2'"
    ));
    assert_debug_snapshot!(parse("name; os_name=='a' or os_name=='b'"));
    assert_debug_snapshot!(parse("name; os_name=='a' and os_name=='b' or os_name=='c'"));
    assert_debug_snapshot!(parse(
        "name; os_name=='a' and (os_name=='b' or os_name=='c')"
    ));
    assert_debug_snapshot!(parse("name; os_name=='a' or os_name=='b' and os_name=='c'"));
    assert_debug_snapshot!(parse(
        "name; (os_name=='a' or os_name=='b') and os_name=='c'"
    ));
}

// additional tests, based on https://pip.pypa.io/en/stable/topics/secure-installs/#hash-checking-mode
#[test]
fn additional() {
    assert_debug_snapshot!(parse("FooProject == 1.2 \
  --hash=sha256:2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824 \
  --hash=sha256:486ea46224d1bb4fb680f34f7c9ad96a8f24ec88be73ea8e5a6c65260e9cb8a7"));
}
