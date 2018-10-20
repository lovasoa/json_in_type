use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use super::json_string::JSONString;
use super::json_value::JSONValue;

/// Write a single key-value pair
fn write_object_entry<W, K, V>(w: &mut W, key: &K, value: &V) -> io::Result<()>
    where W: io::Write,
          K: JSONString,
          V: JSONValue {
    key.write_json(w)?;
    w.write_all(b":")?;
    value.write_json(w)
}

/// Write a list of key-value pairs to a writer as a json object
fn write_object<'a, W, K, V, I>(w: &mut W, iter: &mut I) -> io::Result<()>
    where W: io::Write,
          K: JSONString,
          V: JSONValue,
          V: 'a,
          K: 'a,
          I: Iterator<Item=(&'a K, &'a V)> {
    w.write_all(b"{")?;
    if let Some((key, value)) = iter.next() {
        write_object_entry(w, key, value)?;
        for (key, value) in iter {
            w.write_all(b",")?;
            write_object_entry(w, key, value)?;
        }
    }
    w.write_all(b"}")
}

/// A struct used to wrap another type and make it serializable as a json object.
/// The other type has to be able to yield (key, value) pairs by implementing IntoIterator.
///
/// # Examples
///
/// Serialize a vec as a json object
///
/// ```
/// use json_in_type::json_object::ToJSONObject;
/// use json_in_type::JSONValue;
///
/// let my_obj = ToJSONObject(vec![("x", 1), ("y", 2)]);
///
/// assert_eq!("{\"x\":1,\"y\":2}", my_obj.to_json_string());
/// ```
pub struct ToJSONObject<K, V, I>(pub I)
    where K: JSONString,
          V: JSONValue,
          for<'a> &'a I: IntoIterator<Item=&'a (K, V)>;

impl<K, V, I> JSONValue for ToJSONObject<K, V, I>
    where K: JSONString,
          V: JSONValue,
          for<'a> &'a I: IntoIterator<Item=&'a (K, V)>
{
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        let mut iter = (&self.0)
            .into_iter()
            .map(|(k, v)| (k, v)); // Convert a borrowed tuple to a tuple of borrowed values
        write_object(w, &mut iter)
    }
}


/// Serialize a HashMap to a JSON object. The property order is not guaranteed.
impl<K: JSONString + Eq + Hash, V: JSONValue> JSONValue for HashMap<K, V> {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        write_object(w, &mut self.iter())
    }
}

pub trait JSONObject: JSONValue {
    fn write_json_ending<W: io::Write>(&self, f: &mut W, first: bool) -> io::Result<()>;
    #[inline]
    fn write_json_full<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        self.write_json_ending(w, true)
    }
}


/// A JSON object stored as a static linked list.
/// This is a generic structure that specializes at compile-time
/// to a structure whose type stores the exact shape of the object.
pub struct JSONObjectEntry<K: JSONString, V: JSONValue, U: JSONObject> {
    pub key: K,
    pub value: V,
    pub next: U,
}

impl<K: JSONString, V: JSONValue, U: JSONObject> JSONObject for JSONObjectEntry<K, V, U> {
    #[inline(always)]
    fn write_json_ending<W: io::Write>(&self, w: &mut W, first: bool) -> io::Result<()> {
        w.write_all(if first { b"{" } else { b"," })?;
        self.key.write_json(w)?;
        w.write_all(b":")?;
        self.value.write_json(w)?;
        self.next.write_json_ending(w, false)
    }
}

impl<K: JSONString, V: JSONValue, U: JSONObject> JSONValue for JSONObjectEntry<K, V, U> {
    #[inline(always)]
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        self.write_json_full(w)
    }
}

/// An empty JSON object. This is a Zero Sized Type.
/// It just serves to mark the end of an object in its type,
/// but takes no space in memory at runtime.
pub struct JSONObjectEnd;

impl JSONObject for JSONObjectEnd {
    #[inline(always)]
    fn write_json_ending<W: io::Write>(&self, w: &mut W, first: bool) -> io::Result<()> {
        w.write_all(if first { b"{}" } else { b"}" })
    }
}

