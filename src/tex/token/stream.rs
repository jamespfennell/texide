//! Definition of the token stream trait and some general-purpose implementations.
//!
//! The simplest example of a stream is a vector of tokens. However, streams are more general
//! than this and can encompass situations in which the full contents cannot be determined in
//! advance. The classic example of the latter kind of stream comes from the following LaTeX
//! snippet:
//! ```tex
//! \makeatletter \do@
//! ```
//! Assuming the default TeX catcode map, if we were to parse this input all at once we would
//! get three tokens: the control sequence `makeatletter`, the control sequence `do`, and a
//! single character token with value `@` and catcode "other". This is not the correct result,
//! though: the first control sequence changes the tokenization rules such that `@` is now
//! an admissible character in the name of a control sequence. The correct input is thus
//! the control sequence `makeatletter` followed by the control sequence `do@`.
//!
//! This example demonstrates that one must be careful when processing streams. After reading
//! a token, all possible side effects must take place before the next token is read.
//!
//! # Getting the next token
//!
//! Tokens are consume from a stream using the `next` method. This method is almost the same
//! as the `next` method in Rust's iterator trait, except a stream can return an error.
//! ```
//! # use texide::tex::token::stream::VecStream;
//! # use texide::tex::token::stream::Stream;
//! # use texide::tex::token::token::Token;
//! let mut stream = VecStream::new(vec![
//!     Token::new_letter('a'),
//!     Token::new_letter('b'),
//!     Token::new_letter('c'),
//! ]);
//!
//! assert_eq!(stream.next().unwrap(), Some(Token::new_letter('a')));
//! assert_eq!(stream.next().unwrap(), Some(Token::new_letter('b')));
//! assert_eq!(stream.next().unwrap(), Some(Token::new_letter('c')));
//! assert_eq!(stream.next().unwrap(), None);
//! ```
//! As with iterators, a result of `Ok(None)` indicates that the stream is exhausted.
//!
//! # Peeking at the next token
//!
//! In many sitations it is necessary to examine the next token without consuming it; i.e.,
//! _peek_ at the next token. An example is reading an integer from a stream, in which one needs
//! to peek at the next token to see if it is a digit. Consuming the token with `next` is not
//! correct in this situation if the token is not a digit. The simplist way to peek is with
//! the `peek` method.
//! ```
//! # use texide::tex::token::stream::VecStream;
//! # use texide::tex::token::stream::Stream;
//! # use texide::tex::token::token::Token;
//! let mut stream = VecStream::new(vec![
//!     Token::new_letter('a'),
//!     Token::new_letter('b'),
//!     Token::new_letter('c'),
//! ]);
//!
//! assert_eq!(stream.peek().unwrap(), Some(&Token::new_letter('a')));
//! assert_eq!(stream.peek().unwrap(), Some(&Token::new_letter('a')));
//! assert_eq!(stream.next().unwrap(), Some(Token::new_letter('a')));
//! assert_eq!(stream.peek().unwrap(), Some(&Token::new_letter('b')));
//! ```
//! The `peek` method returns an immutable reference to the token: because the token is not
//! being consumed, ownership cannot be transferred as in `next`.
//!
//! The `peek` method must be idempotent: consecutive calls to `peek` with no intervening
//! change to the state or the stream must return the same result.
//!
//! For consumers, it is important to note that the peek method requires a mutable reference
//! to the stream. This is because some mutable processing may be needed in order to determine
//! what the next token is. For example:
//!
//! 1. When reading tokens from a file, peeking at the next token may involve reading more bytes
//!     from the file and thus mutating the file pointer. This mutations is easy to undo in
//!     general.
//!
//! 1. When performing expansion on a stream, the next token in the stream may need to be expanded
//!     rather than returned. The next token will be the first token in the expansion in this case,
//!     or the following token in the remaining stream if the expansion returns no tokens.
//!     This mutation is generally irreversable.
//!
//! # Immutable peeking
//!
//! Requiring a mutable borrow to peek at the stream is problematical for stream consumers.
//! The following code seems entirely reasonable: the stream is peeked at, and a lookup occurs
//! to check if the immutable token corresponds to an expansion control sequence:
//! ```compile_fail
//! let state_and_stream: dyn StateAndStream<State> = // some state and stream
//! let token = state_and_stream.stream().peek();
//! let is_expansion_command = match token.value {
//!     None => false,
//!     Some(ref name) => {
//!         state_and_stream.state().get_expansion_command(name).is_value()
//!     }
//! }
//! ```
//! This however does not compile: searching in the expansion map requires an immutable borrow
//! of the state (and thus the `StateAndStream`) which the borrow checker forbids because the
//! immutable reference to the token is keeping the mutable borrow of the stream alive.
//!
//! However, idempotency of the `peek` method means that in general there should be possible
//! to retrieve an immutable reference to the next token if a mutable peek has already occured.
//! In the worst case, stream implementations can maintain an internal cache of the next token,
//! populate this cache on the first peek, and then return an immutable refernce to this cache
//! on subsequent peeks.
//!
//! This idea is implemetented with a pair of methods on the stream trait:
//! `prepare_imut_peek` and `imut_peek`. The first method takes a mutable reference to the
//! stream and performs all necessary mutable processing to generate the next token in the
//! stream. The second method then returns an immutable reference to this generated token:
//! ```
//!
//! ```
//! The previous code sample can now be made to work:
//! ```
//!
//! ```
//!
//! ## Restrictions on immutable peeking
//!
//! In general, it is an error to invoke `imut_peek` without first invoking `prepare_imut_peek`.
//! Stream implementations which require mutation before peeking (i.e., have a non-empty
//! implementation of `prepare_imut_peek`) should always return an error from imut_peek` if the
//! prepare function has not been called first.
//!
//! It is also an error to mutate state between `prepare_imut_peek` and `imut_peek`. The borrow
//! checker will sometimes detect this situation. Stream implementations are not expected to
//! error in this case because detecting state changes is expensive.
//!
//! For some stream implementations, like `VecStream`, it is admissible to skip
//! `prepare_imut_state`. This exception is on an per-implementation basis.

