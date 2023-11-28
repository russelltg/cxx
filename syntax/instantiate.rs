use crate::syntax::{Atom, NamedType, Ty1, Type};
use proc_macro2::{Ident, Span};
use std::{
    fmt::Write,
    hash::{Hash, Hasher},
};
use syn::Token;

use super::Types;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) enum ImplKey<'a> {
    RustBox(NamedImplKey<'a>),
    RustVec(NamedImplKey<'a>),
    UniquePtr(NamedImplKey<'a>),
    SharedPtr(NamedImplKey<'a>),
    WeakPtr(NamedImplKey<'a>),
    CxxVector(NamedImplKey<'a>),
    CxxFunction(FunctionImplKey<'a>),
}

#[derive(Clone)]
pub(crate) struct FunctionImplKey<'a> {
    pub ret: Option<&'a Ident>,
    pub args: Vec<&'a Ident>,

    pub ty: &'a Type,
}

#[derive(Copy, Clone)]
pub(crate) struct NamedImplKey<'a> {
    #[allow(dead_code)] // only used by cxxbridge-macro, not cxx-build
    pub begin_span: Span,
    pub rust: &'a Ident,
    #[allow(dead_code)] // only used by cxxbridge-macro, not cxx-build
    pub lt_token: Option<Token![<]>,
    #[allow(dead_code)] // only used by cxxbridge-macro, not cxx-build
    pub gt_token: Option<Token![>]>,
    #[allow(dead_code)] // only used by cxxbridge-macro, not cxx-build
    pub end_span: Span,
}

impl Type {
    pub(crate) fn impl_key(&self) -> Option<ImplKey> {
        if let Type::RustBox(ty) = self {
            if let Type::Ident(ident) = &ty.inner {
                return Some(ImplKey::RustBox(NamedImplKey::new(ty, ident)));
            }
        } else if let Type::RustVec(ty) = self {
            if let Type::Ident(ident) = &ty.inner {
                return Some(ImplKey::RustVec(NamedImplKey::new(ty, ident)));
            }
        } else if let Type::UniquePtr(ty) = self {
            if let Type::Ident(ident) = &ty.inner {
                return Some(ImplKey::UniquePtr(NamedImplKey::new(ty, ident)));
            }
        } else if let Type::SharedPtr(ty) = self {
            if let Type::Ident(ident) = &ty.inner {
                return Some(ImplKey::SharedPtr(NamedImplKey::new(ty, ident)));
            }
        } else if let Type::WeakPtr(ty) = self {
            if let Type::Ident(ident) = &ty.inner {
                return Some(ImplKey::WeakPtr(NamedImplKey::new(ty, ident)));
            }
        } else if let Type::CxxVector(ty) = self {
            if let Type::Ident(ident) = &ty.inner {
                return Some(ImplKey::CxxVector(NamedImplKey::new(ty, ident)));
            }
        } else if let Type::CxxFunction(ty) = self {
            if let Type::Fn(sig) = &ty.inner {
                let ret = match &sig.ret {
                    Some(Type::Ident(ret_id)) => Some(&ret_id.rust),
                    Some(_) => return None, // non-ident return, weird
                    None => None,
                };

                let args = sig
                    .args
                    .iter()
                    .map(|a| {
                        if let Type::Ident(a) = &a.ty {
                            Some(&a.rust)
                        } else {
                            None
                        }
                    })
                    .flatten()
                    .collect();

                return Some(ImplKey::CxxFunction(FunctionImplKey::new(&self, ret, args)));
            }
        }
        None
    }
}

impl<'a> PartialEq for NamedImplKey<'a> {
    fn eq(&self, other: &Self) -> bool {
        PartialEq::eq(self.rust, other.rust)
    }
}

impl<'a> Eq for NamedImplKey<'a> {}

impl<'a> Hash for NamedImplKey<'a> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.rust.hash(hasher);
    }
}

impl<'a> NamedImplKey<'a> {
    fn new(outer: &Ty1, inner: &'a NamedType) -> Self {
        NamedImplKey {
            begin_span: outer.name.span(),
            rust: &inner.rust,
            lt_token: inner.generics.lt_token,
            gt_token: inner.generics.gt_token,
            end_span: outer.rangle.span,
        }
    }
}

impl<'a> FunctionImplKey<'a> {
    fn new(ty: &'a Type, ret: Option<&'a Ident>, args: Vec<&'a Ident>) -> FunctionImplKey<'a> {
        FunctionImplKey { ret, args, ty }
    }

    pub fn link_name_invoke(&self, types: &Types) -> String {
        let ret_str = if let Some(ret) = self.ret {
            if let Some(atom) = Atom::from(ret) {
                atom.to_string()
            } else {
                types.resolve(ret).name.to_symbol().to_string()
            }
        } else {
            "".to_string()
        };

        let mut prefix = format!("cxxbridge1$std$function${}", ret_str);
        for arg in &self.args {
            if let Some(atom) = Atom::from(arg) {
                write!(&mut prefix, "${}", atom).unwrap();
            } else {
                write!(&mut prefix, "${}", types.resolve(*arg).name.to_symbol()).unwrap();
            }
        }
        prefix.push('$');

        format!("{}invoke", prefix)
    }
}

impl<'a> PartialEq for FunctionImplKey<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.ret == other.ret && self.args == other.args
    }
}

impl<'a> Eq for FunctionImplKey<'a> {}

impl<'a> Hash for FunctionImplKey<'a> {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.ret.hash(hasher);
        self.args.hash(hasher);
    }
}
