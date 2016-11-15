use super::bezier_patch::BezierPatch;
use super::{EvalS, Eval, Tessellate, CountVertices};

use std::vec::Vec;
use std::cmp::max;
use std::fs::File;
use std::io::BufReader;

use auxiliary::{NumberScanner, Vertex};

use na;
use na::{Point3, Vector3};


#[derive(Debug)]
pub struct BezierObject {
    patches: Vec<BezierPatch>,
    n_sample: usize,
}

impl BezierObject {
    pub fn new(n_sample: usize) -> BezierObject {
        BezierObject {
            patches: vec![],
            n_sample: n_sample,
        }
    }

    pub fn refresh(&mut self) -> Vec<Vertex> {
        self.eval();
        self.tess()
    }

    pub fn upsample(&mut self) {
        self.n_sample += 1;
    }

    pub fn downsample(&mut self) {
        self.n_sample = max(2, self.n_sample - 1);
    }

    pub fn read_bezier_file(&mut self, fp: &str) {
        let f = File::open(fp).unwrap();
        let mut numbers = NumberScanner::new(BufReader::new(f));

        loop {
            if let Some(n) = numbers.next::<usize>() {
                if n > 0 {
                    for _ in 0..n {
                        let uc = numbers.next::<usize>().unwrap();
                        let vc = numbers.next::<usize>().unwrap();
                        let mut cs = vec![vec![na::origin::<Point3<f64>>(); uc + 1]; vc + 1];

                        for v in (0..(vc + 1)).rev() {
                            for u in 0..(uc + 1) {
                                let x = numbers.next::<f64>().unwrap();
                                let y = numbers.next::<f64>().unwrap();
                                let z = numbers.next::<f64>().unwrap();

                                cs[v][u].x = x;
                                cs[v][u].y = y;
                                cs[v][u].z = z;
                            }
                        }

                        self.patches.push(BezierPatch::new(&cs));
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}

impl EvalS for BezierObject {
    fn eval(&mut self) {
        for mut bz in self.patches.iter_mut() {
            bz.eval(self.n_sample);
        }
    }
}

impl Tessellate for BezierObject {
    fn tess(&self) -> Vec<Vertex> {
        let mut ret = Vec::with_capacity(self.n_vertices());

        for p in self.patches.iter().map(BezierPatch::tess) {
            ret.extend(p);
        }

        ret
    }
}

impl CountVertices for BezierObject {
    fn n_vertices(&self) -> usize {
        self.patches.iter().map(BezierPatch::n_vertices).sum()
    }
}
