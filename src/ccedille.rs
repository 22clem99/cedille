//! Module for the lexer and parser for CCedille
//! WARNING: this file has been generated by
//! Hime Parser Generator 3.5.1.0

use std::io::Read;

use hime_redist::ast::AstNode;
use hime_redist::errors::ParseErrors;
use hime_redist::lexers::automaton::Automaton;
use hime_redist::lexers::impls::ContextFreeLexer;
use hime_redist::parsers::Parser;
use hime_redist::parsers::lrk::LRkAutomaton;
use hime_redist::parsers::lrk::LRkParser;
use hime_redist::result::ParseResult;
use hime_redist::symbols::SemanticBody;
use hime_redist::symbols::SemanticElementTrait;
use hime_redist::symbols::Symbol;
use hime_redist::text::Text;
use hime_redist::tokens::TokenRepository;
use hime_redist::utils::iterable::Iterable;

/// Static resource for the serialized lexer automaton
const LEXER_AUTOMATON: &[u8] = include_bytes!("CCedilleLexer.bin");

/// The unique identifier for terminal EGAL
pub const ID_TERMINAL_EGAL: u32 = 0x0003;
/// The unique identifier for terminal PLUS
pub const ID_TERMINAL_PLUS: u32 = 0x0004;
/// The unique identifier for terminal MOINS
pub const ID_TERMINAL_MOINS: u32 = 0x0005;
/// The unique identifier for terminal FOIS
pub const ID_TERMINAL_FOIS: u32 = 0x0006;
/// The unique identifier for terminal DIVISE
pub const ID_TERMINAL_DIVISE: u32 = 0x0007;
/// The unique identifier for terminal MODULO
pub const ID_TERMINAL_MODULO: u32 = 0x0008;
/// The unique identifier for terminal ET
pub const ID_TERMINAL_ET: u32 = 0x0009;
/// The unique identifier for terminal OU
pub const ID_TERMINAL_OU: u32 = 0x000A;
/// The unique identifier for terminal NON
pub const ID_TERMINAL_NON: u32 = 0x000B;
/// The unique identifier for terminal GRAND
pub const ID_TERMINAL_GRAND: u32 = 0x000C;
/// The unique identifier for terminal PETIT
pub const ID_TERMINAL_PETIT: u32 = 0x000D;
/// The unique identifier for terminal TANT
pub const ID_TERMINAL_TANT: u32 = 0x000E;
/// The unique identifier for terminal QUE
pub const ID_TERMINAL_QUE: u32 = 0x000F;
/// The unique identifier for terminal FAIRE
pub const ID_TERMINAL_FAIRE: u32 = 0x0010;
/// The unique identifier for terminal ALORS
pub const ID_TERMINAL_ALORS: u32 = 0x0011;
/// The unique identifier for terminal VRAI
pub const ID_TERMINAL_VRAI: u32 = 0x0012;
/// The unique identifier for terminal FAUX
pub const ID_TERMINAL_FAUX: u32 = 0x0013;
/// The unique identifier for terminal FLOTANT
pub const ID_TERMINAL_FLOTANT: u32 = 0x0014;
/// The unique identifier for terminal ENTIER
pub const ID_TERMINAL_ENTIER: u32 = 0x0015;
/// The unique identifier for terminal CHAINE
pub const ID_TERMINAL_CHAINE: u32 = 0x0016;
/// The unique identifier for terminal DIFFERENT
pub const ID_TERMINAL_DIFFERENT: u32 = 0x0017;
/// The unique identifier for terminal DANS
pub const ID_TERMINAL_DANS: u32 = 0x0018;
/// The unique identifier for terminal ENTREE
pub const ID_TERMINAL_ENTREE: u32 = 0x0019;
/// The unique identifier for terminal SORTIE
pub const ID_TERMINAL_SORTIE: u32 = 0x001A;
/// The unique identifier for terminal FONCTION
pub const ID_TERMINAL_FONCTION: u32 = 0x001B;
/// The unique identifier for terminal STRUCTURE
pub const ID_TERMINAL_STRUCTURE: u32 = 0x001C;
/// The unique identifier for terminal IMPORTER
pub const ID_TERMINAL_IMPORTER: u32 = 0x001D;
/// The unique identifier for terminal OBRACE
pub const ID_TERMINAL_OBRACE: u32 = 0x001E;
/// The unique identifier for terminal CBRACE
pub const ID_TERMINAL_CBRACE: u32 = 0x001F;
/// The unique identifier for terminal OPARENT
pub const ID_TERMINAL_OPARENT: u32 = 0x0020;
/// The unique identifier for terminal CPARENT
pub const ID_TERMINAL_CPARENT: u32 = 0x0021;
/// The unique identifier for terminal SEMI
pub const ID_TERMINAL_SEMI: u32 = 0x0022;
/// The unique identifier for terminal COMMA
pub const ID_TERMINAL_COMMA: u32 = 0x0023;
/// The unique identifier for terminal SLASH
pub const ID_TERMINAL_SLASH: u32 = 0x0024;
/// The unique identifier for terminal PERCENT
pub const ID_TERMINAL_PERCENT: u32 = 0x0025;
/// The unique identifier for terminal EXCLAM
pub const ID_TERMINAL_EXCLAM: u32 = 0x0026;
/// The unique identifier for terminal DOT
pub const ID_TERMINAL_DOT: u32 = 0x0027;
/// The unique identifier for terminal WHITE_SPACE
pub const ID_TERMINAL_WHITE_SPACE: u32 = 0x0028;
/// The unique identifier for terminal SEPARATOR
pub const ID_TERMINAL_SEPARATOR: u32 = 0x0029;
/// The unique identifier for terminal LETTER
pub const ID_TERMINAL_LETTER: u32 = 0x002A;
/// The unique identifier for terminal DIGIT
pub const ID_TERMINAL_DIGIT: u32 = 0x002B;
/// The unique identifier for terminal IDENT
pub const ID_TERMINAL_IDENT: u32 = 0x002C;
/// The unique identifier for terminal POSITIVE_DIGIT
pub const ID_TERMINAL_POSITIVE_DIGIT: u32 = 0x002D;
/// The unique identifier for terminal S_INT
pub const ID_TERMINAL_S_INT: u32 = 0x002E;
/// The unique identifier for terminal DEC
pub const ID_TERMINAL_DEC: u32 = 0x002F;
/// The unique identifier for terminal FLOAT
pub const ID_TERMINAL_FLOAT: u32 = 0x0030;
/// The unique identifier for terminal EOL
pub const ID_TERMINAL_EOL: u32 = 0x0031;
/// The unique identifier for terminal STRING
pub const ID_TERMINAL_STRING: u32 = 0x0032;
/// The unique identifier for terminal COMMENT
pub const ID_TERMINAL_COMMENT: u32 = 0x0033;

