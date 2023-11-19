// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
use glib::{bitflags::bitflags, prelude::*, translate::*};

#[cfg(feature = "v4_14")]
bitflags! {
    #[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    #[doc(alias = "GskPathForeachFlags")]
    pub struct PathForeachFlags: u32 {
        #[doc(alias = "GSK_PATH_FOREACH_ALLOW_ONLY_LINES")]
        const ONLY_LINES = ffi::GSK_PATH_FOREACH_ALLOW_ONLY_LINES as _;
        #[doc(alias = "GSK_PATH_FOREACH_ALLOW_QUAD")]
        const QUAD = ffi::GSK_PATH_FOREACH_ALLOW_QUAD as _;
        #[doc(alias = "GSK_PATH_FOREACH_ALLOW_CUBIC")]
        const CUBIC = ffi::GSK_PATH_FOREACH_ALLOW_CUBIC as _;
        #[doc(alias = "GSK_PATH_FOREACH_ALLOW_CONIC")]
        const CONIC = ffi::GSK_PATH_FOREACH_ALLOW_CONIC as _;
    }
}

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
#[doc(hidden)]
impl IntoGlib for PathForeachFlags {
    type GlibType = ffi::GskPathForeachFlags;

    #[inline]
    fn into_glib(self) -> ffi::GskPathForeachFlags {
        self.bits()
    }
}

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
#[doc(hidden)]
impl FromGlib<ffi::GskPathForeachFlags> for PathForeachFlags {
    #[inline]
    unsafe fn from_glib(value: ffi::GskPathForeachFlags) -> Self {
        skip_assert_initialized!();
        Self::from_bits_truncate(value)
    }
}

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
impl StaticType for PathForeachFlags {
    #[inline]
    #[doc(alias = "gsk_path_foreach_flags_get_type")]
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_path_foreach_flags_get_type()) }
    }
}

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
impl glib::HasParamSpec for PathForeachFlags {
    type ParamSpec = glib::ParamSpecFlags;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> glib::ParamSpecFlagsBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
impl glib::value::ValueType for PathForeachFlags {
    type Type = Self;
}

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
unsafe impl<'a> glib::value::FromValue<'a> for PathForeachFlags {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_flags(value.to_glib_none().0))
    }
}

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
impl ToValue for PathForeachFlags {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(feature = "v4_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "v4_14")))]
impl From<PathForeachFlags> for glib::Value {
    #[inline]
    fn from(v: PathForeachFlags) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}