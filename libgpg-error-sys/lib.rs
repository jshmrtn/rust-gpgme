#![allow(non_camel_case_types)]

pub use self::types::*;
pub use self::consts::*;
pub use self::funcs::*;

pub mod types {
    extern crate libc;

    pub type gpg_error_t = libc::c_uint;
    pub type gpg_err_source_t = libc::c_uint;
    pub type gpg_err_code_t = libc::c_uint;
}

pub mod consts {
    use types::gpg_error_t;
    use types::gpg_err_source_t;
    use types::gpg_err_code_t;

    pub const GPG_ERR_SOURCE_UNKNOWN: gpg_err_source_t = 0;
    pub const GPG_ERR_SOURCE_GCRYPT: gpg_err_source_t = 1;
    pub const GPG_ERR_SOURCE_GPG: gpg_err_source_t = 2;
    pub const GPG_ERR_SOURCE_GPGSM: gpg_err_source_t = 3;
    pub const GPG_ERR_SOURCE_GPGAGENT: gpg_err_source_t = 4;
    pub const GPG_ERR_SOURCE_PINENTRY: gpg_err_source_t = 5;
    pub const GPG_ERR_SOURCE_SCD: gpg_err_source_t = 6;
    pub const GPG_ERR_SOURCE_GPGME: gpg_err_source_t = 7;
    pub const GPG_ERR_SOURCE_KEYBOX: gpg_err_source_t = 8;
    pub const GPG_ERR_SOURCE_KSBA: gpg_err_source_t = 9;
    pub const GPG_ERR_SOURCE_DIRMNGR: gpg_err_source_t = 10;
    pub const GPG_ERR_SOURCE_GSTI: gpg_err_source_t = 11;
    pub const GPG_ERR_SOURCE_GPA: gpg_err_source_t = 12;
    pub const GPG_ERR_SOURCE_KLEO: gpg_err_source_t = 13;
    pub const GPG_ERR_SOURCE_G13: gpg_err_source_t = 14;
    pub const GPG_ERR_SOURCE_ASSUAN: gpg_err_source_t = 15;
    pub const GPG_ERR_SOURCE_TLS: gpg_err_source_t = 17;
    pub const GPG_ERR_SOURCE_ANY: gpg_err_source_t = 31;
    pub const GPG_ERR_SOURCE_USER_1: gpg_err_source_t = 32;
    pub const GPG_ERR_SOURCE_USER_2: gpg_err_source_t = 33;
    pub const GPG_ERR_SOURCE_USER_3: gpg_err_source_t = 34;
    pub const GPG_ERR_SOURCE_USER_4: gpg_err_source_t = 35;
    pub const GPG_ERR_SOURCE_DIM: gpg_err_source_t = 128;

