extern crate tempdir;
extern crate gpgme;

use gpgme::{Context, Data};

use self::support::{check_data, passphrase_cb, setup};

#[macro_use]
mod support;

const CIPHER_1: &'static [u8] = include_bytes!("./data/cipher-1.asc");

#[test]
fn test_decrypt() {
    let _gpghome = setup();
    let mut ctx = fail_if_err!(Context::from_protocol(gpgme::Protocol::OpenPgp));
    ctx.with_passphrase_provider(passphrase_cb, |mut ctx| {
        let mut input = fail_if_err!(Data::from_buffer(CIPHER_1));
        let mut output = fail_if_err!(Data::new());
        match fail_if_err!(ctx.decrypt(&mut input, &mut output)).unsupported_algorithm() {
            Ok(alg) => panic!("unsupported algorithm: {}", alg),
            Err(Some(_)) => panic!("unsupported algorithm"),
            _ => {}
        }
        check_data(&mut output,
                   b"Wenn Sie dies lesen k\xf6nnen, ist es wohl nicht\n\
                geheim genug.\n");
    });
}