/// The unique identifier for the default context
pub const CONTEXT_DEFAULT: u16 = 0;

/// The collection of terminals matched by this lexer
/// The terminals are in an order consistent with the automaton,
/// so that terminal indices in the automaton can be used to retrieve the terminals in this table
const TERMINALS: &[Symbol] = &[
    Symbol { id: 0x0001, name: "??" },
    Symbol { id: 0x0002, name: "$" },
    Symbol { id: 0x0003, name: "EGAL" },
    Symbol { id: 0x0004, name: "PLUS" },
    Symbol { id: 0x0005, name: "MOINS" },
    Symbol { id: 0x0006, name: "FOIS" },
    Symbol { id: 0x0007, name: "DIVISE" },
    Symbol { id: 0x0008, name: "MODULO" },
    Symbol { id: 0x0009, name: "ET" },
    Symbol { id: 0x000A, name: "OU" },
    Symbol { id: 0x000B, name: "NON" },
    Symbol { id: 0x000C, name: "GRAND" },
    Symbol { id: 0x000D, name: "PETIT" },
    Symbol { id: 0x000E, name: "TANT" },
    Symbol { id: 0x000F, name: "QUE" },
    Symbol { id: 0x0010, name: "FAIRE" },
    Symbol { id: 0x0011, name: "ALORS" },
    Symbol { id: 0x0012, name: "VRAI" },
    Symbol { id: 0x0013, name: "FAUX" },
    Symbol { id: 0x0014, name: "FLOTANT" },
    Symbol { id: 0x0015, name: "ENTIER" },
    Symbol { id: 0x0016, name: "CHAINE" },
    Symbol { id: 0x0017, name: "DIFFERENT" },
    Symbol { id: 0x0018, name: "DANS" },
    Symbol { id: 0x0019, name: "ENTREE" },
    Symbol { id: 0x001A, name: "SORTIE" },
    Symbol { id: 0x001B, name: "FONCTION" },
    Symbol { id: 0x001C, name: "STRUCTURE" },
    Symbol { id: 0x001D, name: "IMPORTER" },
    Symbol { id: 0x001E, name: "OBRACE" },
    Symbol { id: 0x001F, name: "CBRACE" },
    Symbol { id: 0x0020, name: "OPARENT" },
    Symbol { id: 0x0021, name: "CPARENT" },
    Symbol { id: 0x0022, name: "SEMI" },
    Symbol { id: 0x0023, name: "COMMA" },
    Symbol { id: 0x0024, name: "SLASH" },
    Symbol { id: 0x0025, name: "PERCENT" },
    Symbol { id: 0x0026, name: "EXCLAM" },
    Symbol { id: 0x0027, name: "DOT" },
    Symbol { id: 0x0028, name: "WHITE_SPACE" },
    Symbol { id: 0x0029, name: "SEPARATOR" },
    Symbol { id: 0x002A, name: "LETTER" },
    Symbol { id: 0x002B, name: "DIGIT" },
    Symbol { id: 0x002C, name: "IDENT" },
    Symbol { id: 0x002D, name: "POSITIVE_DIGIT" },
    Symbol { id: 0x002E, name: "S_INT" },
    Symbol { id: 0x002F, name: "DEC" },
    Symbol { id: 0x0030, name: "FLOAT" },
    Symbol { id: 0x0031, name: "EOL" },
    Symbol { id: 0x0032, name: "STRING" },
    Symbol { id: 0x0033, name: "COMMENT" }];