    pub const GPG_ERR_SYSTEM_ERROR: gpg_err_code_t = 1 << 15;
    pub const GPG_ERR_NO_ERROR: gpg_err_code_t = 0;
    pub const GPG_ERR_GENERAL: gpg_err_code_t = 1;
    pub const GPG_ERR_UNKNOWN_PACKET: gpg_err_code_t = 2;
    pub const GPG_ERR_UNKNOWN_VERSION: gpg_err_code_t = 3;
    pub const GPG_ERR_PUBKEY_ALGO: gpg_err_code_t = 4;
    pub const GPG_ERR_DIGEST_ALGO: gpg_err_code_t = 5;
    pub const GPG_ERR_BAD_PUBKEY: gpg_err_code_t = 6;
    pub const GPG_ERR_BAD_SECKEY: gpg_err_code_t = 7;
    pub const GPG_ERR_BAD_SIGNATURE: gpg_err_code_t = 8;
    pub const GPG_ERR_NO_PUBKEY: gpg_err_code_t = 9;
    pub const GPG_ERR_CHECKSUM: gpg_err_code_t = 10;
    pub const GPG_ERR_BAD_PASSPHRASE: gpg_err_code_t = 11;
    pub const GPG_ERR_CIPHER_ALGO: gpg_err_code_t = 12;
    pub const GPG_ERR_KEYRING_OPEN: gpg_err_code_t = 13;
    pub const GPG_ERR_INV_PACKET: gpg_err_code_t = 14;
    pub const GPG_ERR_INV_ARMOR: gpg_err_code_t = 15;
    pub const GPG_ERR_NO_USER_ID: gpg_err_code_t = 16;
    pub const GPG_ERR_NO_SECKEY: gpg_err_code_t = 17;
    pub const GPG_ERR_WRONG_SECKEY: gpg_err_code_t = 18;
    pub const GPG_ERR_BAD_KEY: gpg_err_code_t = 19;
    pub const GPG_ERR_COMPR_ALGO: gpg_err_code_t = 20;
    pub const GPG_ERR_NO_PRIME: gpg_err_code_t = 21;
    pub const GPG_ERR_NO_ENCODING_METHOD: gpg_err_code_t = 22;
    pub const GPG_ERR_NO_ENCRYPTION_SCHEME: gpg_err_code_t = 23;
    pub const GPG_ERR_NO_SIGNATURE_SCHEME: gpg_err_code_t = 24;
    pub const GPG_ERR_INV_ATTR: gpg_err_code_t = 25;
    pub const GPG_ERR_NO_VALUE: gpg_err_code_t = 26;
    pub const GPG_ERR_NOT_FOUND: gpg_err_code_t = 27;
    pub const GPG_ERR_VALUE_NOT_FOUND: gpg_err_code_t = 28;
    pub const GPG_ERR_SYNTAX: gpg_err_code_t = 29;
    pub const GPG_ERR_BAD_MPI: gpg_err_code_t = 30;
    pub const GPG_ERR_INV_PASSPHRASE: gpg_err_code_t = 31;
    pub const GPG_ERR_SIG_CLASS: gpg_err_code_t = 32;
    pub const GPG_ERR_RESOURCE_LIMIT: gpg_err_code_t = 33;
    pub const GPG_ERR_INV_KEYRING: gpg_err_code_t = 34;
    pub const GPG_ERR_TRUSTDB: gpg_err_code_t = 35;
    pub const GPG_ERR_BAD_CERT: gpg_err_code_t = 36;
    pub const GPG_ERR_INV_USER_ID: gpg_err_code_t = 37;
    pub const GPG_ERR_UNEXPECTED: gpg_err_code_t = 38;
    pub const GPG_ERR_TIME_CONFLICT: gpg_err_code_t = 39;
    pub const GPG_ERR_KEYSERVER: gpg_err_code_t = 40;
    pub const GPG_ERR_WRONG_PUBKEY_ALGO: gpg_err_code_t = 41;
    pub const GPG_ERR_TRIBUTE_TO_D_A: gpg_err_code_t = 42;
    pub const GPG_ERR_WEAK_KEY: gpg_err_code_t = 43;
    pub const GPG_ERR_INV_KEYLEN: gpg_err_code_t = 44;
    pub const GPG_ERR_INV_ARG: gpg_err_code_t = 45;
    pub const GPG_ERR_BAD_URI: gpg_err_code_t = 46;
    pub const GPG_ERR_INV_URI: gpg_err_code_t = 47;
    pub const GPG_ERR_NETWORK: gpg_err_code_t = 48;
    pub const GPG_ERR_UNKNOWN_HOST: gpg_err_code_t = 49;
    pub const GPG_ERR_SELFTEST_FAILED: gpg_err_code_t = 50;
    pub const GPG_ERR_NOT_ENCRYPTED: gpg_err_code_t = 51;
    pub const GPG_ERR_NOT_PROCESSED: gpg_err_code_t = 52;
    pub const GPG_ERR_UNUSABLE_PUBKEY: gpg_err_code_t = 53;
    pub const GPG_ERR_UNUSABLE_SECKEY: gpg_err_code_t = 54;
    pub const GPG_ERR_INV_VALUE: gpg_err_code_t = 55;
    pub const GPG_ERR_BAD_CERT_CHAIN: gpg_err_code_t = 56;
    pub const GPG_ERR_MISSING_CERT: gpg_err_code_t = 57;
    pub const GPG_ERR_NO_DATA: gpg_err_code_t = 58;
    pub const GPG_ERR_BUG: gpg_err_code_t = 59;
    pub const GPG_ERR_NOT_SUPPORTED: gpg_err_code_t = 60;
    pub const GPG_ERR_INV_OP: gpg_err_code_t = 61;
    pub const GPG_ERR_TIMEOUT: gpg_err_code_t = 62;
    pub const GPG_ERR_INTERNAL: gpg_err_code_t = 63;
    pub const GPG_ERR_EOF_GCRYPT: gpg_err_code_t = 64;
    pub const GPG_ERR_INV_OBJ: gpg_err_code_t = 65;
    pub const GPG_ERR_TOO_SHORT: gpg_err_code_t = 66;
    pub const GPG_ERR_TOO_LARGE: gpg_err_code_t = 67;
    pub const GPG_ERR_NO_OBJ: gpg_err_code_t = 68;
    pub const GPG_ERR_NOT_IMPLEMENTED: gpg_err_code_t = 69;
    pub const GPG_ERR_CONFLICT: gpg_err_code_t = 70;
    pub const GPG_ERR_INV_CIPHER_MODE: gpg_err_code_t = 71;
    pub const GPG_ERR_INV_FLAG: gpg_err_code_t = 72;
    pub const GPG_ERR_INV_HANDLE: gpg_err_code_t = 73;
    pub const GPG_ERR_TRUNCATED: gpg_err_code_t = 74;
    pub const GPG_ERR_INCOMPLETE_LINE: gpg_err_code_t = 75;
    pub const GPG_ERR_INV_RESPONSE: gpg_err_code_t = 76;
    pub const GPG_ERR_NO_AGENT: gpg_err_code_t = 77;
    pub const GPG_ERR_AGENT: gpg_err_code_t = 78;
    pub const GPG_ERR_INV_DATA: gpg_err_code_t = 79;
    pub const GPG_ERR_ASSUAN_SERVER_FAULT: gpg_err_code_t = 80;
    pub const GPG_ERR_ASSUAN: gpg_err_code_t = 81;
    pub const GPG_ERR_INV_SESSION_KEY: gpg_err_code_t = 82;
    pub const GPG_ERR_INV_SEXP: gpg_err_code_t = 83;
    pub const GPG_ERR_UNSUPPORTED_ALGORITHM: gpg_err_code_t = 84;
    pub const GPG_ERR_NO_PIN_ENTRY: gpg_err_code_t = 85;
    pub const GPG_ERR_PIN_ENTRY: gpg_err_code_t = 86;
    pub const GPG_ERR_BAD_PIN: gpg_err_code_t = 87;
    pub const GPG_ERR_INV_NAME: gpg_err_code_t = 88;
    pub const GPG_ERR_BAD_DATA: gpg_err_code_t = 89;
    pub const GPG_ERR_INV_PARAMETER: gpg_err_code_t = 90;
    pub const GPG_ERR_WRONG_CARD: gpg_err_code_t = 91;
    pub const GPG_ERR_NO_DIRMNGR: gpg_err_code_t = 92;
    pub const GPG_ERR_DIRMNGR: gpg_err_code_t = 93;
    pub const GPG_ERR_CERT_REVOKED: gpg_err_code_t = 94;
    pub const GPG_ERR_NO_CRL_KNOWN: gpg_err_code_t = 95;
    pub const GPG_ERR_CRL_TOO_OLD: gpg_err_code_t = 96;
    pub const GPG_ERR_LINE_TOO_LONG: gpg_err_code_t = 97;
    pub const GPG_ERR_NOT_TRUSTED: gpg_err_code_t = 98;
    pub const GPG_ERR_CANCELED: gpg_err_code_t = 99;
    pub const GPG_ERR_BAD_CA_CERT: gpg_err_code_t = 100;
    pub const GPG_ERR_CERT_EXPIRED: gpg_err_code_t = 101;
    pub const GPG_ERR_CERT_TOO_YOUNG: gpg_err_code_t = 102;
    pub const GPG_ERR_UNSUPPORTED_CERT: gpg_err_code_t = 103;
    pub const GPG_ERR_UNKNOWN_SEXP: gpg_err_code_t = 104;
    pub const GPG_ERR_UNSUPPORTED_PROTECTION: gpg_err_code_t = 105;
    pub const GPG_ERR_CORRUPTED_PROTECTION: gpg_err_code_t = 106;
    pub const GPG_ERR_AMBIGUOUS_NAME: gpg_err_code_t = 107;
    pub const GPG_ERR_CARD: gpg_err_code_t = 108;
    pub const GPG_ERR_CARD_RESET: gpg_err_code_t = 109;
    pub const GPG_ERR_CARD_REMOVED: gpg_err_code_t = 110;
    pub const GPG_ERR_INV_CARD: gpg_err_code_t = 111;
    pub const GPG_ERR_CARD_NOT_PRESENT: gpg_err_code_t = 112;
    pub const GPG_ERR_NO_PKCS15_APP: gpg_err_code_t = 113;
    pub const GPG_ERR_NOT_CONFIRMED: gpg_err_code_t = 114;
    pub const GPG_ERR_CONFIGURATION: gpg_err_code_t = 115;
    pub const GPG_ERR_NO_POLICY_MATCH: gpg_err_code_t = 116;
    pub const GPG_ERR_INV_INDEX: gpg_err_code_t = 117;
    pub const GPG_ERR_INV_ID: gpg_err_code_t = 118;
    pub const GPG_ERR_NO_SCDAEMON: gpg_err_code_t = 119;
    pub const GPG_ERR_SCDAEMON: gpg_err_code_t = 120;
    pub const GPG_ERR_UNSUPPORTED_PROTOCOL: gpg_err_code_t = 121;
    pub const GPG_ERR_BAD_PIN_METHOD: gpg_err_code_t = 122;
    pub const GPG_ERR_CARD_NOT_INITIALIZED: gpg_err_code_t = 123;
    pub const GPG_ERR_UNSUPPORTED_OPERATION: gpg_err_code_t = 124;
    pub const GPG_ERR_WRONG_KEY_USAGE: gpg_err_code_t = 125;
    pub const GPG_ERR_NOTHING_FOUND: gpg_err_code_t = 126;
    pub const GPG_ERR_WRONG_BLOB_TYPE: gpg_err_code_t = 127;
    pub const GPG_ERR_MISSING_VALUE: gpg_err_code_t = 128;
    pub const GPG_ERR_HARDWARE: gpg_err_code_t = 129;
    pub const GPG_ERR_PIN_BLOCKED: gpg_err_code_t = 130;
    pub const GPG_ERR_USE_CONDITIONS: gpg_err_code_t = 131;
    pub const GPG_ERR_PIN_NOT_SYNCED: gpg_err_code_t = 132;
    pub const GPG_ERR_INV_CRL: gpg_err_code_t = 133;
    pub const GPG_ERR_BAD_BER: gpg_err_code_t = 134;
    pub const GPG_ERR_INV_BER: gpg_err_code_t = 135;
    pub const GPG_ERR_ELEMENT_NOT_FOUND: gpg_err_code_t = 136;
    pub const GPG_ERR_IDENTIFIER_NOT_FOUND: gpg_err_code_t = 137;
    pub const GPG_ERR_INV_TAG: gpg_err_code_t = 138;
    pub const GPG_ERR_INV_LENGTH: gpg_err_code_t = 139;
    pub const GPG_ERR_INV_KEYINFO: gpg_err_code_t = 140;
    pub const GPG_ERR_UNEXPECTED_TAG: gpg_err_code_t = 141;
    pub const GPG_ERR_NOT_DER_ENCODED: gpg_err_code_t = 142;
    pub const GPG_ERR_NO_CMS_OBJ: gpg_err_code_t = 143;
    pub const GPG_ERR_INV_CMS_OBJ: gpg_err_code_t = 144;
    pub const GPG_ERR_UNKNOWN_CMS_OBJ: gpg_err_code_t = 145;
    pub const GPG_ERR_UNSUPPORTED_CMS_OBJ: gpg_err_code_t = 146;
    pub const GPG_ERR_UNSUPPORTED_ENCODING: gpg_err_code_t = 147;
    pub const GPG_ERR_UNSUPPORTED_CMS_VERSION: gpg_err_code_t = 148;
    pub const GPG_ERR_UNKNOWN_ALGORITHM: gpg_err_code_t = 149;
    pub const GPG_ERR_INV_ENGINE: gpg_err_code_t = 150;
    pub const GPG_ERR_PUBKEY_NOT_TRUSTED: gpg_err_code_t = 151;
    pub const GPG_ERR_DECRYPT_FAILED: gpg_err_code_t = 152;
    pub const GPG_ERR_KEY_EXPIRED: gpg_err_code_t = 153;
    pub const GPG_ERR_SIG_EXPIRED: gpg_err_code_t = 154;
    pub const GPG_ERR_ENCODING_PROBLEM: gpg_err_code_t = 155;
    pub const GPG_ERR_INV_STATE: gpg_err_code_t = 156;
    pub const GPG_ERR_DUP_VALUE: gpg_err_code_t = 157;
    pub const GPG_ERR_MISSING_ACTION: gpg_err_code_t = 158;
    pub const GPG_ERR_MODULE_NOT_FOUND: gpg_err_code_t = 159;
    pub const GPG_ERR_INV_OID_STRING: gpg_err_code_t = 160;
    pub const GPG_ERR_INV_TIME: gpg_err_code_t = 161;
    pub const GPG_ERR_INV_CRL_OBJ: gpg_err_code_t = 162;
    pub const GPG_ERR_UNSUPPORTED_CRL_VERSION: gpg_err_code_t = 163;
    pub const GPG_ERR_INV_CERT_OBJ: gpg_err_code_t = 164;
    pub const GPG_ERR_UNKNOWN_NAME: gpg_err_code_t = 165;
    pub const GPG_ERR_LOCALE_PROBLEM: gpg_err_code_t = 166;
    pub const GPG_ERR_NOT_LOCKED: gpg_err_code_t = 167;
    pub const GPG_ERR_PROTOCOL_VIOLATION: gpg_err_code_t = 168;
    pub const GPG_ERR_INV_MAC: gpg_err_code_t = 169;
    pub const GPG_ERR_INV_REQUEST: gpg_err_code_t = 170;
    pub const GPG_ERR_UNKNOWN_EXTN: gpg_err_code_t = 171;
    pub const GPG_ERR_UNKNOWN_CRIT_EXTN: gpg_err_code_t = 172;
    pub const GPG_ERR_LOCKED: gpg_err_code_t = 173;
    pub const GPG_ERR_UNKNOWN_OPTION: gpg_err_code_t = 174;
    pub const GPG_ERR_UNKNOWN_COMMAND: gpg_err_code_t = 175;
    pub const GPG_ERR_NOT_OPERATIONAL: gpg_err_code_t = 176;
    pub const GPG_ERR_NO_PASSPHRASE: gpg_err_code_t = 177;
    pub const GPG_ERR_NO_PIN: gpg_err_code_t = 178;
    pub const GPG_ERR_NOT_ENABLED: gpg_err_code_t = 179;
    pub const GPG_ERR_NO_ENGINE: gpg_err_code_t = 180;
    pub const GPG_ERR_MISSING_KEY: gpg_err_code_t = 181;
    pub const GPG_ERR_TOO_MANY: gpg_err_code_t = 182;
    pub const GPG_ERR_LIMIT_REACHED: gpg_err_code_t = 183;
    pub const GPG_ERR_NOT_INITIALIZED: gpg_err_code_t = 184;
    pub const GPG_ERR_MISSING_ISSUER_CERT: gpg_err_code_t = 185;
    pub const GPG_ERR_NO_KEYSERVER: gpg_err_code_t = 186;
    pub const GPG_ERR_INV_CURVE: gpg_err_code_t = 187;
    pub const GPG_ERR_UNKNOWN_CURVE: gpg_err_code_t = 188;
    pub const GPG_ERR_DUP_KEY: gpg_err_code_t = 189;
    pub const GPG_ERR_AMBIGUOUS: gpg_err_code_t = 190;
    pub const GPG_ERR_NO_CRYPT_CTX: gpg_err_code_t = 191;
    pub const GPG_ERR_WRONG_CRYPT_CTX: gpg_err_code_t = 192;
    pub const GPG_ERR_BAD_CRYPT_CTX: gpg_err_code_t = 193;
    pub const GPG_ERR_CRYPT_CTX_CONFLICT: gpg_err_code_t = 194;
    pub const GPG_ERR_BROKEN_PUBKEY: gpg_err_code_t = 195;
    pub const GPG_ERR_BROKEN_SECKEY: gpg_err_code_t = 196;
    pub const GPG_ERR_MAC_ALGO: gpg_err_code_t = 197;
    pub const GPG_ERR_FULLY_CANCELED: gpg_err_code_t = 198;
    pub const GPG_ERR_UNFINISHED: gpg_err_code_t = 199;
    pub const GPG_ERR_BUFFER_TOO_SHORT: gpg_err_code_t = 200;
    pub const GPG_ERR_SEXP_INV_LEN_SPEC: gpg_err_code_t = 201;
    pub const GPG_ERR_SEXP_STRING_TOO_LONG: gpg_err_code_t = 202;
    pub const GPG_ERR_SEXP_UNMATCHED_PAREN: gpg_err_code_t = 203;
    pub const GPG_ERR_SEXP_NOT_CANONICAL: gpg_err_code_t = 204;
    pub const GPG_ERR_SEXP_BAD_CHARACTER: gpg_err_code_t = 205;
    pub const GPG_ERR_SEXP_BAD_QUOTATION: gpg_err_code_t = 206;
    pub const GPG_ERR_SEXP_ZERO_PREFIX: gpg_err_code_t = 207;
    pub const GPG_ERR_SEXP_NESTED_DH: gpg_err_code_t = 208;
    pub const GPG_ERR_SEXP_UNMATCHED_DH: gpg_err_code_t = 209;
    pub const GPG_ERR_SEXP_UNEXPECTED_PUNC: gpg_err_code_t = 210;
    pub const GPG_ERR_SEXP_BAD_HEX_CHAR: gpg_err_code_t = 211;
    pub const GPG_ERR_SEXP_ODD_HEX_NUMBERS: gpg_err_code_t = 212;
    pub const GPG_ERR_SEXP_BAD_OCT_CHAR: gpg_err_code_t = 213;
    pub const GPG_ERR_NO_CERT_CHAIN: gpg_err_code_t = 226;
    pub const GPG_ERR_CERT_TOO_LARGE: gpg_err_code_t = 227;
    pub const GPG_ERR_INV_RECORD: gpg_err_code_t = 228;
    pub const GPG_ERR_BAD_MAC: gpg_err_code_t = 229;
    pub const GPG_ERR_UNEXPECTED_MSG: gpg_err_code_t = 230;
    pub const GPG_ERR_COMPR_FAILED: gpg_err_code_t = 231;
    pub const GPG_ERR_WOULD_WRAP: gpg_err_code_t = 232;
    pub const GPG_ERR_FATAL_ALERT: gpg_err_code_t = 233;
    pub const GPG_ERR_NO_CIPHER: gpg_err_code_t = 234;
    pub const GPG_ERR_MISSING_CLIENT_CERT: gpg_err_code_t = 235;
    pub const GPG_ERR_CLOSE_NOTIFY: gpg_err_code_t = 236;
    pub const GPG_ERR_TICKET_EXPIRED: gpg_err_code_t = 237;
    pub const GPG_ERR_BAD_TICKET: gpg_err_code_t = 238;
    pub const GPG_ERR_UNKNOWN_IDENTITY: gpg_err_code_t = 239;
    pub const GPG_ERR_BAD_HS_CERT: gpg_err_code_t = 240;
    pub const GPG_ERR_BAD_HS_CERT_REQ: gpg_err_code_t = 241;
    pub const GPG_ERR_BAD_HS_CERT_VER: gpg_err_code_t = 242;
    pub const GPG_ERR_BAD_HS_CHANGE_CIPHER: gpg_err_code_t = 243;
    pub const GPG_ERR_BAD_HS_CLIENT_HELLO: gpg_err_code_t = 244;
    pub const GPG_ERR_BAD_HS_SERVER_HELLO: gpg_err_code_t = 245;
    pub const GPG_ERR_BAD_HS_SERVER_HELLO_DONE: gpg_err_code_t = 246;
    pub const GPG_ERR_BAD_HS_FINISHED: gpg_err_code_t = 247;
    pub const GPG_ERR_BAD_HS_SERVER_KEX: gpg_err_code_t = 248;
    pub const GPG_ERR_BAD_HS_CLIENT_KEX: gpg_err_code_t = 249;
    pub const GPG_ERR_BOGUS_STRING: gpg_err_code_t = 250;
    pub const GPG_ERR_KEY_DISABLED: gpg_err_code_t = 252;
    pub const GPG_ERR_KEY_ON_CARD: gpg_err_code_t = 253;
    pub const GPG_ERR_INV_LOCK_OBJ: gpg_err_code_t = 254;
    pub const GPG_ERR_ASS_GENERAL: gpg_err_code_t = 257;
    pub const GPG_ERR_ASS_ACCEPT_FAILED: gpg_err_code_t = 258;
    pub const GPG_ERR_ASS_CONNECT_FAILED: gpg_err_code_t = 259;
    pub const GPG_ERR_ASS_INV_RESPONSE: gpg_err_code_t = 260;
    pub const GPG_ERR_ASS_INV_VALUE: gpg_err_code_t = 261;
    pub const GPG_ERR_ASS_INCOMPLETE_LINE: gpg_err_code_t = 262;
    pub const GPG_ERR_ASS_LINE_TOO_LONG: gpg_err_code_t = 263;
    pub const GPG_ERR_ASS_NESTED_COMMANDS: gpg_err_code_t = 264;
    pub const GPG_ERR_ASS_NO_DATA_CB: gpg_err_code_t = 265;
    pub const GPG_ERR_ASS_NO_INQUIRE_CB: gpg_err_code_t = 266;
    pub const GPG_ERR_ASS_NOT_A_SERVER: gpg_err_code_t = 267;
    pub const GPG_ERR_ASS_NOT_A_CLIENT: gpg_err_code_t = 268;
    pub const GPG_ERR_ASS_SERVER_START: gpg_err_code_t = 269;
    pub const GPG_ERR_ASS_READ_ERROR: gpg_err_code_t = 270;
    pub const GPG_ERR_ASS_WRITE_ERROR: gpg_err_code_t = 271;
    pub const GPG_ERR_ASS_TOO_MUCH_DATA: gpg_err_code_t = 273;
    pub const GPG_ERR_ASS_UNEXPECTED_CMD: gpg_err_code_t = 274;
    pub const GPG_ERR_ASS_UNKNOWN_CMD: gpg_err_code_t = 275;
    pub const GPG_ERR_ASS_SYNTAX: gpg_err_code_t = 276;
    pub const GPG_ERR_ASS_CANCELED: gpg_err_code_t = 277;
    pub const GPG_ERR_ASS_NO_INPUT: gpg_err_code_t = 278;
    pub const GPG_ERR_ASS_NO_OUTPUT: gpg_err_code_t = 279;
    pub const GPG_ERR_ASS_PARAMETER: gpg_err_code_t = 280;
    pub const GPG_ERR_ASS_UNKNOWN_INQUIRE: gpg_err_code_t = 281;
    pub const GPG_ERR_USER_1: gpg_err_code_t = 1024;
    pub const GPG_ERR_USER_2: gpg_err_code_t = 1025;
    pub const GPG_ERR_USER_3: gpg_err_code_t = 1026;
    pub const GPG_ERR_USER_4: gpg_err_code_t = 1027;
    pub const GPG_ERR_USER_5: gpg_err_code_t = 1028;
    pub const GPG_ERR_USER_6: gpg_err_code_t = 1029;
    pub const GPG_ERR_USER_7: gpg_err_code_t = 1030;
    pub const GPG_ERR_USER_8: gpg_err_code_t = 1031;
    pub const GPG_ERR_USER_9: gpg_err_code_t = 1032;
    pub const GPG_ERR_USER_10: gpg_err_code_t = 1033;
    pub const GPG_ERR_USER_11: gpg_err_code_t = 1034;
    pub const GPG_ERR_USER_12: gpg_err_code_t = 1035;
    pub const GPG_ERR_USER_13: gpg_err_code_t = 1036;
    pub const GPG_ERR_USER_14: gpg_err_code_t = 1037;
    pub const GPG_ERR_USER_15: gpg_err_code_t = 1038;
    pub const GPG_ERR_USER_16: gpg_err_code_t = 1039;
    pub const GPG_ERR_MISSING_ERRNO: gpg_err_code_t = 16381;
    pub const GPG_ERR_UNKNOWN_ERRNO: gpg_err_code_t = 16382;
    pub const GPG_ERR_EOF: gpg_err_code_t = 16383;
    pub const GPG_ERR_E2BIG: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 0;
    pub const GPG_ERR_EACCES: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 1;
    pub const GPG_ERR_EADDRINUSE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 2;
    pub const GPG_ERR_EADDRNOTAVAIL: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 3;
    pub const GPG_ERR_EADV: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 4;
    pub const GPG_ERR_EAFNOSUPPORT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 5;
    pub const GPG_ERR_EAGAIN: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 6;
    pub const GPG_ERR_EALREADY: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 7;
    pub const GPG_ERR_EAUTH: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 8;
    pub const GPG_ERR_EBACKGROUND: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 9;
    pub const GPG_ERR_EBADE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 10;
    pub const GPG_ERR_EBADF: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 11;
    pub const GPG_ERR_EBADFD: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 12;
    pub const GPG_ERR_EBADMSG: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 13;
    pub const GPG_ERR_EBADR: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 14;
    pub const GPG_ERR_EBADRPC: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 15;
    pub const GPG_ERR_EBADRQC: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 16;
    pub const GPG_ERR_EBADSLT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 17;
    pub const GPG_ERR_EBFONT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 18;
    pub const GPG_ERR_EBUSY: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 19;
    pub const GPG_ERR_ECANCELED: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 20;
    pub const GPG_ERR_ECHILD: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 21;
    pub const GPG_ERR_ECHRNG: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 22;
    pub const GPG_ERR_ECOMM: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 23;
    pub const GPG_ERR_ECONNABORTED: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 24;
    pub const GPG_ERR_ECONNREFUSED: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 25;
    pub const GPG_ERR_ECONNRESET: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 26;
    pub const GPG_ERR_ED: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 27;
    pub const GPG_ERR_EDEADLK: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 28;
    pub const GPG_ERR_EDEADLOCK: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 29;
    pub const GPG_ERR_EDESTADDRREQ: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 30;
    pub const GPG_ERR_EDIED: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 31;
    pub const GPG_ERR_EDOM: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 32;
    pub const GPG_ERR_EDOTDOT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 33;
    pub const GPG_ERR_EDQUOT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 34;
    pub const GPG_ERR_EEXIST: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 35;
    pub const GPG_ERR_EFAULT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 36;
    pub const GPG_ERR_EFBIG: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 37;
    pub const GPG_ERR_EFTYPE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 38;
    pub const GPG_ERR_EGRATUITOUS: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 39;
    pub const GPG_ERR_EGREGIOUS: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 40;
    pub const GPG_ERR_EHOSTDOWN: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 41;
    pub const GPG_ERR_EHOSTUNREACH: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 42;
    pub const GPG_ERR_EIDRM: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 43;
    pub const GPG_ERR_EIEIO: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 44;
    pub const GPG_ERR_EILSEQ: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 45;
    pub const GPG_ERR_EINPROGRESS: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 46;
    pub const GPG_ERR_EINTR: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 47;
    pub const GPG_ERR_EINVAL: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 48;
    pub const GPG_ERR_EIO: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 49;
    pub const GPG_ERR_EISCONN: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 50;
    pub const GPG_ERR_EISDIR: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 51;
    pub const GPG_ERR_EISNAM: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 52;
    pub const GPG_ERR_EL2HLT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 53;
    pub const GPG_ERR_EL2NSYNC: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 54;
    pub const GPG_ERR_EL3HLT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 55;
    pub const GPG_ERR_EL3RST: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 56;
    pub const GPG_ERR_ELIBACC: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 57;
    pub const GPG_ERR_ELIBBAD: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 58;
    pub const GPG_ERR_ELIBEXEC: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 59;
    pub const GPG_ERR_ELIBMAX: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 60;
    pub const GPG_ERR_ELIBSCN: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 61;
    pub const GPG_ERR_ELNRNG: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 62;
    pub const GPG_ERR_ELOOP: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 63;
    pub const GPG_ERR_EMEDIUMTYPE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 64;
    pub const GPG_ERR_EMFILE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 65;
    pub const GPG_ERR_EMLINK: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 66;
    pub const GPG_ERR_EMSGSIZE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 67;
    pub const GPG_ERR_EMULTIHOP: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 68;
    pub const GPG_ERR_ENAMETOOLONG: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 69;
    pub const GPG_ERR_ENAVAIL: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 70;
    pub const GPG_ERR_ENEEDAUTH: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 71;
    pub const GPG_ERR_ENETDOWN: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 72;
    pub const GPG_ERR_ENETRESET: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 73;
    pub const GPG_ERR_ENETUNREACH: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 74;
    pub const GPG_ERR_ENFILE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 75;
    pub const GPG_ERR_ENOANO: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 76;
    pub const GPG_ERR_ENOBUFS: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 77;
    pub const GPG_ERR_ENOCSI: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 78;
    pub const GPG_ERR_ENODATA: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 79;
    pub const GPG_ERR_ENODEV: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 80;
    pub const GPG_ERR_ENOENT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 81;
    pub const GPG_ERR_ENOEXEC: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 82;
    pub const GPG_ERR_ENOLCK: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 83;
    pub const GPG_ERR_ENOLINK: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 84;
    pub const GPG_ERR_ENOMEDIUM: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 85;
    pub const GPG_ERR_ENOMEM: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 86;
    pub const GPG_ERR_ENOMSG: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 87;
    pub const GPG_ERR_ENONET: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 88;
    pub const GPG_ERR_ENOPKG: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 89;
    pub const GPG_ERR_ENOPROTOOPT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 90;
    pub const GPG_ERR_ENOSPC: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 91;
    pub const GPG_ERR_ENOSR: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 92;
    pub const GPG_ERR_ENOSTR: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 93;
    pub const GPG_ERR_ENOSYS: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 94;
    pub const GPG_ERR_ENOTBLK: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 95;
    pub const GPG_ERR_ENOTCONN: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 96;
    pub const GPG_ERR_ENOTDIR: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 97;
    pub const GPG_ERR_ENOTEMPTY: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 98;
    pub const GPG_ERR_ENOTNAM: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 99;
    pub const GPG_ERR_ENOTSOCK: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 100;
    pub const GPG_ERR_ENOTSUP: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 101;
    pub const GPG_ERR_ENOTTY: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 102;
    pub const GPG_ERR_ENOTUNIQ: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 103;
    pub const GPG_ERR_ENXIO: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 104;
    pub const GPG_ERR_EOPNOTSUPP: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 105;
    pub const GPG_ERR_EOVERFLOW: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 106;
    pub const GPG_ERR_EPERM: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 107;
    pub const GPG_ERR_EPFNOSUPPORT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 108;
    pub const GPG_ERR_EPIPE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 109;
    pub const GPG_ERR_EPROCLIM: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 110;
    pub const GPG_ERR_EPROCUNAVAIL: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 111;
    pub const GPG_ERR_EPROGMISMATCH: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 112;
    pub const GPG_ERR_EPROGUNAVAIL: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 113;
    pub const GPG_ERR_EPROTO: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 114;
    pub const GPG_ERR_EPROTONOSUPPORT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 115;
    pub const GPG_ERR_EPROTOTYPE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 116;
    pub const GPG_ERR_ERANGE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 117;
    pub const GPG_ERR_EREMCHG: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 118;
    pub const GPG_ERR_EREMOTE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 119;
    pub const GPG_ERR_EREMOTEIO: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 120;
    pub const GPG_ERR_ERESTART: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 121;
    pub const GPG_ERR_EROFS: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 122;
    pub const GPG_ERR_ERPCMISMATCH: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 123;
    pub const GPG_ERR_ESHUTDOWN: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 124;
    pub const GPG_ERR_ESOCKTNOSUPPORT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 125;
    pub const GPG_ERR_ESPIPE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 126;
    pub const GPG_ERR_ESRCH: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 127;
    pub const GPG_ERR_ESRMNT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 128;
    pub const GPG_ERR_ESTALE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 129;
    pub const GPG_ERR_ESTRPIPE: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 130;
    pub const GPG_ERR_ETIME: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 131;
    pub const GPG_ERR_ETIMEDOUT: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 132;
    pub const GPG_ERR_ETOOMANYREFS: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 133;
    pub const GPG_ERR_ETXTBSY: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 134;
    pub const GPG_ERR_EUCLEAN: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 135;
    pub const GPG_ERR_EUNATCH: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 136;
    pub const GPG_ERR_EUSERS: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 137;
    pub const GPG_ERR_EWOULDBLOCK: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 138;
    pub const GPG_ERR_EXDEV: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 139;
    pub const GPG_ERR_EXFULL: gpg_err_code_t = GPG_ERR_SYSTEM_ERROR | 140;
    pub const GPG_ERR_CODE_DIM: gpg_err_code_t = 65536;

