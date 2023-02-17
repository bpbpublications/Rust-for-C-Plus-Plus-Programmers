use std::clone::Clone;
use std::fmt::Debug;

struct Baz<T, K, V>{
	t: T, 
	k: K, 
	v: V, 
}

trait Bar<T: Clone, K: Copy + Clone, V: Clone + Debug>{
	fn only_clone(&self) -> T;
	fn copy_clone(&self) -> K;
	fn debugged(&self) -> V;
}

impl<T: Clone, K: Copy + Clone, V: Clone +Debug> Bar<T, K, V> for Baz<T, K, V> {
	fn only_clone(&self) -> T{
		self.t.clone()
	}
	fn copy_clone(&self) -> K{
		self.k
	}
	fn debugged(&self) -> V{
		self.v.clone()
	}
}

fn main(){
	let baz = Baz{
		t: "hello".to_string(), // String implements Clone
		k: 11, // Primitives implement Copy + Clone 
		v: vec![1, 2, 4, 9] // Vec implements Debug and Clone
	};
	
	println!("T: {}", baz.only_clone()); // T: hello
	println!("K: {}", baz.copy_clone()); // K: 11
	println!("V: {:?}", baz.debugged()); // V: [1,2,4,9]
}
