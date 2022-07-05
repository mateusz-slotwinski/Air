use crate::lexer::lexer::{to_char, to_float, to_int, to_string};
use logos::Logos;

#[derive(Debug, Clone, Logos, PartialEq)]
pub enum Token {
  /* ------------------------ COMMON ------------------------ */
  #[token(",")]
  Comma,
  #[token(":")]
  Colon,
  #[token(".")]
  Dot,
  /* ------------------------ BRACKETS ------------------------ */
  #[token("(")]
  BracketLeft,
  #[token(")")]
  BracketRight,
  #[token("[")]
  BracketLeftSquare,
  #[token("]")]
  BracketRightSquare,
  #[token("{")]
  BracketLeftCurly,
  #[token("}")]
  BracketRightCurly,

  /* ------------------------ OPERATORS ------------------------ */
  #[token("+")]
  OperatorPlus,
  #[token("-")]
  OperatorMinus,
  #[token("*")]
  OperatorMulti,
  #[token("**")]
  OperatorPower,
  #[token("/")]
  OperatorSlash,
  #[token("%")]
  OperatorPercent,
  #[token("++")]
  OperatorIncrement,
  #[token("--")]
  OperatorDecrement,

  #[token("=")]
  OperatorAssign,
  #[token("+=")]
  OperatorAssignAddition,
  #[token("-=")]
  OperatorAssignSubtraction,
  #[token("*=")]
  OperatorAssignMultiplication,
  #[token("/=")]
  OperatorAssignDivision,
  #[token(":=")]
  OperatorDeclareAssign,

  // Logic
  #[token("==")]
  OperatorEquals,
  #[token("?=")]
  OperatorAlmostEquals,
  #[token("!=")]
  OperatorNotEquals,
  #[token(">")]
  OperatorGreater,
  #[token("<")]
  OperatorLesser,
  #[token(">=")]
  OperatorGreaterOrEqual,
  #[token("<=")]
  OperatorLesserOrEqual,
  #[token("&&")]
  OperatorAnd,
  #[token("||")]
  OperatorOr,
  #[token("!")]
  OperatorNot,

  // Bit
  #[token("&")]
  OperatorBitAnd,
  #[token("|")]
  OperatorBitOr,
  #[token("~")]
  OperatorBitNot,
  #[token("^")]
  OperatorBitXor,
  #[token("<<<")]
  OperatorBitLeft,
  #[token(">>>")]
  OperatorBitRight,

  // Tenary
  // OperatorTenaryIf,
  // OperatorTenaryElse,

  // Pointer // ? Maybe
  // OperatorPointerAdress,   // &
  // OperatorPointerOperator, // *

  // Others operators
  // OperatorGlobal,        // $
  #[token("@")]
  OperatorDecorator,
  #[token("->")]
  OperatorArrow,
  #[token("=>")]
  OperatorFlatArrow,
  #[token("~>")]
  OperatorQuasiArrow,
  #[token(">>")]
  OperatorPipe,
  #[token("<<")]
  OperatorPipeReverse,
  #[token("..")]
  OperatorRange,
  #[token("...")]
  OperatorSpread,
  #[token("_")]
  OperatorUnderscore,
  // OperatorNullable,
  // OperatorNullForgiving,
  #[token("typeof")]
  OperatorTypeOf,
  #[token("sizeof")]
  OperatorSizeOf,

  /* ----------------------- VARIABLES  ----------------------- */
  #[token("var")]
  VariablesVar,
  #[token("let")]
  VariablesLet,
  #[token("const")]
  VariablesConst,

  /* ----------------------- CONDITIONS  ----------------------- */
  #[token("if")]
  ConditionsIf,
  #[token("else")]
  ConditionsElse,
  #[token("switch")]
  ConditionsSwitch,
  #[token("case")]
  ConditionsCase,

  /* ------------------------    LOOP    ------------------------ */
  #[token("for")]
  LoopFor,
  #[token("in")]
  LoopIn,
  #[token("not in")]
  LoopNotIn,
  #[token("while")]
  LoopWhile,
  #[token("loop")]
  LoopInf,
  #[token("break")]
  LoopBreak,
  #[token("continue")]
  LoopContinue,

  /* ------------------------  FUNCTIONS  ------------------------ */
  #[token("func")]
  FunctionFunc,

  #[token("return")]
  FunctionReturn,

  #[token("async")]
  FunctionAsync,
  #[token("await")]
  FunctionAwait,

  /* ------------------------  OBJECTS  ------------------------ */
  #[token("class")]
  ObjectsClass,
  #[token("struct")]
  ObjectsStruct,
  #[token("interface")]
  ObjectsInterface,
  #[token("enum")]
  ObjectsEnum,
  #[token("new")]
  ObjectsNew,
  #[token("super")]
  ObjectsSuper,
  #[token("static")]
  ObjectsStatic,
  #[token("public")]
  ObjectsPublic,
  #[token("readonly")]
  ObjectsReadonly,
  #[token("private")]
  ObjectsPrivate,
  #[token("this")]
  This,

  /* ------------------------  ERRORS  ------------------------ */
  #[token("try")]
  ErrorsTry,
  #[token("catch")]
  ErrorsCatch,
  #[token("finally")]
  ErrorsFinally,
  #[token("throw")]
  ErrorsThrow,

  /* ------------------------  MODULES  ------------------------ */
  #[token("import")]
  ModulesImport,
  #[token("export")]
  ModulesExport,
  #[token("from")]
  ModulesFrom,
  #[token("as")]
  ModulesAs,

  /*
    import IO, Math, Strings, Utilities
    ? When no quotes, imports standard libs
    import "world"
    import * from "world"
    ? Both forms are equivalent and allow you to import full module from world.air
    ? If you want to use variable hello from world module type world.hello
    ! Important - import files relative to the air.module file
    import { hello, hi, wellcome } from "world"
    ? If you want to import only few variables you can do something like that
    ? It imports hello variable (must have export keyword) from world.air file
    ? if you want to use variable you still need to type world.variable
  */

  /* ------------------------ DATA TYPES ------------------------ */
  #[token("void")]
  TypeVoid,
  #[token("char")]
  TypeChar,
  #[token("string")]
  TypeString,
  #[token("int")]
  TypeInt,
  #[token("float")]
  TypeFloat,
  #[token("bool")]
  TypeBool,

  // FUNCTION, DECORATOR
  // ARRAY, TUPPLE, MAP, OBJECT

  /* ------------------------ OTHERS ------------------------ */
  #[token("false")]
  False,
  #[token("true")]
  True,
  #[token("null")]
  Null,

  #[regex(r"[a-zA-Z_]+", to_string)]
  Identifier(String),

  #[regex(r"[0-9]+", priority = 2, callback = to_int)]
  Int(i64),

  #[regex(r"([0-9]+[.])?[0-9]+", to_float)]
  Float(f64),

  #[regex(r"'[^ \n\t\f]'", to_char)]
  #[regex(r##""[^ \n\t\f]""##, to_char)]
  Char(char),

  #[regex(r##""([^"]*)""##, to_string)]
  #[regex(r##"'([^']*)'"##, to_string)]
  String(String),

  #[error]
  #[token("\r\n", logos::skip)]
  #[regex(r"//[^\n]*", logos::skip)]
  #[regex(r"[ \t\n\f]+", logos::skip)]
  Error,
}
