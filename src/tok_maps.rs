use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::token::{Ident, T};

#[derive(Debug, Eq, PartialEq)]
#[allow(non_snake_case)]
pub struct Keywords {
    pub auto_ident: Rc<RefCell<Ident>>,
    pub break_ident: Rc<RefCell<Ident>>,
    pub case_ident: Rc<RefCell<Ident>>,
    pub char_ident: Rc<RefCell<Ident>>,
    pub const_ident: Rc<RefCell<Ident>>,
    pub continue_ident: Rc<RefCell<Ident>>,
    pub default_ident: Rc<RefCell<Ident>>,
    pub do_ident: Rc<RefCell<Ident>>,
    pub double_ident: Rc<RefCell<Ident>>,
    pub else_ident: Rc<RefCell<Ident>>,
    pub enum_ident: Rc<RefCell<Ident>>,
    pub extern_ident: Rc<RefCell<Ident>>,
    pub float_ident: Rc<RefCell<Ident>>,
    pub for_ident: Rc<RefCell<Ident>>,
    pub goto_ident: Rc<RefCell<Ident>>,
    pub if_ident: Rc<RefCell<Ident>>,
    pub inline_ident: Rc<RefCell<Ident>>,
    pub int_ident: Rc<RefCell<Ident>>,
    pub long_ident: Rc<RefCell<Ident>>,
    pub register_ident: Rc<RefCell<Ident>>,
    pub restrict_ident: Rc<RefCell<Ident>>,
    pub return_ident: Rc<RefCell<Ident>>,
    pub short_ident: Rc<RefCell<Ident>>,
    pub signed_ident: Rc<RefCell<Ident>>,
    pub sizeof_ident: Rc<RefCell<Ident>>,
    pub static_ident: Rc<RefCell<Ident>>,
    pub struct_ident: Rc<RefCell<Ident>>,
    pub switch_ident: Rc<RefCell<Ident>>,
    pub typedef_ident: Rc<RefCell<Ident>>,
    pub union_ident: Rc<RefCell<Ident>>,
    pub unsigned_ident: Rc<RefCell<Ident>>,
    pub void_ident: Rc<RefCell<Ident>>,
    pub volatile_ident: Rc<RefCell<Ident>>,
    pub while_ident: Rc<RefCell<Ident>>,
    pub _Alignas_ident: Rc<RefCell<Ident>>,
    pub _Alignof_ident: Rc<RefCell<Ident>>,
    pub _Atomic_ident: Rc<RefCell<Ident>>,
    pub _Bool_ident: Rc<RefCell<Ident>>,
    pub _Complex_ident: Rc<RefCell<Ident>>,
    pub _Decimal128_ident: Rc<RefCell<Ident>>,
    pub _Decimal32_ident: Rc<RefCell<Ident>>,
    pub _Decimal64_ident: Rc<RefCell<Ident>>,
    pub _Generic_ident: Rc<RefCell<Ident>>,
    pub _Imaginary_ident: Rc<RefCell<Ident>>,
    pub _Noreturn_ident: Rc<RefCell<Ident>>,
    pub _Static_assert_ident: Rc<RefCell<Ident>>,
    pub _Thread_local_ident: Rc<RefCell<Ident>>,
    pub asm_ident: Rc<RefCell<Ident>>,
    pub __asm_ident: Rc<RefCell<Ident>>,
    pub __asm___ident: Rc<RefCell<Ident>>,
    pub __alignof_ident: Rc<RefCell<Ident>>,
    pub __alignof___ident: Rc<RefCell<Ident>>,
    pub __attribute_ident: Rc<RefCell<Ident>>,
    pub __attribute___ident: Rc<RefCell<Ident>>,
    pub __complex_ident: Rc<RefCell<Ident>>,
    pub __complex___ident: Rc<RefCell<Ident>>,
    pub __const_ident: Rc<RefCell<Ident>>,
    pub __const___ident: Rc<RefCell<Ident>>,
    pub __inline_ident: Rc<RefCell<Ident>>,
    pub __inline___ident: Rc<RefCell<Ident>>,
    pub __restrict_ident: Rc<RefCell<Ident>>,
    pub __restrict___ident: Rc<RefCell<Ident>>,
    pub __signed_ident: Rc<RefCell<Ident>>,
    pub __signed___ident: Rc<RefCell<Ident>>,
    pub __thread_ident: Rc<RefCell<Ident>>,
    pub typeof_ident: Rc<RefCell<Ident>>,
    pub __typeof_ident: Rc<RefCell<Ident>>,
    pub __typeof___ident: Rc<RefCell<Ident>>,
    pub __volatile_ident: Rc<RefCell<Ident>>,
    pub __volatile___ident: Rc<RefCell<Ident>>,
    pub __label___ident: Rc<RefCell<Ident>>,
    pub __extension___ident: Rc<RefCell<Ident>>,
}

