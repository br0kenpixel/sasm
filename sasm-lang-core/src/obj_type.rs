use crate::{object::SasmObjectType, typetrait::SasmType};
use std::{
    any::Any,
    cmp::Ordering,
    fmt::Debug,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

type TypelessValue = Box<dyn Any>;
type ToStringMethod<T> = fn(&T) -> String;
type ComparisonMethod<T> = fn(&T, &T) -> bool;
type MathOpMethod<T> = fn(&T, &T) -> T;
type MathOpAssignMethod<T> = fn(&mut T, &T);
type CopyMethod<T> = fn(&T) -> T;

/// Represents any SASM object (even `NULL`).
///
/// Objects may optionally implement certain methods for performing basic operations.
//#[allow(clippy::type_complexity)]
pub struct SasmObject {
    /// Type information.
    kind: SasmObjectType,
    /// The actual value.
    value: TypelessValue,

    /*** Stringification ***/
    /// `ToString` implementation for this object.
    to_string: Option<ToStringMethod<Self>>,
    /// Method for creating a string representation of this object.
    repr: Box<dyn Fn(&Self) -> String>,

    /*** Comparisons ***/
    /// Method for comparing equality of this and other object.
    cmp_equality: Option<ComparisonMethod<Self>>,
    /// Method for comparing `>` of this and other object.
    cmp_greater: Option<ComparisonMethod<Self>>,
    /// Method for comparing `>=` of this and other object.
    cmp_greater_eq: Option<ComparisonMethod<Self>>,
    /// Method for comparing `<` of this and other object.
    cmp_less: Option<ComparisonMethod<Self>>,
    /// Method for comparing `<=` of this and other object.
    cmp_less_eq: Option<ComparisonMethod<Self>>,

    /*** Math ops ***/
    /// `+` implementation
    m_add: Option<MathOpMethod<Self>>,
    /// `+=` implementation
    m_iadd: Option<MathOpAssignMethod<Self>>,
    /// `-` implementation
    m_sub: Option<MathOpMethod<Self>>,
    /// `-=` implementation
    m_isub: Option<MathOpAssignMethod<Self>>,
    /// `*` implementation
    m_mul: Option<MathOpMethod<Self>>,
    /// `*=` implementation
    m_imul: Option<MathOpAssignMethod<Self>>,
    /// `/` implementation
    m_div: Option<MathOpMethod<Self>>,
    /// `/=` implementation
    m_idiv: Option<MathOpAssignMethod<Self>>,

    /*** Bitwise ops ***/

    /*** Memory ops ***/
    /// Method for creating a copy of this object.
    copy: Option<CopyMethod<Self>>,
}

impl SasmObject {
    pub fn create_null() -> Self {
        Self::from(())
    }

    pub const fn kind(&self) -> SasmObjectType {
        self.kind
    }

    pub fn to_string(&self) -> Option<String> {
        self.to_string.as_ref().map(|func| func(self))
    }

    pub fn repr(&self) -> String {
        (self.repr)(self)
    }

    pub fn cmp_equals(&self, rhs: &Self) -> Option<bool> {
        self.cmp_equality.as_ref().map(|func| func(self, rhs))
    }

    pub fn cmp_greater(&self, rhs: &Self) -> Option<bool> {
        self.cmp_greater.as_ref().map(|func| func(self, rhs))
    }

    pub fn cmp_greater_eq(&self, rhs: &Self) -> Option<bool> {
        self.cmp_greater_eq.as_ref().map(|func| func(self, rhs))
    }

    pub fn cmp_less(&self, rhs: &Self) -> Option<bool> {
        self.cmp_less.as_ref().map(|func| func(self, rhs))
    }

    pub fn cmp_less_eq(&self, rhs: &Self) -> Option<bool> {
        self.cmp_less_eq.as_ref().map(|func| func(self, rhs))
    }

    pub fn deepcopy(&self) -> Option<Self> {
        self.copy.as_ref().map(|func| func(self))
    }

    pub fn expect<T: 'static + SasmType>(&self) -> Option<&T> {
        self.value.downcast_ref()
    }

    pub fn expect_mut<T: 'static + SasmType>(&mut self) -> Option<&mut T> {
        self.value.downcast_mut()
    }

    fn expect_unchecked<T: 'static>(&self) -> &T {
        unsafe { self.value.downcast_ref().unwrap_unchecked() }
    }

    fn expect_unchecked_mut<T: 'static>(&mut self) -> &mut T {
        unsafe { self.value.downcast_mut().unwrap_unchecked() }
    }
}

impl From<()> for SasmObject {
    fn from(_value: ()) -> Self {
        Self {
            kind: SasmObjectType::NULL,
            value: Box::new(()),
            to_string: None,
            repr: Box::new(|_| "NULL".to_string()),
            cmp_equality: Some(|this, other| this.kind == other.kind),
            cmp_greater: None,
            cmp_greater_eq: None,
            cmp_less: None,
            cmp_less_eq: None,
            m_add: None,
            m_iadd: None,
            m_sub: None,
            m_isub: None,
            m_div: None,
            m_idiv: None,
            m_mul: None,
            m_imul: None,
            copy: Some(|_| Self::create_null()),
        }
    }
}

