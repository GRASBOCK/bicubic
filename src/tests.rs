
#[cfg(test)]
mod tests {

	fn output(file: String, sampling: &dyn Fn(f64, f64)-> f64, x_min: f64, x_max: f64, y_min: f64, y_max: f64, delta: f64){
		let x_steps = ((x_max - x_min)/delta) as usize;
		let y_steps = ((y_max - y_min)/delta) as usize;
		println!("x_steps: {}, y_steps {}", x_steps, y_steps);
		let mut contents = String::default();
		for yi in 0..y_steps{
			for xi in 0..x_steps{
				let x = xi as f64*delta+x_min;
				let y = yi as f64*delta+y_min;
				let z= sampling(x, y);
				contents.push_str(format!("{} {} {}\n", x, y, z).as_str());
			}
		}
		std::fs::write(&file, contents).unwrap();
	}
	
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
		
		let x = vec![-2.5, 0.0, 1.5];
		let y = vec![-4.5, 3.2];
		let f = vec![12.4, 1.45, 1.33, 13.4, 13.2, 6.];
		let fx = vec![1.4, -3., 2., 5., -2., -0.3];
		let fy = vec![5.4, -2., 3., 7., -2., 3.];
		let fxy = vec![1.4, -0.2, 2., 0.5, -0.6, 0.3];
		/*
		let fx = vec![0.; f.len()];
		let fy = vec![0.; f.len()];
		let fxy = vec![0.; f.len()];
		*/

		let bicubic = crate::from_vec(&x, &y, &f, &fx, &fy, &fxy);

		let x_min = x[0]-0.2;
		let x_max = x[x.len()-1]+0.2;
		let y_min = y[0]-0.2;
		let y_max = y[y.len()-1]+0.2;
		output(format!("testing/manual"), &|x, y|bicubic.sample(x, y), x_min, x_max, y_min, y_max, 0.1);
	}
}