/// Creates a new lexer
fn new_lexer<'a>(
    repository: TokenRepository<'a>,
    errors: &'a mut ParseErrors
) -> ContextFreeLexer<'a> {
    let automaton = Automaton::new(LEXER_AUTOMATON);
    ContextFreeLexer::new(repository, errors, automaton, 0x0029)
}

/// Static resource for the serialized parser automaton
const PARSER_AUTOMATON: &[u8] = include_bytes!("CCedilleParser.bin");

/// The unique identifier for variable inst
pub const ID_VARIABLE_INST: u32 = 0x0034;


/// The collection of variables matched by this parser
/// The variables are in an order consistent with the automaton,
/// so that variable indices in the automaton can be used to retrieve the variables in this table
const VARIABLES: &[Symbol] = &[
    Symbol { id: 0x0034, name: "inst" },
    Symbol { id: 0x0035, name: "__VAxiom" }];

/// The collection of virtuals matched by this parser
/// The virtuals are in an order consistent with the automaton,
/// so that virtual indices in the automaton can be used to retrieve the virtuals in this table
const VIRTUALS: &[Symbol] = &[
];

/// Parses the specified string with this parser
pub fn parse_string(input: &str) -> ParseResult {
    let text = Text::new(input);
    parse_text(text)
}

/// Parses the specified stream of UTF-16 with this parser
pub fn parse_utf16(input: &mut dyn Read, big_endian: bool) -> ParseResult {
    let text = Text::from_utf16_stream(input, big_endian);
    parse_text(text)
}

