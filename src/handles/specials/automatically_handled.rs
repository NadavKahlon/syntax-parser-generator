use crate::handles::{Handle, HandleCore, Handled};

/// A data type for which handles can be automatically generated to identify its instances.
///
/// To do so, we need to implement the [AutomaticallyHandled::serial] method, that generates a
/// unique serial-number with each instance of the type. This serial will be used to construct the
/// identifying handle of the instance.
///
/// This usually serves for generating handles to instances of very simple data types, most commonly
/// `enum`s. Such data types can be so easily enumerated, that there's no need to manage an
/// auxiliary collection (e.g. [HandledVec](crate::handles::collections::HandledVec),
/// [HandledHashMap](crate::handles::collections::HandledHashMap)) to associate enumerated handles
/// with their known instances.
///
/// # Example
/// ```rust
/// # use syntax_parser_generator::handles::specials::AutomaticallyHandled;
/// # #[derive(Clone, Copy)]
/// enum MyLexemeType { Identifier, IntegerLiteral, If, While }
/// impl AutomaticallyHandled for MyLexemeType {
///     type HandleCoreType = u8;
///     fn serial(&self) -> usize { *self as usize }
/// }
/// let identifier_lexeme_type = MyLexemeType::Identifier.handle();
/// ```
pub trait AutomaticallyHandled: Sized {
    /// The internal representation of this type's handles (see [HandleCore] for more detail).
    type HandleCoreType: HandleCore;

    /// Generate a serial-number that identifies this instance of the type.
    fn serial(&self) -> usize;

    /// Get a handle to this instance of the type.
    fn handle(&self) -> Handle<Self> {
        self.serial().into()
    }
}

impl<T> Handled for T
where
    T: AutomaticallyHandled,
{ type HandleCoreType = T::HandleCoreType; }
