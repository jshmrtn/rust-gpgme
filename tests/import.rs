extern crate tempdir;
extern crate gpgme;

use gpgme::{Context, Data};

use self::support::setup;

#[macro_use]
mod support;

const PUBKEY_1: &'static [u8] = include_bytes!("./data/pubkey-1.asc");
const SECKEY_1: &'static [u8] = include_bytes!("./data/seckey-1.asc");

const FINGERPRINT: &'static str = "ADAB7FCC1F4DE2616ECFA402AF82244F9CD9FD55";

fn check_result(result: gpgme::ImportResult, secret: bool) {
    if !secret {
        assert_eq!(result.considered(), 1);
    } else if result.considered() != 3 {
        assert_eq!(result.considered(), 1);
    }
    assert_eq!(result.without_user_id(), 0);
    if secret {
        assert_eq!(result.imported(), 0);
    } else if result.imported() != 0 {
        assert_eq!(result.imported(), 1);
    }
    assert_eq!(result.imported_rsa(), 0);
    if secret {
        if result.unchanged() != 0 {
            assert_eq!(result.unchanged(), 1);
        }
    } else if result.imported() > 0 {
        assert_eq!(result.unchanged(), 0);
    } else {
        assert_eq!(result.unchanged(), 1);
    }
    assert_eq!(result.new_user_ids(), 0);
    assert_eq!(result.new_subkeys(), 0);
    assert_eq!(result.new_revocations(), 0);

    if !secret {
        assert_eq!(result.secret_considered(), 0);
    } else if result.secret_considered() != 3 {
        assert_eq!(result.secret_considered(), 1);
    }
    if !secret || ((result.secret_imported() != 1) && (result.secret_imported() != 2)) {
        assert_eq!(result.secret_imported(), 0);
    }
    if !secret {
        assert_eq!(result.secret_unchanged(), 0);
    } else if result.secret_imported() > 0 {
        assert_eq!(result.secret_unchanged(), 0);
    } else if result.secret_unchanged() != 1 {
        assert_eq!(result.secret_unchanged(), 2);
    }
    assert_eq!(result.not_imported(), 0);

    let filter_imports = |p: &gpgme::Import| -> bool {
        p.status().contains(gpgme::IMPORT_NEW) || p.status().contains(gpgme::IMPORT_SIG) ||
        p.status().contains(gpgme::IMPORT_UID)
    };

    let count = result.imports().filter(&filter_imports).count();
    if !secret || (count != 2) {
        assert_eq!(count, 1);
    }

    for import in result.imports().filter(&filter_imports) {
        assert_eq!(import.fingerprint(), Ok(FINGERPRINT));
        assert_eq!(import.result(), Ok(()));
    }
}

#[test]
fn test_import() {
    let _gpghome = setup();
    let mut ctx = fail_if_err!(Context::from_protocol(gpgme::Protocol::OpenPgp));

    let mut input = fail_if_err!(Data::from_buffer(PUBKEY_1));
    check_result(fail_if_err!(ctx.import(&mut input)), false);

    input = fail_if_err!(Data::from_buffer(SECKEY_1));
    check_result(fail_if_err!(ctx.import(&mut input)), true);
}
