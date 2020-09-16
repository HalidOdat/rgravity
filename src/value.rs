use crate::bindings::{
    gravity_value_equals, gravity_value_from_bool, gravity_value_from_float,
    gravity_value_from_int, gravity_value_from_null, gravity_value_from_undefined, gravity_value_t,
};

pub type Value = gravity_value_t;

impl Value {
    #[inline]
    pub fn null() -> Self {
        unsafe { gravity_value_from_undefined() }
    }

    #[inline]
    pub fn undefined() -> Self {
        unsafe { gravity_value_from_null() }
    }

    #[inline]
    pub fn boolean<T>(value: T) -> Self
    where
        T: Into<bool>,
    {
        unsafe { gravity_value_from_bool(value.into()) }
    }

    #[inline]
    pub fn integer<T>(value: T) -> Self
    where
        T: Into<i64>,
    {
        unsafe { gravity_value_from_int(value.into()) }
    }

    #[inline]
    pub fn float<T>(value: T) -> Self
    where
        T: Into<f64>,
    {
        unsafe { gravity_value_from_float(value.into()) }
    }
}

impl From<u8> for Value {
    #[inline]
    fn from(value: u8) -> Self {
        Self::integer(value)
    }
}

impl From<u16> for Value {
    #[inline]
    fn from(value: u16) -> Self {
        Self::integer(value)
    }
}

impl From<u32> for Value {
    #[inline]
    fn from(value: u32) -> Self {
        Self::integer(value)
    }
}

impl From<i8> for Value {
    #[inline]
    fn from(value: i8) -> Self {
        Self::integer(value)
    }
}

impl From<i16> for Value {
    #[inline]
    fn from(value: i16) -> Self {
        Self::integer(value)
    }
}

impl From<i32> for Value {
    #[inline]
    fn from(value: i32) -> Self {
        Self::integer(value)
    }
}

impl From<i64> for Value {
    #[inline]
    fn from(value: i64) -> Self {
        Self::integer(value)
    }
}

impl From<f64> for Value {
    #[inline]
    fn from(value: f64) -> Self {
        Self::float(value)
    }
}

impl From<f32> for Value {
    #[inline]
    fn from(value: f32) -> Self {
        Self::float(value)
    }
}

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Value) -> bool {
        unsafe { gravity_value_equals(*self, *other) }
    }
}
