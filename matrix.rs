
struct mat {
    mut m_ptr: ~[float],
    dims: [uint * 2]
}

// Row-major ordering for now
impl mat {
    static fn from_fn(dims: [uint * 2], op: &fn(uint, uint) -> float) -> mat {
        mat { 
            m_ptr: vec::from_fn(dims[1] * dims[0], |idx| op(idx/dims[1], idx % dims[1])), 
            dims: dims 
        }
    }

    static fn eye(size: uint) -> mat {
        mat {
            m_ptr: vec::from_fn(size * size, |idx| if ((idx - (idx / size)) % size) == 0 {1 as float} else {0 as float}),
            dims: [size, size]
        }
    }
    
    fn get_elm(&self, x: uint, y: uint) -> float {
        self.m_ptr[x * self.dims[0] + y]
    }
}

fn dot(u: ~[float], v: ~[float]) {
    
}

fn linspace (start: float, end: float, size: uint) -> ~[float] {
    vec::from_fn(size, |idx| start + idx as float * (end - start) / (size as float - 1.0))
}

fn main() {
    let v = linspace(0.0, 1.0, 100);
    let u = mat::from_fn([3, 4], |x, y| (x*y) as float);
    io::println(fmt!("%?\n\n%?", v, u));
}