impl JSONValue for JSONObjectEnd {
    fn write_json<W: io::Write>(&self, w: &mut W) -> io::Result<()> {
        self.write_json_full(w)
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! inlined_json_object {
    (key : $key:ident, value : $value:expr, next : $next:expr) => {{
        use $crate::json_object::JSONObject;
        use $crate::JSONValue;

        struct InlinedJSONObjectEntry<V: JSONValue, U: JSONObject> {
            value:V,
            next: U
        }

        impl<V: JSONValue, U: JSONObject> JSONObject
            for InlinedJSONObjectEntry<V,U> {
            #[inline(always)]
            fn write_json_ending<W: ::std::io::Write>(&self, w: &mut W, first: bool)
                -> ::std::io::Result<()> {
                w.write_all(
                    if first {
                        concat!("{\"", stringify!($key),"\":")
                    } else {
                        concat!(",\"", stringify!($key),"\":")
                    }.as_bytes()
                )?;
                self.value.write_json(w)?;
                self.next.write_json_ending(w, false)
            }
        }

        impl<V: JSONValue, U: JSONObject> JSONValue
            for InlinedJSONObjectEntry<V,U> {
            fn write_json<W: ::std::io::Write>(&self, w: &mut W) -> ::std::io::Result<()> {
                self.write_json_full(w)
            }
        }

        InlinedJSONObjectEntry{value:$value, next:$next}
    }};
}

/// Creates a static json object that can be serialized very fast.
/// Returns an object implementing JSONValue
/// 
/// The macro takes a comma-separated list of key-value pairs.
/// Keys can be written litterally, or surrounded by brackets (`[key]`)
/// to reference external variables.
/// Values are expression of a type implementing JSONValue.
///
/// # Examples
///
/// ### Create a simple json object.
/// ```
/// use json_in_type::*;
///
/// let my_obj = json_object!{
///     hello: "world"
/// };
///
/// assert_eq!(r#"{"hello":"world"}"#, my_obj.to_json_string());
/// ```
///
/// ### Reference external variables
/// ```
/// use json_in_type::*;
///
/// let x = "hello";
/// let my_obj = json_object!{
///     [x]: "world", // The trailing comma is ok
/// };
///
/// assert_eq!(r#"{"hello":"world"}"#, my_obj.to_json_string());
/// ```
///
/// ### Compute keys dynamically
/// ```
/// use json_in_type::*;
///
/// let x = "hello";
/// let my_obj = json_object!{
///     [x]: "world",
///     [[x, "_suffix"].concat()]: 42
/// };
///
/// assert_eq!(r#"{"hello":"world","hello_suffix":42}"#, my_obj.to_json_string());
/// ```
///
/// ### Shorthand property names
/// It is common to create a json object from a set of variables,
/// using the variable names as keys and the contents of the variables as values.
/// ```
/// use json_in_type::*;
///
/// let x = "hello";
/// let y = 42;
/// let z = true;
/// let my_obj = json_object!{ x, y, z };
///
/// assert_eq!(r#"{"x":"hello","y":42,"z":true}"#, my_obj.to_json_string());
/// ```
#[macro_export]
macro_rules! json_object {
    () => { $crate::json_object::JSONObjectEnd{} };
    // A key that references a variable of the same name
    ($key:ident, $($rest:tt)*) => {
        inlined_json_object!{
            key: $key,
            value: $key,
            next: json_object!($($rest)*)
         }
    };
    // Literal key
    ($key:ident : $value:expr, $($rest:tt)*) => {
        inlined_json_object!{
            key: $key,
            value: $value,
            next: json_object!($($rest)*)
         }
    };
    // The key is an expression in square brackets
    ([$key:expr] : $value:expr, $($rest:tt)*) => {
        $crate::json_object::JSONObjectEntry {
            key: $key,
            value: $value,
            next: json_object!($($rest)*)
        }
    };
    // Simply adding a trailing colon
    ($key:ident : $value:expr) => { json_object!($key:$value,) };
    ([$key:expr] : $value:expr) => { json_object!([$key]:$value,) };
    ($key:ident) => { json_object!($key,) };
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!("{}", JSONObjectEnd.to_json_string());
        assert_eq!("{}", json_object!().to_json_string());
    }

    #[test]
    fn test_single_pair() {
        assert_eq!(r#"{"x":{}}"#, json_object!(x : json_object!()).to_json_string());
        // With a trailing comma:
        assert_eq!(r#"{"x":{}}"#, json_object!(x : json_object!(),).to_json_string());
    }

    #[test]
    fn test_two_pairs() {
        assert_eq!(r#"{"x":{},"y":{}}"#, json_object! {
            x : json_object!(),
            y : json_object!()
        }.to_json_string());
    }

    #[test]
    fn test_nested() {
        assert_eq!(r#"{"x":{"y":{}}}"#, json_object! {
            x : json_object! {
                y : json_object!()
            }
        }.to_json_string());
    }

    #[test]
    fn test_dynamic_keys() {
        let x = "x";
        let y = String::from("y");
        assert_eq!(r#"{"x":{"y":{}}}"#, json_object! {
            [x] : json_object! {
                [y] : json_object!()
            }
        }.to_json_string());
    }

    #[test]
    fn test_hashmap() {
        let mut map = HashMap::new();
        map.insert("x", 1);
        map.insert("y", 2);
        // The order in which the keys are serialized is not guaranteed
        let expected = vec![
            r#"{"x":1,"y":2}"#,
            r#"{"y":2,"x":1}"#
        ];
        assert!(expected.contains(&&map.to_json_string()[..]));
    }
}
