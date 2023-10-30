//! Generated file, do not edit by hand, see `xtask/codegen`

use super::TemplateRegistry;
use crate::convert::{FromV8, ToV8};
use biome_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};
pub(super) fn register_interfaces(
    scope: &mut v8::HandleScope<'_, ()>,
    global: v8::Local<'_, v8::ObjectTemplate>,
    registry: &mut TemplateRegistry,
) {
    registry
        .build_enum::<biome_json_syntax::JsonSyntaxKind>(scope, global, "JsonSyntaxKind")
        .variant("EOF", biome_json_syntax::JsonSyntaxKind::EOF)
        .variant("COLON", biome_json_syntax::JsonSyntaxKind::COLON)
        .variant("COMMA", biome_json_syntax::JsonSyntaxKind::COMMA)
        .variant("L_PAREN", biome_json_syntax::JsonSyntaxKind::L_PAREN)
        .variant("R_PAREN", biome_json_syntax::JsonSyntaxKind::R_PAREN)
        .variant("L_CURLY", biome_json_syntax::JsonSyntaxKind::L_CURLY)
        .variant("R_CURLY", biome_json_syntax::JsonSyntaxKind::R_CURLY)
        .variant("L_BRACK", biome_json_syntax::JsonSyntaxKind::L_BRACK)
        .variant("R_BRACK", biome_json_syntax::JsonSyntaxKind::R_BRACK)
        .variant("NULL_KW", biome_json_syntax::JsonSyntaxKind::NULL_KW)
        .variant("TRUE_KW", biome_json_syntax::JsonSyntaxKind::TRUE_KW)
        .variant("FALSE_KW", biome_json_syntax::JsonSyntaxKind::FALSE_KW)
        .variant(
            "JSON_STRING_LITERAL",
            biome_json_syntax::JsonSyntaxKind::JSON_STRING_LITERAL,
        )
        .variant(
            "JSON_NUMBER_LITERAL",
            biome_json_syntax::JsonSyntaxKind::JSON_NUMBER_LITERAL,
        )
        .variant(
            "ERROR_TOKEN",
            biome_json_syntax::JsonSyntaxKind::ERROR_TOKEN,
        )
        .variant("NEWLINE", biome_json_syntax::JsonSyntaxKind::NEWLINE)
        .variant("WHITESPACE", biome_json_syntax::JsonSyntaxKind::WHITESPACE)
        .variant("IDENT", biome_json_syntax::JsonSyntaxKind::IDENT)
        .variant("COMMENT", biome_json_syntax::JsonSyntaxKind::COMMENT)
        .variant(
            "MULTILINE_COMMENT",
            biome_json_syntax::JsonSyntaxKind::MULTILINE_COMMENT,
        )
        .variant("JSON_ROOT", biome_json_syntax::JsonSyntaxKind::JSON_ROOT)
        .variant(
            "JSON_NUMBER_VALUE",
            biome_json_syntax::JsonSyntaxKind::JSON_NUMBER_VALUE,
        )
        .variant(
            "JSON_STRING_VALUE",
            biome_json_syntax::JsonSyntaxKind::JSON_STRING_VALUE,
        )
        .variant(
            "JSON_BOOLEAN_VALUE",
            biome_json_syntax::JsonSyntaxKind::JSON_BOOLEAN_VALUE,
        )
        .variant(
            "JSON_NULL_VALUE",
            biome_json_syntax::JsonSyntaxKind::JSON_NULL_VALUE,
        )
        .variant(
            "JSON_ARRAY_VALUE",
            biome_json_syntax::JsonSyntaxKind::JSON_ARRAY_VALUE,
        )
        .variant(
            "JSON_OBJECT_VALUE",
            biome_json_syntax::JsonSyntaxKind::JSON_OBJECT_VALUE,
        )
        .variant(
            "JSON_MEMBER_LIST",
            biome_json_syntax::JsonSyntaxKind::JSON_MEMBER_LIST,
        )
        .variant(
            "JSON_MEMBER",
            biome_json_syntax::JsonSyntaxKind::JSON_MEMBER,
        )
        .variant(
            "JSON_MEMBER_NAME",
            biome_json_syntax::JsonSyntaxKind::JSON_MEMBER_NAME,
        )
        .variant(
            "JSON_ARRAY_ELEMENT_LIST",
            biome_json_syntax::JsonSyntaxKind::JSON_ARRAY_ELEMENT_LIST,
        )
        .variant("JSON_BOGUS", biome_json_syntax::JsonSyntaxKind::JSON_BOGUS)
        .variant(
            "JSON_BOGUS_VALUE",
            biome_json_syntax::JsonSyntaxKind::JSON_BOGUS_VALUE,
        )
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonArrayValue>(scope, global, "JsonArrayValue")
        .extends::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "l_brack_token", JsonArrayValue_l_brack_token)
        .method(scope, "elements", JsonArrayValue_elements)
        .method(scope, "r_brack_token", JsonArrayValue_r_brack_token)
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonBooleanValue>(scope, global, "JsonBooleanValue")
        .extends::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "value_token", JsonBooleanValue_value_token)
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonMember>(scope, global, "JsonMember")
        .extends::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "name", JsonMember_name)
        .method(scope, "colon_token", JsonMember_colon_token)
        .method(scope, "value", JsonMember_value)
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonMemberName>(scope, global, "JsonMemberName")
        .extends::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "value_token", JsonMemberName_value_token)
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonNullValue>(scope, global, "JsonNullValue")
        .extends::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "value_token", JsonNullValue_value_token)
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonNumberValue>(scope, global, "JsonNumberValue")
        .extends::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "value_token", JsonNumberValue_value_token)
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonObjectValue>(scope, global, "JsonObjectValue")
        .extends::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "l_curly_token", JsonObjectValue_l_curly_token)
        .method(scope, "json_member_list", JsonObjectValue_json_member_list)
        .method(scope, "r_curly_token", JsonObjectValue_r_curly_token)
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonRoot>(scope, global, "JsonRoot")
        .extends::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "value", JsonRoot_value)
        .method(scope, "eof_token", JsonRoot_eof_token)
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonStringValue>(scope, global, "JsonStringValue")
        .extends::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>(scope)
        .method(scope, "value_token", JsonStringValue_value_token)
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonBogus>(scope, global, "JsonBogus")
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonBogusValue>(scope, global, "JsonBogusValue")
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonArrayElementList>(
            scope,
            global,
            "JsonArrayElementList",
        )
        .method(scope, "iter", JsonArrayElementList_iter)
        .finish(scope);
    registry
        .build_class::<biome_json_syntax::JsonMemberList>(scope, global, "JsonMemberList")
        .method(scope, "iter", JsonMemberList_iter)
        .finish(scope);
    registry
        .build_interface::<biome_rowan::AstSeparatedListNodesIterator<
            biome_json_syntax::JsonLanguage,
            biome_json_syntax::AnyJsonValue,
        >>(scope)
        .iterable(scope, AstSeparatedListNodesIterator_next)
        .finish(scope);
    registry
        .build_interface::<biome_rowan::AstSeparatedListNodesIterator<
            biome_json_syntax::JsonLanguage,
            biome_json_syntax::JsonMember,
        >>(scope)
        .iterable(scope, AstSeparatedListNodesIterator_next)
        .finish(scope);
}
#[allow(non_snake_case)]
fn AstSeparatedListNodesIterator_next<'s, T: ToV8<'s>>(
    item: biome_rowan::SyntaxResult<T>,
    scope: &mut v8::HandleScope<'s>,
) -> anyhow::Result<v8::Local<'s, v8::Value>> {
    ToV8::to_v8(item?, scope)
}
impl<'s> ToV8<'s> for biome_json_syntax::JsonArrayValue {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_json_syntax::JsonArrayValue,
            biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonArrayValue_l_brack_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonArrayValue::cast_ref(&*this).unwrap();
    let result = this.l_brack_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn JsonArrayValue_elements<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonArrayValue::cast_ref(&*this).unwrap();
    let result = this.elements();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
#[allow(non_snake_case)]
fn JsonArrayValue_r_brack_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonArrayValue::cast_ref(&*this).unwrap();
    let result = this.r_brack_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_json_syntax::JsonBooleanValue {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_json_syntax::JsonBooleanValue,
            biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonBooleanValue_value_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonBooleanValue::cast_ref(&*this).unwrap();
    let result = this.value_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_json_syntax::JsonMember {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_json_syntax::JsonMember,
            biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonMember_name<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonMember::cast_ref(&*this).unwrap();
    let result = this.name();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn JsonMember_colon_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonMember::cast_ref(&*this).unwrap();
    let result = this.colon_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn JsonMember_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonMember::cast_ref(&*this).unwrap();
    let result = this.value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_json_syntax::JsonMemberName {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_json_syntax::JsonMemberName,
            biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonMemberName_value_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonMemberName::cast_ref(&*this).unwrap();
    let result = this.value_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_json_syntax::JsonNullValue {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_json_syntax::JsonNullValue,
            biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonNullValue_value_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonNullValue::cast_ref(&*this).unwrap();
    let result = this.value_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_json_syntax::JsonNumberValue {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_json_syntax::JsonNumberValue,
            biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonNumberValue_value_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonNumberValue::cast_ref(&*this).unwrap();
    let result = this.value_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_json_syntax::JsonObjectValue {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_json_syntax::JsonObjectValue,
            biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonObjectValue_l_curly_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonObjectValue::cast_ref(&*this).unwrap();
    let result = this.l_curly_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn JsonObjectValue_json_member_list<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonObjectValue::cast_ref(&*this).unwrap();
    let result = this.json_member_list();
    let result = ToV8::to_v8(result, scope).unwrap();
    res.set(result);
}
#[allow(non_snake_case)]
fn JsonObjectValue_r_curly_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonObjectValue::cast_ref(&*this).unwrap();
    let result = this.r_curly_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_json_syntax::JsonRoot {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_json_syntax::JsonRoot,
            biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonRoot_value<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonRoot::cast_ref(&*this).unwrap();
    let result = this.value();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
#[allow(non_snake_case)]
fn JsonRoot_eof_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonRoot::cast_ref(&*this).unwrap();
    let result = this.eof_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_json_syntax::JsonStringValue {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        let node = self.into_syntax();
        crate::registry::instantiate_as::<
            biome_json_syntax::JsonStringValue,
            biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>,
        >(scope, node)
        .map(Into::into)
    }
}
#[allow(non_snake_case)]
fn JsonStringValue_value_token<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_rowan::SyntaxNode<biome_json_syntax::JsonLanguage>>::from_v8(
        scope, this,
    )
    .unwrap();
    let this = biome_json_syntax::JsonStringValue::cast_ref(&*this).unwrap();
    let result = this.value_token();
    match result {
        Ok(result) => {
            let result = ToV8::to_v8(result, scope).unwrap();
            res.set(result);
        }
        Err(err) => {
            let message = err.to_string();
            let message = v8::String::new(scope, &message).unwrap();
            let exception = v8::Exception::error(scope, message);
            scope.throw_exception(exception);
        }
    }
}
impl<'s> ToV8<'s> for biome_json_syntax::AnyJsonValue {
    fn to_v8(self, scope: &mut v8::HandleScope<'s>) -> anyhow::Result<v8::Local<'s, v8::Value>> {
        match self {
            Self::JsonArrayValue(node) => ToV8::to_v8(node, scope),
            Self::JsonBogusValue(node) => ToV8::to_v8(node, scope),
            Self::JsonBooleanValue(node) => ToV8::to_v8(node, scope),
            Self::JsonNullValue(node) => ToV8::to_v8(node, scope),
            Self::JsonNumberValue(node) => ToV8::to_v8(node, scope),
            Self::JsonObjectValue(node) => ToV8::to_v8(node, scope),
            Self::JsonStringValue(node) => ToV8::to_v8(node, scope),
        }
    }
}
crate::convert::impl_convert_native!(biome_json_syntax::JsonBogus);
crate::convert::impl_convert_native!(biome_json_syntax::JsonBogusValue);
crate::convert::impl_convert_native!(biome_json_syntax::JsonArrayElementList);
#[allow(non_snake_case)]
fn JsonArrayElementList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this =
        std::cell::Ref::<biome_json_syntax::JsonArrayElementList>::from_v8(scope, this).unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate::convert::impl_convert_native!(biome_json_syntax::JsonMemberList);
#[allow(non_snake_case)]
fn JsonMemberList_iter<'s>(
    scope: &mut v8::HandleScope<'s>,
    args: v8::FunctionCallbackArguments<'s>,
    mut res: v8::ReturnValue,
) {
    let this = args.this().into();
    let this = std::cell::Ref::<biome_json_syntax::JsonMemberList>::from_v8(scope, this).unwrap();
    let iter = this.iter();
    let iter = ToV8::to_v8(iter, scope).unwrap();
    res.set(iter);
}
crate :: convert :: impl_convert_native ! (biome_rowan :: AstSeparatedListNodesIterator < biome_json_syntax :: JsonLanguage , biome_json_syntax :: AnyJsonValue >);
crate :: convert :: impl_convert_native ! (biome_rowan :: AstSeparatedListNodesIterator < biome_json_syntax :: JsonLanguage , biome_json_syntax :: JsonMember >);