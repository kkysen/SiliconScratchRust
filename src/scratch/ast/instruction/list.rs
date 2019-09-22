use crate::scratch::ast::instruction::Value;

pub enum ListInstruction<'a> {
    Void(VoidListInstruction<'a>),
    Returning(ReturningListInstruction<'a>),
}

pub enum VoidListInstruction<'a> {
    Set(Value<'a>, Value<'a>),
    Insert(Value<'a>, Value<'a>),
    Remove(Value<'a>),
    Push(Value<'a>, PushPop),
    // Push is like a constant time version of Insert and Remove
    Clear(),
}

pub enum ReturningListInstruction<'a> {
    Length(),
    Get(Value<'a>),
    Pop(PushPop),
    Find(Value<'a>),
    Contains(Value<'a>),
    ContainsSeq(&'a Value<'a>),
    Concat(&'a Value<'a>),
}

pub struct PushPop {
    side: PushPopSide,
    index: u8,
    // also support popping the nth to last element, where n is a small constant
    // 0 is the normal behavior (e.x. pop the 0th element)
}

pub enum PushPopSide {
    Left,
    Right,
}
