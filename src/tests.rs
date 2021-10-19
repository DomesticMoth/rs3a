/*
    This file is part of rs3a.

    rs3a is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Foobar is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Foobar.  If not, see <https://www.gnu.org/licenses/>.
*/
mod tests {
    use crate::*;
    #[test]
    fn test_escape_comments(){
        let a = "01\t23\n45\t67\n89".to_string();
        let b = "014589".to_string();
        assert_eq!(escape_comments(&a), b);
    }
}

mod body_to_text_test{
    use crate::*;
    #[test]
    fn test_correct_fullcolor(){
        let text_reference = "AAAAaabb1122\nBBBBaabc1122\nCCCCaaaa1111\nDDDDabcd1111\n\nAAAAaabb1122\nBBBBaabc1122\nCCCCaaaa1111\nDDDDabcd1111\n";
        let body = Body{
            frames: vec![
                vec![
                    vec![
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "BB".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "CCCC".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_MAGENTA),
                        },
                    ],
                ],
                vec![
                    vec![
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "BB".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "CCCC".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_MAGENTA),
                        },
                    ],
                ],
            ],
        };
        assert_eq!(body.to_string(true), text_reference);
    }
}

mod body_from_text_test{
    use crate::*;
    #[test]
    fn test_correct_fullcolor(){
        let header = Header{
            width: 4,
            height: 4,
            delay: 200,
            loop_enable: true,
            color_mod: ColorMod::Full,
            utf8: false,
            datacols: 3,
            preview: DEFAULT_PREVIEW,
            audio: None,
            title: None,
            author: None,
        };
        let body_reference = Body{
            frames: vec![
                vec![
                    vec![
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "BB".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "CCCC".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_MAGENTA),
                        },
                    ],
                ],
                vec![
                    vec![
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "AA".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "BB".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "B".to_string(),
                            bg_color: Some(Color::GREEN),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "CCCC".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                    ],
                    vec![
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_GREEN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_CYAN),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_RED),
                        },
                        RowFragment{
                            text: "D".to_string(),
                            bg_color: Some(Color::BLUE),
                            fg_color: Some(Color::BRIGHT_MAGENTA),
                        },
                    ],
                ],
            ],
        };
        let text = "AAAAaabb1122\nBBBBaabc1122\nCCCCaaaa1111\nDDDDabcd1111\n\nAAAAaabb1122\nBBBBaabc1122\nCCCCaaaa1111\nDDDDabcd1111\n";
        match Body::from_string(text.to_string(),header) {
            Ok(result) => {assert_eq!(body_reference, result);}
            Err(_) => {assert!(false, "Unexpected parcing error");}
        }
    }
}

mod colormod_test{
    use crate::*;
    use std::convert::{TryFrom, Into};
    #[test]
    fn test_none(){
        let base = ColorMod::None;
        let s: String = base.into();
        let s: &str = &s;
        match ColorMod::try_from(s){
            Ok(result) => {assert_eq!(base, result);}
            Err(_) => {assert!(false, "Unexpected parcing error");}
        }
    }
    #[test]
    fn test_fg(){
        let base = ColorMod::Fg;
        let s: String = base.into();
        let s: &str = &s;
        match ColorMod::try_from(s){
            Ok(result) => {assert_eq!(base, result);}
            Err(_) => {assert!(false, "Unexpected parcing error");}
        }
    }
    #[test]
    fn test_bg(){
        let base = ColorMod::Bg;
        let s: String = base.into();
        let s: &str = &s;
        match ColorMod::try_from(s){
            Ok(result) => {assert_eq!(base, result);}
            Err(_) => {assert!(false, "Unexpected parcing error");}
        }
    }
    #[test]
    fn test_full(){
        let base = ColorMod::Full;
        let s: String = base.into();
        let s: &str = &s;
        match ColorMod::try_from(s){
            Ok(result) => {assert_eq!(base, result);}
            Err(_) => {assert!(false, "Unexpected parcing error");}
        }
    }
}