impl Keywords {
    pub fn new() -> Self {
        Keywords {
            auto_ident: Rc::new(RefCell::new(Ident::new("auto".to_string(), 0))),
            break_ident: Rc::new(RefCell::new(Ident::new("break".to_string(), 1))),
            case_ident: Rc::new(RefCell::new(Ident::new("case".to_string(), 2))),
            char_ident: Rc::new(RefCell::new(Ident::new("char".to_string(), 3))),
            const_ident: Rc::new(RefCell::new(Ident::new("const".to_string(), 4))),
            continue_ident: Rc::new(RefCell::new(Ident::new("continue".to_string(), 5))),
            default_ident: Rc::new(RefCell::new(Ident::new("default".to_string(), 6))),
            do_ident: Rc::new(RefCell::new(Ident::new("do".to_string(), 7))),
            double_ident: Rc::new(RefCell::new(Ident::new("double".to_string(), 8))),
            else_ident: Rc::new(RefCell::new(Ident::new("else".to_string(), 9))),
            enum_ident: Rc::new(RefCell::new(Ident::new("enum".to_string(), 10))),
            extern_ident: Rc::new(RefCell::new(Ident::new("extern".to_string(), 11))),
            float_ident: Rc::new(RefCell::new(Ident::new("float".to_string(), 12))),
            for_ident: Rc::new(RefCell::new(Ident::new("for".to_string(), 13))),
            goto_ident: Rc::new(RefCell::new(Ident::new("goto".to_string(), 14))),
            if_ident: Rc::new(RefCell::new(Ident::new("if".to_string(), 15))),
            inline_ident: Rc::new(RefCell::new(Ident::new("inline".to_string(), 16))),
            int_ident: Rc::new(RefCell::new(Ident::new("int".to_string(), 17))),
            long_ident: Rc::new(RefCell::new(Ident::new("long".to_string(), 18))),
            register_ident: Rc::new(RefCell::new(Ident::new("register".to_string(), 19))),
            restrict_ident: Rc::new(RefCell::new(Ident::new("restrict".to_string(), 20))),
            return_ident: Rc::new(RefCell::new(Ident::new("return".to_string(), 21))),
            short_ident: Rc::new(RefCell::new(Ident::new("short".to_string(), 22))),
            signed_ident: Rc::new(RefCell::new(Ident::new("signed".to_string(), 23))),
            sizeof_ident: Rc::new(RefCell::new(Ident::new("sizeof".to_string(), 24))),
            static_ident: Rc::new(RefCell::new(Ident::new("static".to_string(), 25))),
            struct_ident: Rc::new(RefCell::new(Ident::new("struct".to_string(), 26))),
            switch_ident: Rc::new(RefCell::new(Ident::new("switch".to_string(), 27))),
            typedef_ident: Rc::new(RefCell::new(Ident::new("typedef".to_string(), 28))),
            union_ident: Rc::new(RefCell::new(Ident::new("union".to_string(), 29))),
            unsigned_ident: Rc::new(RefCell::new(Ident::new("unsigned".to_string(), 30))),
            void_ident: Rc::new(RefCell::new(Ident::new("void".to_string(), 31))),
            volatile_ident: Rc::new(RefCell::new(Ident::new("volatile".to_string(), 32))),
            while_ident: Rc::new(RefCell::new(Ident::new("while".to_string(), 33))),
            _Alignas_ident: Rc::new(RefCell::new(Ident::new("_Alignas".to_string(), 34))),
            _Alignof_ident: Rc::new(RefCell::new(Ident::new("_Alignof".to_string(), 35))),
            _Atomic_ident: Rc::new(RefCell::new(Ident::new("_Atomic".to_string(), 36))),
            _Bool_ident: Rc::new(RefCell::new(Ident::new("_Bool".to_string(), 37))),
            _Complex_ident: Rc::new(RefCell::new(Ident::new("_Complex".to_string(), 38))),
            _Decimal128_ident: Rc::new(RefCell::new(Ident::new("_Decimal128".to_string(), 39))),
            _Decimal32_ident: Rc::new(RefCell::new(Ident::new("_Decimal32".to_string(), 40))),
            _Decimal64_ident: Rc::new(RefCell::new(Ident::new("_Decimal64".to_string(), 41))),
            _Generic_ident: Rc::new(RefCell::new(Ident::new("_Generic".to_string(), 42))),
            _Imaginary_ident: Rc::new(RefCell::new(Ident::new("_Imaginary".to_string(), 43))),
            _Noreturn_ident: Rc::new(RefCell::new(Ident::new("_Noreturn".to_string(), 44))),
            _Static_assert_ident: Rc::new(RefCell::new(Ident::new("_Static_assert".to_string(), 45))),
            _Thread_local_ident: Rc::new(RefCell::new(Ident::new("_Thread_local".to_string(), 46))),
            asm_ident: Rc::new(RefCell::new(Ident::new("asm".to_string(), 47))),
            __asm_ident: Rc::new(RefCell::new(Ident::new("__asm".to_string(), 48))),
            __asm___ident: Rc::new(RefCell::new(Ident::new("__asm__".to_string(), 49))),
            __alignof_ident: Rc::new(RefCell::new(Ident::new("__alignof".to_string(), 50))),
            __alignof___ident: Rc::new(RefCell::new(Ident::new("__alignof__".to_string(), 51))),
            __attribute_ident: Rc::new(RefCell::new(Ident::new("__attribute".to_string(), 52))),
            __attribute___ident: Rc::new(RefCell::new(Ident::new("__attribute__".to_string(), 53))),
            __complex_ident: Rc::new(RefCell::new(Ident::new("__complex".to_string(), 54))),
            __complex___ident: Rc::new(RefCell::new(Ident::new("__complex__".to_string(), 55))),
            __const_ident: Rc::new(RefCell::new(Ident::new("__const".to_string(), 56))),
            __const___ident: Rc::new(RefCell::new(Ident::new("__const__".to_string(), 57))),
            __inline_ident: Rc::new(RefCell::new(Ident::new("__inline".to_string(), 58))),
            __inline___ident: Rc::new(RefCell::new(Ident::new("__inline__".to_string(), 59))),
            __restrict_ident: Rc::new(RefCell::new(Ident::new("__restrict".to_string(), 60))),
            __restrict___ident: Rc::new(RefCell::new(Ident::new("__restrict__".to_string(), 61))),
            __signed_ident: Rc::new(RefCell::new(Ident::new("__signed".to_string(), 62))),
            __signed___ident: Rc::new(RefCell::new(Ident::new("__signed__".to_string(), 63))),
            __thread_ident: Rc::new(RefCell::new(Ident::new("__thread".to_string(), 64))),
            typeof_ident: Rc::new(RefCell::new(Ident::new("typeof".to_string(), 65))),
            __typeof_ident: Rc::new(RefCell::new(Ident::new("__typeof".to_string(), 66))),
            __typeof___ident: Rc::new(RefCell::new(Ident::new("__typeof__".to_string(), 67))),
            __volatile_ident: Rc::new(RefCell::new(Ident::new("__volatile".to_string(), 68))),
            __volatile___ident: Rc::new(RefCell::new(Ident::new("__volatile__".to_string(), 69))),
            __label___ident: Rc::new(RefCell::new(Ident::new("__label__".to_string(), 70))),
            __extension___ident: Rc::new(RefCell::new(Ident::new("__extension__".to_string(), 71))),
        }
    }
}

