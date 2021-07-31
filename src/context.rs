/// A macro to easily create a template rendering context.
///
/// Invocations of this macro expand to a value of an anonymous type which
/// implements [`serde::Serialize`]. Fields can be literal expressions or
/// variables captured from a surrounding scope, as long as all fields implement
/// `Serialize`.
///
/// # Examples
///
/// The following code:
///
/// ```rust
/// # #[macro_use] extern crate rocket;
/// # use rocket_dyn_templates::{Template, context};
/// #[get("/<foo>")]
/// fn render_index(foo: u64) -> Template {
///     Template::render("index", context! {
///         // Note that shorthand field syntax is supports.
///         // This is equivalent to `foo: foo,`
///         foo,
///         bar: "Hello world",
///     })
/// }
/// ```
///
/// is equivalent to the following, but without the need to manually define an
/// `IndexContext` struct:
///
/// ```rust
/// # use rocket_dyn_templates::Template;
/// # use rocket::serde::Serialize;
/// # use rocket::get;
/// #[derive(Serialize)]
/// # #[serde(crate = "rocket::serde")]
/// struct IndexContext<'a> {
///     foo: u64,
///     bar: &'a str,
/// }
///
/// #[get("/<foo>")]
/// fn render_index(foo: u64) -> Template {
///     Template::render("index", IndexContext {
///         foo,
///         bar: "Hello world",
///     })
/// }
/// ```
///
/// ## Nesting
///
/// Nested objects can be created by nesting calls to `context!`:
///
/// ```rust
/// # use rocket_dyn_templates::context;
/// # fn main() {
/// let ctx = context! {
///     planet: "Earth",
///     info: context! {
///         mass: 5.97e24,
///         radius: "6371 km",
///         moons: 1,
///     },
/// };
/// # }
/// ```
#[macro_export]
macro_rules! context {
    ($($key:ident $(: $value:expr)?),*$(,)?) => {{
        use serde::ser::{Serialize, Serializer, SerializeMap};
        use ::std::fmt::{Debug, Formatter};

        #[allow(non_camel_case_types)]
        struct ContextMacroCtxObject<$($key: Serialize),*> {
            $($key: $key),*
        }

        #[allow(non_camel_case_types)]
        impl<$($key: Serialize),*> Serialize for ContextMacroCtxObject<$($key),*> {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer,
            {
                let mut map = serializer.serialize_map(None)?;
                $(map.serialize_entry(stringify!($key), &self.$key)?;)*
                map.end()
            }
        }

        #[allow(non_camel_case_types)]
        impl<$($key: Debug + Serialize),*> Debug for ContextMacroCtxObject<$($key),*> {
            fn fmt(&self, f: &mut Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct("context!")
                    $(.field(stringify!($key), &self.$key))*
                    .finish()
            }
        }

        ContextMacroCtxObject {
            $($key $(: $value)?),*
        }
    }};
}