mod header_to_string_tests{
    use crate::*;
    #[test]
    fn all_params(){
        let header = Header{
            width: 1,
            height: 2,
            delay: DEFAULT_DELAY+1,
            loop_enable: !DEFAULT_LOOP,
            color_mod: ColorMod::Full,
            utf8: true,
            datacols: 123,
            preview: 1,
            audio: Some("1234567".to_string()),
            title: None,
            author: None,
        };
        let s_ref = "width 1\nheight 2\ndelay 51\nloop false\ncolors full\nutf8\ndatacols 123\npreview 1\naudio 1234567\n\n".to_string();
        let s: String = header.into();
        assert_eq!(s_ref, s);
    }
    #[test]
    fn default_params(){
        let header = Header{
            width: 1,
            height: 2,
            delay: DEFAULT_DELAY,
            loop_enable: DEFAULT_LOOP,
            color_mod: DEFAULT_COLORS,
            utf8: DEFAULT_UTF8,
            datacols: DEFAULT_COLORS.to_datacols(),
            preview: DEFAULT_PREVIEW,
            audio: None,
            title: None,
            author: None,
        };
        let s_ref = "width 1\nheight 2\n\n".to_string();
        let s: String = header.into();
        assert_eq!(s_ref, s);
    }
    #[test]
    fn datacols(){
        let header = Header{
            width: 1,
            height: 2,
            delay: DEFAULT_DELAY,
            loop_enable: DEFAULT_LOOP,
            color_mod: DEFAULT_COLORS,
            utf8: DEFAULT_UTF8,
            datacols: DEFAULT_COLORS.to_datacols()+1,
            preview: DEFAULT_PREVIEW,
            audio: None,
            title: None,
            author: None,
        };
        let s_ref = "width 1\nheight 2\ndatacols 2\n\n".to_string();
        let s: String = header.into();
        assert_eq!(s_ref, s);
    }
}

mod header_from_string_tests{
    use crate::*;
    use std::convert::{TryFrom};
    #[test]
    fn full(){
        let s = "width 1\nheight 2\ndelay 3\nloop false\ncolors full\nutf8\ndatacols 5\npreview 1\naudio 12345".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
            delay: 3,
            loop_enable: false,
            color_mod: ColorMod::Full,
            utf8: true,
            datacols: 5,
            preview: 1,
            audio: Some("12345".to_string()),
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
    }
    #[test]
    fn only_required(){
        let s = "width 1\nheight 2".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
             delay: 50,
            loop_enable: true,
            color_mod: ColorMod::None,
            utf8: false,
            datacols: 1,
            preview: 0,
            audio: None,
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
    }
    #[test]
    fn optional_incorrect(){
        let s = "width 1\nheight 2\ndelay safdsfsdf\nloop dsfsdf\ncolors dfdfdf\ndatacols dfsfsddf".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
             delay: 50,
            loop_enable: true,
            color_mod: ColorMod::None,
            utf8: false,
            datacols: 1,
            preview: 0,
            audio: None,
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
    }
    #[test]
    fn width_incorrect(){
        let s = "width sdfsfsdf\nheight 2\ndelay 3\nloop false\ncolors full\nutf8\ndatacols 5\naudio 12345".to_string();
        if let Ok(_) = Header::try_from(s){
            assert!(false, "Unexpected Ok");
        }
    }
    #[test]
    fn height_incorrect(){
        let s = "width 1\nheight sdfsdfsdf\ndelay 3\nloop false\ncolors full\nutf8\ndatacols 5\naudio 12345".to_string();
        if let Ok(_) = Header::try_from(s){
            assert!(false, "Unexpected Ok");
        }
    }
    #[test]
    fn datacols(){
        let s = "width 1\nheight 2\ncolors full".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
             delay: 50,
            loop_enable: true,
            color_mod: ColorMod::Full,
            utf8: false,
            datacols: 3,
            preview: 0,
            audio: None,
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
        let s = "width 1\nheight 2\ncolors full\ndatacols 0".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
             delay: 50,
            loop_enable: true,
            color_mod: ColorMod::Full,
            utf8: false,
            datacols: 0,
            preview: 0,
            audio: None,
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
    }
    #[test]
    fn extra_spaces(){
        let s = "width    1\nheight    2\ndelay    3\nloop    false\ncolors    full \
        \nutf8   \ndatacols    5\naudio    12345".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
            delay: 3,
            loop_enable: false,
            color_mod: ColorMod::Full,
            utf8: true,
            datacols: 5,
            preview: 0,
            audio: Some("12345".to_string()),
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
    }
    #[test]
    fn extra_params(){
        let s = "width 1   sfdfsdf fdsfd sdf \nheight 2 fds dsfsdf\ndelay 3 fd ff \
        \nloop false   fdfdf  \ncolors full fdfd\nutf8  fdfdf\ndatacols 5 fdfd fd d\naudio 12345 fdfdfdf".to_string();
        let refernce = Header{
            width: 1,
            height: 2,
            delay: 3,
            loop_enable: false,
            color_mod: ColorMod::Full,
            utf8: true,
            datacols: 5,
            preview: 0,
            audio: Some("12345".to_string()),
            title: None,
            author: None,
        };
        match Header::try_from(s){
            Ok(result) => { assert_eq!(refernce, result); }
            Err(_) => { assert!(false, "Unexpected parcing error"); }
        }
    }
}