pub fn make_id_map(keywords: &Keywords) -> HashMap<String, Rc<RefCell<Ident>>> {
    let mut idmap = HashMap::new();

    idmap.insert("auto".to_string(), Rc::clone(&keywords.auto_ident));
    idmap.insert("break".to_string(), Rc::clone(&keywords.break_ident));
    idmap.insert("case".to_string(), Rc::clone(&keywords.case_ident));
    idmap.insert("char".to_string(), Rc::clone(&keywords.char_ident));
    idmap.insert("const".to_string(), Rc::clone(&keywords.const_ident));
    idmap.insert("continue".to_string(), Rc::clone(&keywords.continue_ident));
    idmap.insert("default".to_string(), Rc::clone(&keywords.default_ident));
    idmap.insert("do".to_string(), Rc::clone(&keywords.do_ident));
    idmap.insert("double".to_string(), Rc::clone(&keywords.double_ident));
    idmap.insert("else".to_string(), Rc::clone(&keywords.else_ident));
    idmap.insert("enum".to_string(), Rc::clone(&keywords.enum_ident));
    idmap.insert("extern".to_string(), Rc::clone(&keywords.extern_ident));
    idmap.insert("float".to_string(), Rc::clone(&keywords.float_ident));
    idmap.insert("for".to_string(), Rc::clone(&keywords.for_ident));
    idmap.insert("goto".to_string(), Rc::clone(&keywords.goto_ident));
    idmap.insert("if".to_string(), Rc::clone(&keywords.if_ident));
    idmap.insert("inline".to_string(), Rc::clone(&keywords.inline_ident));
    idmap.insert("int".to_string(), Rc::clone(&keywords.int_ident));
    idmap.insert("long".to_string(), Rc::clone(&keywords.long_ident));
    idmap.insert("register".to_string(), Rc::clone(&keywords.register_ident));
    idmap.insert("restrict".to_string(), Rc::clone(&keywords.restrict_ident));
    idmap.insert("return".to_string(), Rc::clone(&keywords.return_ident));
    idmap.insert("short".to_string(), Rc::clone(&keywords.short_ident));
    idmap.insert("signed".to_string(), Rc::clone(&keywords.signed_ident));
    idmap.insert("sizeof".to_string(), Rc::clone(&keywords.sizeof_ident));
    idmap.insert("static".to_string(), Rc::clone(&keywords.static_ident));
    idmap.insert("struct".to_string(), Rc::clone(&keywords.struct_ident));
    idmap.insert("switch".to_string(), Rc::clone(&keywords.switch_ident));
    idmap.insert("typedef".to_string(), Rc::clone(&keywords.typedef_ident));
    idmap.insert("union".to_string(), Rc::clone(&keywords.union_ident));
    idmap.insert("unsigned".to_string(), Rc::clone(&keywords.unsigned_ident));
    idmap.insert("void".to_string(), Rc::clone(&keywords.void_ident));
    idmap.insert("volatile".to_string(), Rc::clone(&keywords.volatile_ident));
    idmap.insert("while".to_string(), Rc::clone(&keywords.while_ident));
    idmap.insert("_Alignas".to_string(), Rc::clone(&keywords._Alignas_ident));
    idmap.insert("_Alignof".to_string(), Rc::clone(&keywords._Alignof_ident));
    idmap.insert("_Atomic".to_string(), Rc::clone(&keywords._Atomic_ident));
    idmap.insert("_Bool".to_string(), Rc::clone(&keywords._Bool_ident));
    idmap.insert("_Complex".to_string(), Rc::clone(&keywords._Complex_ident));
    idmap.insert("_Decimal128".to_string(), Rc::clone(&keywords._Decimal128_ident));
    idmap.insert("_Decimal32".to_string(), Rc::clone(&keywords._Decimal32_ident));
    idmap.insert("_Decimal64".to_string(), Rc::clone(&keywords._Decimal64_ident));
    idmap.insert("_Generic".to_string(), Rc::clone(&keywords._Generic_ident));
    idmap.insert("_Imaginary".to_string(), Rc::clone(&keywords._Imaginary_ident));
    idmap.insert("_Noreturn".to_string(), Rc::clone(&keywords._Noreturn_ident));
    idmap.insert("_Static_assert".to_string(), Rc::clone(&keywords._Static_assert_ident));
    idmap.insert("_Thread_local".to_string(), Rc::clone(&keywords._Thread_local_ident));
    idmap.insert("asm".to_string(), Rc::clone(&keywords.asm_ident));
    idmap.insert("__asm".to_string(), Rc::clone(&keywords.__asm_ident));
    idmap.insert("__asm__".to_string(), Rc::clone(&keywords.__asm___ident));
    idmap.insert("__alignof".to_string(), Rc::clone(&keywords.__alignof_ident));
    idmap.insert("__alignof__".to_string(), Rc::clone(&keywords.__alignof___ident));
    idmap.insert("__attribute".to_string(), Rc::clone(&keywords.__attribute_ident));
    idmap.insert("__attribute__".to_string(), Rc::clone(&keywords.__attribute___ident));
    idmap.insert("__complex".to_string(), Rc::clone(&keywords.__complex_ident));
    idmap.insert("__complex__".to_string(), Rc::clone(&keywords.__complex___ident));
    idmap.insert("__const".to_string(), Rc::clone(&keywords.__const_ident));
    idmap.insert("__const__".to_string(), Rc::clone(&keywords.__const___ident));
    idmap.insert("__inline".to_string(), Rc::clone(&keywords.__inline_ident));
    idmap.insert("__inline__".to_string(), Rc::clone(&keywords.__inline___ident));
    idmap.insert("__restrict".to_string(), Rc::clone(&keywords.__restrict_ident));
    idmap.insert("__restrict__".to_string(), Rc::clone(&keywords.__restrict___ident));
    idmap.insert("__signed".to_string(), Rc::clone(&keywords.__signed_ident));
    idmap.insert("__signed__".to_string(), Rc::clone(&keywords.__signed___ident));
    idmap.insert("__thread".to_string(), Rc::clone(&keywords.__thread_ident));
    idmap.insert("typeof".to_string(), Rc::clone(&keywords.typeof_ident));
    idmap.insert("__typeof".to_string(), Rc::clone(&keywords.__typeof_ident));
    idmap.insert("__typeof__".to_string(), Rc::clone(&keywords.__typeof___ident));
    idmap.insert("__volatile".to_string(), Rc::clone(&keywords.__volatile_ident));
    idmap.insert("__volatile__".to_string(), Rc::clone(&keywords.__volatile___ident));
    idmap.insert("__label__".to_string(), Rc::clone(&keywords.__label___ident));
    idmap.insert("__extension__".to_string(), Rc::clone(&keywords.__extension___ident));

    return idmap;
}