impl From<&str> for SasmObject {
    fn from(value: &str) -> Self {
        Self {
            kind: SasmObjectType::NUMBER,
            value: Box::new(value.to_string()),
            //to_string: Some(Box::new(|obj| obj.expect_unchecked::<String>().to_string())),
            to_string: Some(|obj| obj.expect_unchecked::<String>().to_string()),
            repr: Box::new(|obj| format!("String<{}>", obj.expect_unchecked::<String>())),
            cmp_equality: Some(|this, other| {
                i64::cmp(this.expect_unchecked(), other.expect_unchecked()) == Ordering::Equal
            }),
            cmp_greater: None,
            cmp_greater_eq: None,
            cmp_less: None,
            cmp_less_eq: None,
            m_add: None,
            m_iadd: None,
            m_sub: None,
            m_isub: None,
            m_div: None,
            m_idiv: None,
            m_mul: None,
            m_imul: None,
            copy: Some(|this| this.expect_unchecked::<String>().as_str().into()),
        }
    }
}

impl From<f32> for SasmObject {
    fn from(value: f32) -> Self {
        Self {
            kind: SasmObjectType::NUMBER,
            value: Box::new(value),
            to_string: Some(|obj| obj.expect_unchecked::<f32>().to_string()),
            repr: Box::new(|obj| format!("Float<{}>", obj.expect_unchecked::<f32>())),
            cmp_equality: Some(|this, other| {
                i64::cmp(this.expect_unchecked(), other.expect_unchecked()) == Ordering::Equal
            }),
            cmp_greater: Some(|this, other| {
                i64::cmp(this.expect_unchecked(), other.expect_unchecked()) == Ordering::Greater
            }),
            cmp_greater_eq: Some(|this, other| {
                i64::cmp(this.expect_unchecked(), other.expect_unchecked()).is_ge()
            }),
            cmp_less: Some(|this, other| {
                i64::cmp(this.expect_unchecked(), other.expect_unchecked()) == Ordering::Less
            }),
            cmp_less_eq: Some(|this, other| {
                i64::cmp(this.expect_unchecked(), other.expect_unchecked()).is_le()
            }),
            m_add: Some(|this, other| {
                Self::from(this.expect_unchecked::<f32>() + other.expect_unchecked::<f32>())
            }),
            m_iadd: Some(|this, other| {
                this.expect_unchecked_mut::<f32>()
                    .add_assign(other.expect_unchecked())
            }),
            m_sub: Some(|this, other| {
                Self::from(this.expect_unchecked::<f32>() - other.expect_unchecked::<f32>())
            }),
            m_isub: Some(|this, other| {
                this.expect_unchecked_mut::<f32>()
                    .sub_assign(other.expect_unchecked())
            }),
            m_div: Some(|this, other| {
                Self::from(this.expect_unchecked::<f32>() / other.expect_unchecked::<f32>())
            }),
            m_idiv: Some(|this, other| {
                this.expect_unchecked_mut::<f32>()
                    .div_assign(other.expect_unchecked())
            }),
            m_mul: Some(|this, other| {
                Self::from(this.expect_unchecked::<f32>() * other.expect_unchecked::<f32>())
            }),
            m_imul: Some(|this, other| {
                this.expect_unchecked_mut::<f32>()
                    .mul_assign(other.expect_unchecked())
            }),
            copy: Some(|this| (*this.expect_unchecked::<f32>()).into()),
        }
    }
}

impl From<i64> for SasmObject {
    fn from(value: i64) -> Self {
        Self {
            kind: SasmObjectType::NUMBER,
            value: Box::new(value),
            to_string: Some(|obj| obj.expect_unchecked::<i64>().to_string()),
            repr: Box::new(|obj| format!("Number<{}>", obj.expect_unchecked::<i64>())),
            cmp_equality: Some(|this, other| {
                i64::cmp(this.expect_unchecked(), other.expect_unchecked()) == Ordering::Equal
            }),
            cmp_greater: Some(|this, other| {
                i64::cmp(this.expect_unchecked(), other.expect_unchecked()) == Ordering::Greater
            }),
            cmp_greater_eq: Some(|this, other| {
                i64::cmp(this.expect_unchecked(), other.expect_unchecked()).is_ge()
            }),
            cmp_less: Some(|this, other| {
                i64::cmp(this.expect_unchecked(), other.expect_unchecked()) == Ordering::Less
            }),
            cmp_less_eq: Some(|this, other| {
                i64::cmp(this.expect_unchecked(), other.expect_unchecked()).is_le()
            }),
            m_add: Some(|this, other| {
                Self::from(this.expect_unchecked::<i64>() + other.expect_unchecked::<i64>())
            }),
            m_iadd: Some(|this, other| {
                this.expect_unchecked_mut::<i64>()
                    .add_assign(other.expect_unchecked())
            }),
            m_sub: Some(|this, other| {
                Self::from(this.expect_unchecked::<i64>() - other.expect_unchecked::<i64>())
            }),
            m_isub: Some(|this, other| {
                this.expect_unchecked_mut::<i64>()
                    .sub_assign(other.expect_unchecked())
            }),
            m_div: Some(|this, other| {
                Self::from(this.expect_unchecked::<i64>() / other.expect_unchecked::<i64>())
            }),
            m_idiv: Some(|this, other| {
                this.expect_unchecked_mut::<i64>()
                    .div_assign(other.expect_unchecked())
            }),
            m_mul: Some(|this, other| {
                Self::from(this.expect_unchecked::<i64>() * other.expect_unchecked::<i64>())
            }),
            m_imul: Some(|this, other| {
                this.expect_unchecked_mut::<i64>()
                    .mul_assign(other.expect_unchecked())
            }),
            copy: Some(|this| (*this.expect_unchecked::<i64>()).into()),
        }
    }
}

impl Debug for SasmObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "SasmObject<{}, '{}'>", self.kind.name(), self.repr())
    }
}
