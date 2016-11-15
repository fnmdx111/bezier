use auxiliary::Vertex;

pub trait Eval {
    fn eval(&mut self, n_sample: usize);
}

pub trait EvalS {
    fn eval(&mut self);
}

pub trait Tessellate {
    fn tess(&self) -> Vec<Vertex>;
}

pub trait CountVertices {
    fn n_vertices(&self) -> usize;
}
