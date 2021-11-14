use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "जंग" => "zung",
        "मुख्य" => "main",
        "कार्य" => "fn",
        "पंक्तिछाप" => "println",
        "छाप" => "print",
        "बाहरी" => "extern",
        "टोकरा" => "crate",
        "प्रयोग" => "use",
        "मानक" => "std",
        "समस्या" => "Err",
        "थीक" => "Ok",
        "विकल्प" => "Option",
        "कुछ" => "Some",
        "परिणाम" => "Result",
        "स्वयंरूपेण" => "Self",
        "प्रतीक्शा" => "await",
        "जैसे" => "as",
        "नित्य" => "const",
        "असुरक्शित" => "unsafe",
        "प्र्त्येक" => "for",
        "मध्य" => "in",
        "से" => "from",
        "असत्स" => "false",
        "डालो" => "insert",
        "प्राप्त" => "get",
        "अनुमति" => "allow",
        "घबड़ाहट" | "भक" | "अरे" => "panic",
        "पर्वर्तन्शील" => "mut",
        "नया" | "नयी" => "new",
        "जहान" => "where",
        "वापसी" => "return",
        "मेल" => "match",
        "अगर" => "if",
        "निरूपित" => "let",
        "स्थिर" => "static",
        "संरचना" => "struct",
        "आपेक्षा" => "expect",
        "जब_तक" => "while",
        "सत्य" => "true",
        "में" => "into",
        "सार्वजनिक" => "pub",
        "खोलो" => "unwrap",
        "कार्यरूप" => "impl",
        "नहि_तो" => "else",
        "स्थानांतरण" => "move",
        "संकेत" => "ref",
        "डोरी" => "String",
        "शब्दकोश" => "HashMap",
        "परिगणन" => "enum",
        "अभाव" => "None",
        "अन्त" => "break",
        "सन्केतरूपेण" => "as_ref",
        "स्वयम्" => "self",
        "विशेषता" => "trait",
        "पूर्णांक_16" => "u16",
        "पूर्णांक_32" => "u32",
        "पूर्णांक_64" => "u64",
        "वार्तालाप" => "io",
        "पंक्ति_पढो" => "read_line",
        "विरासत" => "derive",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn zung(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
