use std::rc::Rc;

pub trait Evaluable {
    fn eval(&self) -> f64;
}

impl Evaluable for &dyn Evaluable {
    fn eval(&self) -> f64 {
        (*self).eval()
    }
}

pub type Node = Rc<Box<dyn Evaluable>>;

pub struct Leaf(pub f64);

impl Evaluable for Leaf {
    fn eval(&self) -> f64 {
        return self.0;
    }
}

pub struct Add(pub Node, pub Node);

impl Evaluable for Add {
    fn eval(&self) -> f64 {
        self.0.eval() + self.1.eval()
    }
}

pub struct Subtract(pub Node, pub Node);

impl Evaluable for Subtract {
    fn eval(&self) -> f64 {
        self.0.eval() - self.1.eval()
    }
}

pub struct Multiply(pub Node, pub Node);

impl Evaluable for Multiply {
    fn eval(&self) -> f64 {
        self.0.eval() * self.1.eval()
    }
}

pub struct Divide(pub Node, pub Node);

impl Evaluable for Divide {
    fn eval(&self) -> f64 {
        self.0.eval() / self.1.eval()
    }
}