/// Parses the specified stream of UTF-16 with this parser
pub fn parse_utf8(input: &mut dyn Read) -> ParseResult {
    let text = Text::from_utf8_stream(input);
    parse_text(text)
}

/// Parses the specified text with this parser
fn parse_text(text: Text) -> ParseResult {
    let mut my_actions = |_index: usize, _head: Symbol, _body: &dyn SemanticBody| ();
    let mut result = ParseResult::new(TERMINALS, VARIABLES, VIRTUALS, text);
    {
        let data = result.get_parsing_data();
        let mut lexer = new_lexer(data.0, data.1);
        let automaton = LRkAutomaton::new(PARSER_AUTOMATON);
        let mut parser = LRkParser::new(&mut lexer, automaton, data.2, &mut my_actions);
        parser.parse();
    }
    result
}

/// Visitor interface
pub trait Visitor {
    fn on_terminal_egal(&self, _node: &AstNode) {}
    fn on_terminal_plus(&self, _node: &AstNode) {}
    fn on_terminal_moins(&self, _node: &AstNode) {}
    fn on_terminal_fois(&self, _node: &AstNode) {}
    fn on_terminal_divise(&self, _node: &AstNode) {}
    fn on_terminal_modulo(&self, _node: &AstNode) {}
    fn on_terminal_et(&self, _node: &AstNode) {}
    fn on_terminal_ou(&self, _node: &AstNode) {}
    fn on_terminal_non(&self, _node: &AstNode) {}
    fn on_terminal_grand(&self, _node: &AstNode) {}
    fn on_terminal_petit(&self, _node: &AstNode) {}
    fn on_terminal_tant(&self, _node: &AstNode) {}
    fn on_terminal_que(&self, _node: &AstNode) {}
    fn on_terminal_faire(&self, _node: &AstNode) {}
    fn on_terminal_alors(&self, _node: &AstNode) {}
    fn on_terminal_vrai(&self, _node: &AstNode) {}
    fn on_terminal_faux(&self, _node: &AstNode) {}
    fn on_terminal_flotant(&self, _node: &AstNode) {}
    fn on_terminal_entier(&self, _node: &AstNode) {}
    fn on_terminal_chaine(&self, _node: &AstNode) {}
    fn on_terminal_different(&self, _node: &AstNode) {}
    fn on_terminal_dans(&self, _node: &AstNode) {}
    fn on_terminal_entree(&self, _node: &AstNode) {}
    fn on_terminal_sortie(&self, _node: &AstNode) {}
    fn on_terminal_fonction(&self, _node: &AstNode) {}
    fn on_terminal_structure(&self, _node: &AstNode) {}
    fn on_terminal_importer(&self, _node: &AstNode) {}
    fn on_terminal_obrace(&self, _node: &AstNode) {}
    fn on_terminal_cbrace(&self, _node: &AstNode) {}
    fn on_terminal_oparent(&self, _node: &AstNode) {}
    fn on_terminal_cparent(&self, _node: &AstNode) {}
    fn on_terminal_semi(&self, _node: &AstNode) {}
    fn on_terminal_comma(&self, _node: &AstNode) {}
    fn on_terminal_slash(&self, _node: &AstNode) {}
    fn on_terminal_percent(&self, _node: &AstNode) {}
    fn on_terminal_exclam(&self, _node: &AstNode) {}
    fn on_terminal_dot(&self, _node: &AstNode) {}
    fn on_terminal_white_space(&self, _node: &AstNode) {}
    fn on_terminal_separator(&self, _node: &AstNode) {}
    fn on_terminal_letter(&self, _node: &AstNode) {}
    fn on_terminal_digit(&self, _node: &AstNode) {}
    fn on_terminal_ident(&self, _node: &AstNode) {}
    fn on_terminal_positive_digit(&self, _node: &AstNode) {}
    fn on_terminal_s_int(&self, _node: &AstNode) {}
    fn on_terminal_dec(&self, _node: &AstNode) {}
    fn on_terminal_float(&self, _node: &AstNode) {}
    fn on_terminal_eol(&self, _node: &AstNode) {}
    fn on_terminal_string(&self, _node: &AstNode) {}
    fn on_terminal_comment(&self, _node: &AstNode) {}
    fn on_variable_inst(&self, _node: &AstNode) {}
}

