use crate::bindings::{
    gravity_class_bool, gravity_class_class, gravity_class_closure, gravity_class_fiber,
    gravity_class_float, gravity_class_function, gravity_class_instance, gravity_class_int,
    gravity_class_list, gravity_class_map, gravity_class_null, gravity_class_range,
    gravity_class_string, gravity_value_equals, gravity_value_from_bool, gravity_value_from_float,
    gravity_value_from_int, gravity_value_from_null, gravity_value_from_undefined, gravity_value_t,
};

pub type Value = gravity_value_t;

impl Value {
    #[inline]
    pub fn null() -> Self {
        unsafe { gravity_value_from_null() }
    }

    #[inline]
    pub fn undefined() -> Self {
        unsafe { gravity_value_from_undefined() }
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

    #[inline]
    pub fn is_null_class(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_null) }
    }

    #[inline]
    pub fn is_null(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_null) && self.__bindgen_anon_1.n == 0 }
    }

    #[inline]
    pub fn is_undefined(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_null) && self.__bindgen_anon_1.n == 1 }
    }

    #[inline]
    pub fn is_boolean(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_bool) }
    }

    #[inline]
    pub fn is_integer(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_int) }
    }

    #[inline]
    pub fn is_float(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_float) }
    }

    #[inline]
    pub fn is_function(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_function) }
    }

    #[inline]
    pub fn is_instance(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_instance) }
    }

    #[inline]
    pub fn is_closure(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_closure) }
    }

    #[inline]
    pub fn is_class(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_class) }
    }

    #[inline]
    pub fn is_fiber(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_fiber) }
    }

    #[inline]
    pub fn is_string(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_string) }
    }

    #[inline]
    pub fn is_list(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_list) }
    }

    #[inline]
    pub fn is_map(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_map) }
    }

    #[inline]
    pub fn is_range(&self) -> bool {
        unsafe { std::ptr::eq(self.isa, gravity_class_range) }
    }

    #[inline]
    pub fn is_basic_type(&self) -> bool {
        self.is_string() || self.is_integer() || self.is_float() || self.is_boolean()
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        !std::ptr::eq(self.isa, std::ptr::null())
    }

    #[inline]
    pub fn is_invalid(&self) -> bool {
        std::ptr::eq(self.isa, std::ptr::null())
    }
}

impl From<bool> for Value {
    #[inline]
    fn from(value: bool) -> Self {
        Self::boolean(value)
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

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    fn value_is_null() {
        let x = Value::null();

        assert!(x.is_null())
    }

    #[test]
    fn value_is_undefined() {
        let x = Value::undefined();

        assert!(x.is_undefined())
    }

    #[test]
    fn value_is_integer() {
        let x = Value::from(10);

        assert!(x.is_integer())
    }

    #[test]
    fn value_is_float() {
        let x = Value::from(10.0);

        assert!(x.is_float())
    }

    #[test]
    fn value_is_boolean() {
        let x = Value::from(false);

        assert!(x.is_boolean())
    }
}
