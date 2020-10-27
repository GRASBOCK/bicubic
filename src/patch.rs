/// a unit square patch
/// 
//contains only coefficients necessary for interpolation
#[derive(Debug)]
pub struct Patch{
	a00: f64, 
	a01: f64,
	a02: f64,
	a03: f64,

	a10: f64, 
	a11: f64,
	a12: f64,
	a13: f64,

	a20: f64, 
	a21: f64,
	a22: f64,
	a23: f64,

	a30: f64, 
	a31: f64,
	a32: f64,
	a33: f64,
}

impl Patch{
	/// data for each point in the quad
	///
	/// order like this:
	/// (x=0, y=0), (1, 0), (0, 1), (1, 1)
	pub fn from_data(
		x_delta: f64,
		y_delta: f64,
		f: (f64, f64, f64, f64),
		fx: (f64, f64, f64, f64),
		fy: (f64, f64, f64, f64),
		fxy: (f64, f64, f64, f64)
	)-> Patch{
		let p = [f.0, f.1, f.2, f.3, 
							x_delta*fx.0, x_delta*fx.1, x_delta*fx.2, x_delta*fx.3,
							y_delta*fy.0, y_delta*fy.1, y_delta*fy.2, y_delta*fy.3,
							x_delta*y_delta*fxy.0, x_delta*y_delta*fxy.1, x_delta*y_delta*fxy.2, x_delta*y_delta*fxy.3];
		let a00 = p[0];
		let a10 = p[4];
		let a20 = -3.*p[0] + 3.*p[1] -2.*p[4] - p[5];
		let a30 = 2.*p[0] - 2.*p[1] + p[4] + p[5];

		let a01 = p[8];
		let a11 = p[12];
		let a21 = -3.*p[8] + 3.*p[9] -2.*p[12] - p[13];
		let a31 = 2.*p[8] - 2.*p[9] + p[12] + p[13];

		let a02 = -3.*p[0] + 3.*p[2] - 2.*p[8] -p[10];
		let a12 = -3.*p[4] + 3.*p[6] - 2.*p[12] -p[14];
		let a22 =	9.*p[0] - 9.*p[1] - 9.*p[2] + 9.*p[3]
						+6.*p[4] + 3.*p[5] - 6.*p[6] - 3.*p[7]
						+6.*p[8] - 6.*p[9] + 3.*p[10] - 3.*p[11]
						+4.*p[12] + 2.*p[13] + 2.*p[14] + p[15];
		let a32 =	-6.*p[0] +6.*p[1] +6.*p[2] -6.*p[3]
						-3.*p[4] -3.*p[5] +3.*p[6] +3.*p[7]
						-4.*p[8] +4.*p[9] -2.*p[10] +2.*p[11]
						-2.*p[12] -2.*p[13] -p[14] -p[15];

		let a03 = 2.*p[0] -2.*p[2] + p[8] +p[10];
		let a13 = 2.*p[4] -2.*p[6] +1.*p[12] +p[14];
		let a23 =	-6.*p[0] +6.*p[1] +6.*p[2] -6.*p[3]
						-4.*p[4] -2.*p[5] +4.*p[6] +2.*p[7]
						-3.*p[8] +3.*p[9] -3.*p[10] +3.*p[11]
						-2.*p[12] -p[13] -2.*p[14] -p[15];
		let a33 =	4.*p[0] -4.*p[1] -4.*p[2] +4.*p[3]
						+2.*p[4] +2.*p[5] -2.*p[6] -2.*p[7]
						+2.*p[8] -2.*p[9] +2.*p[10] -2.*p[11]
						+p[12] +p[13] +p[14] +p[15];

		Patch{	a00, a01, a02, a03,
				a10, a11, a12, a13,
				a20, a21, a22, a23,
				a30, a31, a32, a33}
	}

	/// samples x & y in the unit square (patch)
	pub fn sample(&self, x: f64, y: f64) -> f64{
		let result = self.a00+self.a01*y+self.a02*y*y+self.a03*y*y*y
		+(self.a10+self.a11*y+self.a12*y*y+self.a13*y*y*y)*x
		+(self.a20+self.a21*y+self.a22*y*y+self.a23*y*y*y)*x*x
		+(self.a30+self.a31*y+self.a32*y*y+self.a33*y*y*y)*x*x*x;
		result
	}
}
