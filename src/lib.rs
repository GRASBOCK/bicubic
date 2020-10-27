mod patch;
/// contains all necessary information to sample arbitrary points
///
/// # Example
///
/// ```
/// let x = vec![0.0, 6.0];
/// let y = vec![0.0, 5.0];
/// let (f, fx, fy, fxy) = (vec![0.0; 4], vec![0.0; 4], vec![0.0; 4], vec![0.0; 4]);
/// let bci = bicubic::from_vec(&x, &y, &f, &fx, &fy, &fxy);
/// let z = bci.sample(5.6, 3.5);
///  ``` 
///
/// # Panics
///
/// [`panic!`] when one of the f vectors has bad dimensions
///
/// [`panic!`]: https://doc.rust-lang.org/std/macro.panic.html
pub struct Bicubic{
	x: Vec<f64>, 
	y: Vec<f64>,
	patches: Vec<patch::Patch>
}

impl Bicubic{
	/// finds the correct patch to use for the interpolation at specified position
	///
	/// returns an option that can contain a patch with its x, y translated into the patches coordinates
	fn find_patch(&self, x: f64, y: f64) -> Option<(&patch::Patch, f64, f64)>{
		if x <= self.x[0]{
			// extrapolate linear
			return None;
		}
		if x >= self.x[self.x.len()-1]{
			// extrapolate linear
			return None;
		}
		if y <= self.y[0]{
			// extrapolate linear
			return None;
		}
		if y >= self.y[self.y.len()-1]{
			// extrapolate linear
			return None;
		}
		//do binary search in y
		let mut y_upper = self.y.len()-1;
		let mut y_lower = 0;
		while y_upper-y_lower > 1{
			let center = (y_upper+y_lower)/2;
			if y > self.y[center]{
				y_lower = center;
			}else{
				y_upper = center;
			}
		}
		//do binary search in x
		let mut x_upper = self.x.len()-1;
		let mut x_lower = 0;
		while x_upper-x_lower > 1{
			let center = (x_upper+x_lower)/2;
			if x > self.x[center]{
				x_lower = center;
			}else{
				x_upper = center;
			}
		}
		//println!("patches len = {}, y_lower {}, x_lower {}", self.patches.len(), y_lower, x_lower);
		let px = (x - self.x[x_lower]) / (self.x[x_upper]-self.x[x_lower]);
		let py = (y - self.y[y_lower]) / (self.y[y_upper]-self.y[y_lower]);
		Some((&self.patches[y_lower*(self.x.len()-1) + x_lower], px, py))
	}

	/// samples at a given x, y
	///
	/// outside of grid will be 0 (create an issue if you require extrapolation)
	pub fn sample(&self, x:f64, y: f64) -> f64{
		match self.find_patch(x, y){
			Some((patch, px, py)) => patch.sample(px, py),
			None => 0.0
		}
	}
}

/// takes multiple datavectors and creates a bicubic Interpolation from them
///
/// x & y need to be sorted in ascending order
///
/// f needs to be in rowwise order and sorted from bottom-left to top-right 
pub fn from_vec(
	x: &Vec<f64>, 
	y: &Vec<f64>,
	f: &Vec<f64>,
	fx: &Vec<f64>,
	fy: &Vec<f64>,
	fxy: &Vec<f64>
) -> Bicubic{
	assert!(f.len() == x.len()*y.len(), "[Bicubic::from_vec] f has bad dimensions (f.len() = {}). Needs to be x.len()*y.len() = {}", f.len(), x.len()*y.len());
	assert!(fx.len() == f.len(), "[Bicubic::from_vec] fx needs to be of same length as f");
	assert!(fy.len() == f.len(), "[Bicubic::from_vec] fy needs to be of same length as f");
	assert!(fxy.len() == f.len(), "[Bicubic::from_vec] fxy needs to be of same length as f");

	let row_len = x.len();
	let mut patches: Vec<patch::Patch> = Vec::with_capacity((x.len()-1)*(y.len()-1));
	for yi in 0..y.len()-1{
		for xi in 0..x.len()-1{
			let x_min = x[xi];
			let x_max = x[xi+1];
			let y_min = y[yi];
			let y_max = y[yi+1];

			//create patch
			let x_delta = x_max - x_min;
			let y_delta = y_max - y_min;

			let patch_index = xi + yi*row_len;
			let f : (f64, f64, f64, f64) = (
				f[patch_index], f[patch_index+1], f[patch_index+row_len], f[patch_index+row_len+1]
			);
			let fx : (f64, f64, f64, f64) = (
				fx[patch_index], fx[patch_index+1], fx[patch_index+row_len], fx[patch_index+row_len+1]
			);
			let fy : (f64, f64, f64, f64) = (
				fy[patch_index], fy[patch_index+1], fy[patch_index+row_len], fy[patch_index+row_len+1]
			);
			let fxy : (f64, f64, f64, f64) = (
				fxy[patch_index], fxy[patch_index+1], fxy[patch_index+row_len], fxy[patch_index+row_len+1]
			);
			let patch = patch::Patch::from_data(x_delta, y_delta, f, fx, fy, fxy);
			println!("patch: {:?}", patch);
			patches.push(patch);
		}	
	}
	Bicubic{x: x.clone(), y: y.clone(), patches}
}

mod tests;