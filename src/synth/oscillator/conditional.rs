use std::{
    cell::{RefCell, RefMut}, collections::{
        btree_map::Entry::{Occupied, Vacant},
        BTreeMap,
    }, f32::consts::PI, sync::{Arc, Mutex, RwLock}, usize
};

use egui::ahash::HashMap;

use crate::synth::{Oscillator, OscillatorCtx};

use super::Simple;

pub enum ConditionalModifier {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Atoms {
    True,
    False,
    CurrentTime,
    CurrentTimeOffsetted(f32),

    CondOscValue,
    CondOscValueAt(f32),
    FirstOscValue,
    FirstOscValueAt(f32),
    SecondOscValue,
    SecondOscValueAt(f32),

    Value(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum BinaryBoolOps {
    And,
    Or,
    Implies,
    Xor,
    Nand,
    Nor,
}
impl BinaryBoolOps {
    fn eval(&self, l: Value, r: Value) -> bool {
        let l = l.as_bool();
        let r = r.as_bool();
        match self {
            BinaryBoolOps::And => l && r,
            BinaryBoolOps::Or => l || r,
            BinaryBoolOps::Implies => !l && r,
            BinaryBoolOps::Xor => l ^ r,
            BinaryBoolOps::Nand => !(l && r),
            BinaryBoolOps::Nor => !(l || r),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum CompOps {
    Geq,
    Leq,
    Ge,
    Le,
    Eq,
    Neq,
}
impl CompOps {
    fn eval(&self, l: Value, r: Value) -> bool {
        let l = l.as_float();
        let r = r.as_float();
        match self {
            CompOps::Geq => l >= r,
            CompOps::Leq => l <= r,
            CompOps::Ge => l > r,
            CompOps::Le => l < r,
            CompOps::Eq => l == r,
            CompOps::Neq => l != r,
        }
        
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Not;
impl Not {
    fn eval(&self, l: Value) -> Value {
       match l {
        Value::Float(f) => {
            let d: f64 = f.clamp(-1.0, 1.0).into();
            let u: f64 = usize::MAX as _;
            let n: usize = (d * u).floor() as _;
            let n : f64 = (!n) as f64 / u;
            (n as f32).into()
        },
           Value::Bool(b) => Value::Bool(!b),
           }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum BinaryExpr {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    Bitwise(BinaryBoolOps),
    Max,
    Min,
}
impl BinaryExpr {
    fn eval(&self, l: f32, r: f32) -> f32 {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum UnaryExpr {
    Inverse,
    Negate,
    Bitwise(Not),

    Exp,

    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,

    ASin,
    ACos,
    ATan,

    ASinh,
    ACosh,
    ATanh,

    Abs,
    Round,
    Ceil,
    Floor,
    Fract,
}
impl UnaryExpr {
    fn eval(&self, l: f32) -> f32 {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ExprOps {
    Binary(BinaryExpr),
    Unary(UnaryExpr),
}

impl From<BinaryExpr> for ExprOps {
    fn from(v: BinaryExpr) -> Self {
        Self::Binary(v)
    }
}

impl From<UnaryExpr> for ExprOps {
    fn from(v: UnaryExpr) -> Self {
        Self::Unary(v)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Expression {
    Atom(Atoms),
    Binary(BinaryExpr, usize, usize),
    Unary(UnaryExpr, usize),
}
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Condition {
    Comp(CompOps, usize, usize),
    BinaryBool(BinaryBoolOps, usize, usize),
    UnaryBool(Not, usize),
    Atom(Atoms),
    Expr(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ConditionOps {
    Binary(BinaryBoolOps),
    Unary(Not),
    Comp(CompOps),
}

impl From<CompOps> for ConditionOps {
    fn from(v: CompOps) -> Self {
        Self::Comp(v)
    }
}

impl From<Not> for ConditionOps {
    fn from(v: Not) -> Self {
        Self::Unary(v)
    }
}

impl From<BinaryBoolOps> for ConditionOps {
    fn from(v: BinaryBoolOps) -> Self {
        Self::Binary(v)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AdvancedIfElse {
    cond: Condition,
    truthy: Expression,
    falsy: Expression,

    cond_osc: Box<Simple>,
    first_osc: Box<Simple>,
    second_osc: Box<Simple>,

    conds: Vec<Condition>,
    exprs: Vec<Expression>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Value {
    Float(f32),
    Bool(bool),
}

impl Value {
    pub fn as_float(&self) -> f32 {
        match self {
            Value::Float(f) => *f,
            Value::Bool(true) => 1.0,
            Value::Bool(false) => -1.0,
        }
    }
    pub fn as_bool(&self) -> bool {
        match self {
            Value::Float(f) => *f > 0.0,
            Value::Bool(x) => *x,
        }
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Bool(v)
    }
}

impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Self::Float(v)
    }
}

pub struct OscMem {
    cond_osc_value: Option<f32>,
    first_osc_value: Option<f32>,
    second_osc_value: Option<f32>,

    cond_osc_value_at: BTreeMap<usize, f32>,
    first_osc_value_at: BTreeMap<usize, f32>,
    second_osc_value_at: BTreeMap<usize, f32>,
}
impl AdvancedIfElse {
    fn eval(&self, time: f32, ctx: OscillatorCtx)->f32 {
        let mut cond_osc_value: Option<f32> = None;
        let mut first_osc_value: Option<f32> = None;
        let mut second_osc_value: Option<f32> = None;

        let mut cond_osc_value_at: BTreeMap<usize, f32> = BTreeMap::default();
        let mut first_osc_value_at: BTreeMap<usize, f32> = BTreeMap::default();
        let mut second_osc_value_at: BTreeMap<usize, f32> = BTreeMap::default();

        let cond = self.eval_cond(
            time,
            ctx,
            &mut cond_osc_value,
            &mut first_osc_value,
            &mut second_osc_value,
            &mut cond_osc_value_at,
            &mut first_osc_value_at,
            &mut second_osc_value_at,
        );
        if cond {
            let truthy = self.truthy;
            self.eval_expr(
                truthy,
                time,
                ctx,
                &mut cond_osc_value,
                &mut first_osc_value,
                &mut second_osc_value,
                &mut cond_osc_value_at,
                &mut first_osc_value_at,
                &mut second_osc_value_at,
            )
        } else {
            let falsy = self.falsy;
            self.eval_expr(
                falsy,
                time,
                ctx,
                &mut cond_osc_value,
                &mut first_osc_value,
                &mut second_osc_value,
                &mut cond_osc_value_at,
                &mut first_osc_value_at,
                &mut second_osc_value_at,
            )
        }
    }

    fn eval_cond(
        &self,
        time: f32,
        ctx: OscillatorCtx,
        mut cond_osc_value: &mut Option<f32>,
        mut first_osc_value: &mut Option<f32>,
        mut second_osc_value: &mut Option<f32>,

        cond_osc_value_at: &mut BTreeMap<usize, f32>,
        first_osc_value_at: &mut BTreeMap<usize, f32>,
        second_osc_value_at: &mut BTreeMap<usize, f32>,
    ) -> bool {
        let mut values: Vec<Value> = vec![];
        let mut ops: Vec<ConditionOps> = vec![];

        let mut stack = vec![self.cond];
        loop {
            use Condition::*;
            match stack.pop() {
                Some(Comp(op, left, right)) => {
                    ops.push(op.into());
                    stack.push(self.conds[left]);
                    stack.push(self.conds[right]);
                }
                Some(BinaryBool(op, left, right)) => {
                    ops.push(op.into());
                    stack.push(self.conds[right]);
                    stack.push(self.conds[left]);
                }
                Some(UnaryBool(op, left)) => {
                    ops.push(op.into());
                    stack.push(self.conds[left]);
                }
                Some(Expr(idx)) => {
                    let expr = self.exprs[idx];
                    let result = self.eval_expr(
                        expr,
                        time,
                        ctx,
                        cond_osc_value,
                        first_osc_value,
                        second_osc_value,
                        cond_osc_value_at,
                        first_osc_value_at,
                        second_osc_value_at,
                    );
                    values.push(result.into());
                }
                Some(Atom(atom)) => match atom {
                    Atoms::True => values.push(true.into()),
                    Atoms::False => values.push(false.into()),
                    Atoms::CurrentTime => values.push(time.into()),
                    Atoms::CurrentTimeOffsetted(delta) => values.push(((time + delta) % PI).into()),
                    Atoms::CondOscValue => match &mut cond_osc_value {
                        Some(x) => {
                            values.push((*x).into());
                        }
                        x @ None => **x = Some(self.cond_osc.generate_sample(ctx, time)),
                    },
                    Atoms::CondOscValueAt(t) => {
                        let d: f64 = t.clamp(-1.0, 1.0).into();
                        let u: f64 = usize::MAX as _;
                        let n: usize = (d * u).floor() as _;

                        match cond_osc_value_at.entry(n) {
                            Vacant(vacant_entry) => {
                                let v = self.cond_osc.generate_sample(ctx, t);
                                vacant_entry.insert(v);
                            }
                            Occupied(occupied_entry) => {
                                values.push((*occupied_entry.get()).into());
                            }
                        }
                    }
                    Atoms::FirstOscValue => match &mut first_osc_value {
                        Some(x) => {
                            values.push((*x).into());
                        }
                        x @ None => **x = Some(self.cond_osc.generate_sample(ctx, time)),
                    },
                    Atoms::FirstOscValueAt(t) => {
                        let d: f64 = t.clamp(-1.0, 1.0).into();
                        let u: f64 = usize::MAX as _;
                        let n: usize = (d * u).floor() as _;

                        match first_osc_value_at.entry(n) {
                            Vacant(vacant_entry) => {
                                let v = self.first_osc.generate_sample(ctx, t);
                                vacant_entry.insert(v);
                            }
                            Occupied(occupied_entry) => {
                                values.push((*occupied_entry.get()).into());
                            }
                        }
                    }

                    Atoms::SecondOscValue => match &mut second_osc_value {
                        Some(x) => {
                            values.push((*x).into());
                        }
                        x @ None => **x = Some(self.second_osc.generate_sample(ctx, time)),
                    },
                    Atoms::SecondOscValueAt(t) => {
                        let d: f64 = t.clamp(-1.0, 1.0).into();
                        let u: f64 = usize::MAX as _;
                        let n: usize = (d * u).floor() as _;

                        match second_osc_value_at.entry(n) {
                            Vacant(vacant_entry) => {
                                let v = self.second_osc.generate_sample(ctx, t);
                                vacant_entry.insert(v);
                            }
                            Occupied(occupied_entry) => {
                                values.push((*occupied_entry.get()).into());
                            }
                        }
                    }

                    Atoms::Value(v) => {
                        values.push(v.into());
                    }
                },

                None => break,
            }
        }
        values.reverse();
        for op in ops.iter().rev() {
            match op {
                ConditionOps::Comp(op) => {
                    let r = values.pop().unwrap_or(0.0.into());
                    let l = values.pop().unwrap_or(0.0.into());
                    let result: bool = op.eval(l, r);
                    values.push(result.into());
                }
                ConditionOps::Binary(op) => {
                    let r = values.pop().unwrap_or(0.0.into());
                    let l = values.pop().unwrap_or(0.0.into());
                    let result: bool = op.eval(l, r);
                    values.push(result.into());
                }
                ConditionOps::Unary(op) => {
                    let l = values.pop().unwrap_or(0.0.into());
                    let result = op.eval(l);
                    values.push(result);
                }
            }
        }

        values.last().unwrap_or(&false.into()).as_bool()
    }

    fn eval_expr(
        &self,
        expr: Expression,
        time: f32,
        ctx: OscillatorCtx,
        mut cond_osc_value: &mut Option<f32>,
        mut first_osc_value: &mut Option<f32>,
        mut second_osc_value: &mut Option<f32>,

        cond_osc_value_at: &mut BTreeMap<usize, f32>,
        first_osc_value_at: &mut BTreeMap<usize, f32>,
        second_osc_value_at: &mut BTreeMap<usize, f32>,
    ) -> f32 {
        let mut values: Vec<f32> = vec![];
        let mut ops: Vec<ExprOps> = vec![];

        let mut stack = vec![expr];
        loop {
            use Expression::*;
            match stack.pop() {
                Some(Binary(op, left, right)) => {
                    ops.push(op.into());
                    stack.push(self.exprs[right]);
                    stack.push(self.exprs[left]);
                }
                Some(Unary(op, left)) => {
                    ops.push(op.into());
                    stack.push(self.exprs[left]);
                }
                Some(Atom(atom)) => match atom {
                    Atoms::True => values.push(1.0),
                    Atoms::False => values.push(-1.0),
                    Atoms::CurrentTime => values.push(time),
                    Atoms::CurrentTimeOffsetted(delta) => values.push((time + delta) % PI),
                    Atoms::CondOscValue => match &mut cond_osc_value {
                        Some(x) => {
                            values.push(*x);
                        }
                        x @ None => **x = Some(self.cond_osc.generate_sample(ctx, time)),
                    },
                    Atoms::CondOscValueAt(t) => {
                        let d: f64 = t.clamp(-1.0, 1.0).into();
                        let u: f64 = usize::MAX as _;
                        let n: usize = (d * u).floor() as _;

                        match cond_osc_value_at.entry(n) {
                            Vacant(vacant_entry) => {
                                let v = self.cond_osc.generate_sample(ctx, t);
                                vacant_entry.insert(v);
                            }
                            Occupied(occupied_entry) => {
                                values.push(*occupied_entry.get());
                            }
                        }
                    }
                    Atoms::FirstOscValue => match &mut first_osc_value {
                        Some(x) => {
                            values.push(*x);
                        }
                        x @ None => **x = Some(self.cond_osc.generate_sample(ctx, time)),
                    },
                    Atoms::FirstOscValueAt(t) => {
                        let d: f64 = t.clamp(-1.0, 1.0).into();
                        let u: f64 = usize::MAX as _;
                        let n: usize = (d * u).floor() as _;

                        match first_osc_value_at.entry(n) {
                            Vacant(vacant_entry) => {
                                let v = self.first_osc.generate_sample(ctx, t);
                                vacant_entry.insert(v);
                            }
                            Occupied(occupied_entry) => {
                                values.push(*occupied_entry.get());
                            }
                        }
                    }

                    Atoms::SecondOscValue => match &mut second_osc_value {
                        Some(x) => {
                            values.push(*x);
                        }
                        x @ None => **x = Some(self.second_osc.generate_sample(ctx, time)),
                    },
                    Atoms::SecondOscValueAt(t) => {
                        let d: f64 = t.clamp(-1.0, 1.0).into();
                        let u: f64 = usize::MAX as _;
                        let n: usize = (d * u).floor() as _;

                        match second_osc_value_at.entry(n) {
                            Vacant(vacant_entry) => {
                                let v = self.second_osc.generate_sample(ctx, t);
                                vacant_entry.insert(v);
                            }
                            Occupied(occupied_entry) => {
                                values.push(*occupied_entry.get());
                            }
                        }
                    }

                    Atoms::Value(v) => {
                        values.push(v);
                    }
                },
                None => break,
            }
        }
        values.reverse();
        for op in ops.iter().rev() {
            match op {
                ExprOps::Binary(op) => {
                    let r = values.pop().unwrap_or(0.0);
                    let l = values.pop().unwrap_or(0.0);
                    let result: f32 = op.eval(l, r);
                    values.push(result);
                }
                ExprOps::Unary(op) => {
                    let l = values.pop().unwrap_or(0.0);
                    let result: f32 = op.eval(l);
                    values.push(result);
                }
            }
        }

        values.last().unwrap_or(&0.0).clamp(-1.0, 1.0)
    }
}
/*
struct AdvancedIfElseWrapper(Mutex<Box<RefCell<AdvancedIfElse>>>);

impl  Oscillator for AdvancedIfElseWrapper {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.0.lock().expect("wuhuu poisened").borrow_mut().eval(time, ctx)
        
    }
}
*/
impl Oscillator for AdvancedIfElse{
    fn generate_sample(& self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.eval(time, ctx)
    }
    }
pub enum ConditionalOscillator {
    IfElse(IfElse),
    GreaterThan(GreaterThan),
}
impl Oscillator for ConditionalOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        match self {
            ConditionalOscillator::IfElse(osc) => osc.generate_sample(ctx, time),
            ConditionalOscillator::GreaterThan(osc) => osc.generate_sample(ctx, time),
        }
    }
}

pub struct IfElse {
    condition: Box<dyn Oscillator>,
    then_osc: Box<dyn Oscillator>,
    else_osc: Box<dyn Oscillator>,
}

impl IfElse {
    pub fn new(
        condition: Box<dyn Oscillator>,
        then_osc: Box<dyn Oscillator>,
        else_osc: Box<dyn Oscillator>,
    ) -> Self {
        Self {
            condition,
            then_osc,
            else_osc,
        }
    }

    pub fn update_condition(&mut self, condition: Box<dyn Oscillator>) {
        self.condition = condition;
    }

    pub fn update_then(&mut self, then_osc: Box<dyn Oscillator>) {
        self.then_osc = then_osc;
    }

    pub fn update_else(&mut self, else_osc: Box<dyn Oscillator>) {
        self.else_osc = else_osc;
    }
}

impl Oscillator for IfElse {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        if self.condition.generate_sample(ctx.clone(), time) > 0.0 {
            self.then_osc.generate_sample(ctx, time)
        } else {
            self.else_osc.generate_sample(ctx, time)
        }
    }
}

pub struct GreaterThan {
    osc1: Box<dyn Oscillator>,
    osc2: Box<dyn Oscillator>,
}

impl GreaterThan {
    pub fn new(osc1: Box<dyn Oscillator>, osc2: Box<dyn Oscillator>) -> Self {
        Self { osc1, osc2 }
    }

    pub fn update_osc1(&mut self, osc1: Box<dyn Oscillator>) {
        self.osc1 = osc1;
    }

    pub fn update_osc2(&mut self, osc2: Box<dyn Oscillator>) {
        self.osc2 = osc2;
    }
}

impl Oscillator for GreaterThan {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        if self.osc1.generate_sample(ctx.clone(), time) > self.osc2.generate_sample(ctx, time) {
            1.0
        } else {
            0.0
        }
    }
}