    pub const GPG_ERR_CODE_MASK: gpg_error_t = (GPG_ERR_CODE_DIM as gpg_error_t) - 1;

    pub const GPG_ERR_SOURCE_MASK: gpg_error_t = (GPG_ERR_SOURCE_DIM as gpg_error_t) - 1;
    pub const GPG_ERR_SOURCE_SHIFT: gpg_error_t = 24;
}

pub mod funcs {
    extern crate libc;

    use types::gpg_error_t;
    use types::gpg_err_source_t;
    use types::gpg_err_code_t;

    use consts::*;

    pub fn gpg_err_make(source: gpg_err_source_t, code: gpg_err_code_t) -> gpg_error_t {
        if code == GPG_ERR_NO_ERROR {
            GPG_ERR_NO_ERROR
        } else {
            ((source & GPG_ERR_SOURCE_MASK) << GPG_ERR_SOURCE_SHIFT) |
                (code & GPG_ERR_CODE_MASK)
        }
    }

    pub fn gpg_err_code(err: gpg_error_t) -> gpg_err_code_t {
        err & GPG_ERR_CODE_MASK
    }

    pub fn gpg_err_source(err: gpg_error_t) -> gpg_err_source_t {
        (err >> GPG_ERR_SOURCE_SHIFT) & GPG_ERR_SOURCE_MASK
    }

    #[link(name = "gpg-error")]
    extern {
        pub fn gpg_err_init() -> gpg_error_t;
        pub fn gpg_err_deinit(mode: libc::c_int);

        pub fn gpg_strerror(err: gpg_error_t) -> *const libc::c_char;
        pub fn gpg_strerror_r(err: gpg_error_t, buf: *mut libc::c_char, buflen: libc::size_t) -> libc::c_int;

        pub fn gpg_strsource(err: gpg_error_t) -> *const libc::c_char;

        pub fn gpg_err_code_to_errno(code: gpg_err_code_t) -> libc::c_int;
        pub fn gpg_err_code_from_syserror() -> gpg_err_code_t;

        pub fn gpg_err_set_errno(err: libc::c_int);

        pub fn gpg_error_check_version(req_version: *const libc::c_char) -> *const libc::c_char;
    }
}