pub fn make_maps() -> HashMap<&'static str, T> {
    let mut punct_map: HashMap<&str, T> = HashMap::new();

    // " ... && -= >= ~ + ; ] <: "
    // " <<= &= -> >> % , < ^ :> "
    // " >>= *= /= ^= & - = { <% "
    // " != ++ << |= ( . > | %> "
    // " %= += <= || ) / ? } %: "
    // " ## -- == ! * : [ # %:%: "

    // c-digraphs
    punct_map.insert("%:%:", T::T_SHARP_SHARP);
    punct_map.insert("<:", T::T_LEFT_BRACKET);
    punct_map.insert(":>", T::T_RIGHT_BRACKET);
    punct_map.insert("<%", T::T_LEFT_BRACE);
    punct_map.insert("%>", T::T_RIGHT_BRACE);
    punct_map.insert("%:", T::T_SHARP);

    punct_map.insert(">>=", T::T_RSHIFT_EQUAL);
    punct_map.insert("<<=", T::T_LSHIFT_EQUAL);
    punct_map.insert("...", T::T_DOT_DOT_DOT);
    punct_map.insert("->", T::T_ARROW);
    punct_map.insert("--", T::T_MINUS_MINUS);
    punct_map.insert("-=", T::T_MINUS_EQUAL);
    punct_map.insert("!=", T::T_NE);
    punct_map.insert("..", T::T_DOT_DOT);
    punct_map.insert("*=", T::T_TIMES_EQUAL);
    punct_map.insert("/=", T::T_DIVIDE_EQUAL);
    punct_map.insert("&=", T::T_AND_EQUAL);
    punct_map.insert("&&", T::T_AND_AND);
    punct_map.insert("##", T::T_SHARP_SHARP);
    punct_map.insert("%=", T::T_PERCENT_EQUAL);
    punct_map.insert("^=", T::T_XOR_EQUAL);
    punct_map.insert("++", T::T_PLUS_PLUS);
    punct_map.insert("+=", T::T_PLUS_EQUAL);
    punct_map.insert("<=", T::T_LE);
    punct_map.insert("<<", T::T_LSHIFT);
    punct_map.insert("==", T::T_EQ);
    punct_map.insert(">=", T::T_GE);
    punct_map.insert(">>", T::T_RSHIFT);
    punct_map.insert("||", T::T_OR_OR);
    punct_map.insert("|=", T::T_OR_EQUAL);
    punct_map.insert(",", T::T_COMMA);
    punct_map.insert("-", T::T_MINUS);
    punct_map.insert(";", T::T_SEMI_COLON);
    punct_map.insert(":", T::T_COLON);
    punct_map.insert("!", T::T_EXCLAMATION);
    punct_map.insert("?", T::T_QUESTION);
    punct_map.insert(".", T::T_DOT);
    punct_map.insert("(", T::T_LEFT_PAREN);
    punct_map.insert(")", T::T_RIGHT_PAREN);
    punct_map.insert("[", T::T_LEFT_BRACKET);
    punct_map.insert("]", T::T_RIGHT_BRACKET);
    punct_map.insert("{", T::T_LEFT_BRACE);
    punct_map.insert("}", T::T_RIGHT_BRACE);
    punct_map.insert("*", T::T_TIMES);
    punct_map.insert("/", T::T_DIVIDE);
    punct_map.insert("&", T::T_AND);
    punct_map.insert("#", T::T_SHARP);
    punct_map.insert("%", T::T_PERCENT);
    punct_map.insert("^", T::T_XOR);
    punct_map.insert("+", T::T_PLUS);
    punct_map.insert("<", T::T_LT);
    punct_map.insert("=", T::T_ASSIGN);
    punct_map.insert(">", T::T_GT);
    punct_map.insert("|", T::T_OR);
    punct_map.insert("~", T::T_TILDE);

    // not operators, other ascii characters we have to recognize
    punct_map.insert("$", T::T_DOLLAR_SIGN);
    punct_map.insert("@", T::T_AT_SIGN);
    punct_map.insert("`", T::T_GRAVE_ACCENT);
    punct_map.insert("\\", T::T_BACKSLASH);

    return punct_map;
}
