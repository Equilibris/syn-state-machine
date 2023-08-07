mod associated_items;
mod constant_items;
mod enumerations;
mod extern_blocks;
mod extern_crate;
mod functions;
mod generic_parameters;
mod implementations;
mod modules;
mod static_item;
mod structs;
mod traits;
mod type_aliases;
mod union;
mod use_declarations;

pub use associated_items::*;
pub use constant_items::*;
pub use enumerations::*;
pub use extern_blocks::*;
pub use extern_crate::*;
pub use functions::*;
pub use generic_parameters::*;
pub use implementations::*;
pub use modules::*;
pub use static_item::*;
pub use structs::*;
pub use traits::*;
pub use type_aliases::*;
pub use union::*;
pub use use_declarations::*;

use crate::*;

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum Item<Attr, Ty, Expr, Pat> [ attrs <- Rep<OuterAttribute<Attr>>] {
        VisItem(v <- VisItem<Attr, Ty, Expr, Pat, Self>),
        MacroItem(v <- MacroItem)
    }
}
to_tokens! {
    impl ToTokens for enum Item<Attr, Ty, Expr, Pat> [ attrs <- Rep<OuterAttribute<Attr>>] {
        VisItem(v <- VisItem<Attr, Ty, Expr, Pat, Self>),
        MacroItem(v <- MacroItem)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum VisItem<Attr, Ty, Expr, Pat, Item>[ vis <- Option<Visibility> ] {
        Module(v <- Module<Attr, Item>),
        ExternCrate(v <- ExternCrate),
        UseDeclaration(v <- UseDeclaration),
        Function(v <- Function<Attr, Ty, Expr, Pat>),
        TypeAlias(v <- TypeAlias<Attr, Ty>),
        Struct(v <- Struct<Attr, Ty>),
        Enumeration(v <- Enumeration<Attr, Ty, Expr>),
        Union(v <- Union<Attr, Ty>),
        ConstantItem(v <- ConstantItem<Ty, Expr>),
        StaticItem(v <- StaticItem<Ty, Expr>),
        Trait(v <- Trait<Attr, Ty, Expr, Pat>),
        Implementation(v <- Implementation<Attr, Ty, Expr, Pat>),
        ExternBlock(v <- ExternBlock<Attr, Ty, Expr, Pat>),
    }
}
to_tokens! {
    impl ToTokens for enum VisItem<Attr, Ty, Expr, Pat, Item> [ vis <- Option<Visibility> ] {
        Module(v <- Module<Attr, Item>),
        ExternCrate(v <- ExternCrate),
        UseDeclaration(v <- UseDeclaration),
        Function(v <- Function<Attr, Ty, Expr, Pat>),
        TypeAlias(v <- TypeAlias<Attr, Ty>),
        Struct(v <- Struct<Attr, Ty>),
        Enumeration(v <- Enumeration<Attr, Ty, Expr>),
        Union(v <- Union<Attr, Ty>),
        ConstantItem(v <- ConstantItem<Ty, Expr>),
        StaticItem(v <- StaticItem<Ty, Expr>),
        Trait(v <- Trait<Attr, Ty, Expr, Pat>),
        Implementation(v <- Implementation<Attr, Ty, Expr, Pat>),
        ExternBlock(v <- ExternBlock<Attr, Ty, Expr, Pat>),
    }

}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum MacroItem {
        MacroInvocationSemi(v <- MacroInvocationSemi),
        MacroRulesDefinition(v <- MacroRulesDefinition)
    }
}
to_tokens! {
    impl ToTokens for enum MacroItem {
        MacroInvocationSemi(v <- MacroInvocationSemi),
        MacroRulesDefinition(v <- MacroRulesDefinition)
    }
}