/// Walk the AST of a result using a visitor
pub fn visit(result: &ParseResult, visitor: &dyn Visitor) {
    let ast = result.get_ast();
    let root = ast.get_root();
    visit_ast_node(root, visitor);
}

/// Walk the sub-AST from the specified node using a visitor
pub fn visit_ast_node<'a>(node: AstNode<'a>, visitor: &dyn Visitor) {
    let children = node.children();
    for child in children.iter() {
        visit_ast_node(child, visitor);
    }
    match node.get_symbol().id {
        0x0003 => visitor.on_terminal_egal(&node),
        0x0004 => visitor.on_terminal_plus(&node),
        0x0005 => visitor.on_terminal_moins(&node),
        0x0006 => visitor.on_terminal_fois(&node),
        0x0007 => visitor.on_terminal_divise(&node),
        0x0008 => visitor.on_terminal_modulo(&node),
        0x0009 => visitor.on_terminal_et(&node),
        0x000A => visitor.on_terminal_ou(&node),
        0x000B => visitor.on_terminal_non(&node),
        0x000C => visitor.on_terminal_grand(&node),
        0x000D => visitor.on_terminal_petit(&node),
        0x000E => visitor.on_terminal_tant(&node),
        0x000F => visitor.on_terminal_que(&node),
        0x0010 => visitor.on_terminal_faire(&node),
        0x0011 => visitor.on_terminal_alors(&node),
        0x0012 => visitor.on_terminal_vrai(&node),
        0x0013 => visitor.on_terminal_faux(&node),
        0x0014 => visitor.on_terminal_flotant(&node),
        0x0015 => visitor.on_terminal_entier(&node),
        0x0016 => visitor.on_terminal_chaine(&node),
        0x0017 => visitor.on_terminal_different(&node),
        0x0018 => visitor.on_terminal_dans(&node),
        0x0019 => visitor.on_terminal_entree(&node),
        0x001A => visitor.on_terminal_sortie(&node),
        0x001B => visitor.on_terminal_fonction(&node),
        0x001C => visitor.on_terminal_structure(&node),
        0x001D => visitor.on_terminal_importer(&node),
        0x001E => visitor.on_terminal_obrace(&node),
        0x001F => visitor.on_terminal_cbrace(&node),
        0x0020 => visitor.on_terminal_oparent(&node),
        0x0021 => visitor.on_terminal_cparent(&node),
        0x0022 => visitor.on_terminal_semi(&node),
        0x0023 => visitor.on_terminal_comma(&node),
        0x0024 => visitor.on_terminal_slash(&node),
        0x0025 => visitor.on_terminal_percent(&node),
        0x0026 => visitor.on_terminal_exclam(&node),
        0x0027 => visitor.on_terminal_dot(&node),
        0x0028 => visitor.on_terminal_white_space(&node),
        0x0029 => visitor.on_terminal_separator(&node),
        0x002A => visitor.on_terminal_letter(&node),
        0x002B => visitor.on_terminal_digit(&node),
        0x002C => visitor.on_terminal_ident(&node),
        0x002D => visitor.on_terminal_positive_digit(&node),
        0x002E => visitor.on_terminal_s_int(&node),
        0x002F => visitor.on_terminal_dec(&node),
        0x0030 => visitor.on_terminal_float(&node),
        0x0031 => visitor.on_terminal_eol(&node),
        0x0032 => visitor.on_terminal_string(&node),
        0x0033 => visitor.on_terminal_comment(&node),
        0x0034 => visitor.on_variable_inst(&node),
        _ => ()
    };
}
