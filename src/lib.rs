use std::num::ParseIntError;
use std::str;

/// `text` の各行を逆順にしてください。改行コードはLFとします。
/// `text` は空行でない限り、改行で終わることを仮定して構いません。
///
/// ## 例
///
/// ```rust
/// assert_eq!(reverse_each_line("abc\ndef\n"), "cba\nfed\n");
/// ```
pub fn reverse_each_line(_text: &str) -> String {
    unimplemented!();
}

/// HTTPヘッダー名をCGI変数名に変換してください。具体的には以下をしてください。
///
/// - `-` は `_` に変換する。
/// - ASCIIの小文字は大文字に変換する。
/// - `HTTP_` を前置する。
///
/// ## 例
///
/// ```rust
/// assert_eq!(reverse_each_line("User-Agent"), "HTTP_USER_AGENT");
/// ```
pub fn into_cgi_env(_header: &str) -> String {
    unimplemented!();
}

/// カンマ区切りの整数を合計してください。
///
/// - 各要素には空白が含まれる可能性があります。
/// - 空文字列は1要素からなるとして扱います。
/// - 空文字列からなる要素はパースエラーとして構いません。
///
/// ## 例
///
/// ```rust
/// assert_eq!(comma_sum("3, -3"), 0);
/// ```
pub fn comma_sum(_line: &str) -> Result<i32, ParseIntError> {
    unimplemented!();
}

/// 標準的なbase64変換を実装してください。
///
/// ## 例
///
/// ```rust
/// assert_eq!(base64(b"hoge"), "aG9nZQ=");
/// ```
pub fn base64(_data: &[u8]) -> String {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comma_sum_1() {
        assert_eq!(comma_sum("10, 30, 50\n").unwrap(), 90);
    }
    #[test]
    fn test_comma_sum_2() {
        assert_eq!(comma_sum(" -2 ,56").unwrap(), 54);
    }
    #[test]
    fn test_comma_sum_3() {
        assert!(comma_sum("").is_err());
    }
    #[test]
    fn test_comma_sum_4() {
        assert!(comma_sum(",20").is_err());
    }
    #[test]
    fn test_comma_sum_5() {
        assert!(comma_sum("-5,").is_err());
    }
    #[test]
    fn test_comma_sum_6() {
        assert!(comma_sum("65%").is_err());
    }
    #[test]
    fn test_into_cgi_env_1() {
        assert_eq!(into_cgi_env("User-Agent"), "HTTP_USER_AGENT");
    }
    #[test]
    fn test_into_cgi_env_2() {
        assert_eq!(into_cgi_env("host"), "HTTP_HOST");
    }
    #[test]
    fn test_into_cgi_env_3() {
        assert_eq!(into_cgi_env("ACCEPT_ENCODING"), "HTTP_ACCEPT_ENCODING");
    }
    #[test]
    fn test_into_cgi_env_4() {
        assert_eq!(into_cgi_env("référant"), "HTTP_RéFéRANT");
    }
    #[test]
    fn test_reverse_each_line_1() {
        assert_eq!(reverse_each_line("foo\nbar\n"), "oof\nrab\n");
    }
    #[test]
    fn test_reverse_each_line_2() {
        assert_eq!(reverse_each_line(""), "");
    }
    #[test]
    fn test_reverse_each_line_3() {
        assert_eq!(
            reverse_each_line("123\nabcdefg\n9876543210\n"),
            "321\ngfedcba\n0123456789\n"
        );
    }
    #[test]
    fn test_base64_1() {
        assert_eq!(base64(b"hoge"), "aG9nZQ==");
    }
    #[test]
    fn test_base64_2() {
        assert_eq!(
            base64("いろはにほへと".as_bytes()),
            "44GE44KN44Gv44Gr44G744G444Go"
        );
    }
    #[test]
    fn test_base64_3() {
        assert_eq!(base64(b"aG9nZQ=="), "YUc5blpRPT0=");
    }
    #[test]
    fn test_base64_4() {
        assert_eq!(base64(b""), "");
    }
}