use crate::tex::token::token;

use std::convert::TryFrom;

/// A `Stream` is a source of tokens that are possibly generated on demand.
///
/// See the module documentation for details.
pub trait Stream {
    /// Retrieves the next token in the stream.
    fn next(&mut self) -> anyhow::Result<Option<token::Token>>;

    /// Peeks at the next token in the stream.
    ///
    /// To peek using an immutable borrow of the stream, use the methods `prepare_imut_peek`
    /// and `imut_peek`.
    fn peek(&mut self) -> anyhow::Result<Option<&token::Token>> {
        self.prepare_imut_peek()?;
        self.imut_peek()
    }

    /// Performs any mutations needed so as to be able to peek using an immutable borrow
    /// of the stream. See the module documentation for information on why this method
    /// exists.
    fn prepare_imut_peek(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    /// Immutably peeks at the next token in the stream. The method `prepare_imut_peek`
    /// *must* be invoked before this function, otherwise an error will be returned.
    ///
    /// The name of this method is intentionally janky so that consumers think
    /// twice before using it.
    fn imut_peek(&self) -> anyhow::Result<Option<&token::Token>>;

    /// Consumes the next token in the stream without returning it.
    ///
    /// This method is mostly to make code self-documenting. It is typically used in
    /// situations where a peek has already occurred, and the token itself is not needed.
    fn consume(&mut self) -> anyhow::Result<()> {
        self.next().map(|_| ())
    }
}

/// An `EmptyStream` is a stream consisting of no elements.
///
/// The `EmptyStream` type can be helpful when implementing
/// expansion primitives that may return no expanded tokens, for example
/// conditional primitives.
///
/// ```
/// # use texide::tex::token::stream::Stream;
/// # use texide::tex::token::stream::EmptyStream;
/// let mut s = EmptyStream;
/// assert_eq!(s.peek().unwrap(), None);
/// ```
pub struct EmptyStream;

impl Stream for EmptyStream {
    fn next(&mut self) -> anyhow::Result<Option<token::Token>> {
        Ok(None)
    }

    fn imut_peek(&self) -> anyhow::Result<Option<&token::Token>> {
        Ok(None)
    }
}

/// A `SingletonStream` is a stream consisting of exactly one element.
///
/// It is preferable to use
/// this type for the (not uncommon) case when a single token is returned from an expansion
/// primitive.
///
/// A `SingletonStream` may be peeked at immutably without invoking `prepare_imut_peek` first.
///
/// ```
/// # use texide::tex::token::stream::Stream;
/// # use texide::tex::token::stream::SingletonStream;
/// # use texide::tex::token::token::{Token, Value};
/// let t = Token::new_letter('a');
/// let mut s = SingletonStream::new(t.clone());
/// assert_eq!(s.imut_peek().unwrap(), Some(&t));
/// assert_eq!(s.next().unwrap(), Some(t));
/// assert_eq!(s.imut_peek().unwrap(), None);
/// assert_eq!(s.next().unwrap(), None);
/// ```
pub struct SingletonStream {
    t: Option<token::Token>,
}

impl SingletonStream {
    pub fn new(t: token::Token) -> SingletonStream {
        SingletonStream { t: Some(t) }
    }
}

impl Stream for SingletonStream {
    fn next(&mut self) -> anyhow::Result<Option<token::Token>> {
        Ok(self.t.take())
    }

    fn imut_peek(&self) -> anyhow::Result<Option<&token::Token>> {
        Ok(self.t.as_ref())
    }
}

/// A `VecStream` is a stream consisting of a vector of tokens that are returned in order.
///
/// A `VecStream` may be peeked at immutably without invoking `prepare_imut_peek` first.
pub struct VecStream {
    vec: Vec<token::Token>,
}

impl VecStream {
    /// Returns a new `VecStream` consisting of the tokens in the provided vector.
    pub fn new(mut vec: Vec<token::Token>) -> VecStream {
        vec.reverse();
        VecStream { vec }
    }
}

// TODO: destroy
/// This `TryFrom` trait implementation enables easy casting of any `Stream` to a `VecStream`.
impl TryFrom<Box<dyn Stream>> for VecStream {
    type Error = anyhow::Error;

    fn try_from(mut value: Box<dyn Stream>) -> Result<Self, Self::Error> {
        let mut tokens = Vec::new();
        while let Some(token) = value.next()? {
            tokens.push(token);
        }
        Ok(VecStream::new(tokens))
    }
}

impl Stream for VecStream {
    fn next(&mut self) -> anyhow::Result<Option<token::Token>> {
        Ok(self.vec.pop())
    }

    fn imut_peek(&self) -> anyhow::Result<Option<&token::Token>> {
        Ok(self.vec.last())
    }
}
