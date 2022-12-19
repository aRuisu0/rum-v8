use rum::rum::Vm;

fn main() {
	let mut rum = Vm::new_vm();
	rum.boot();
	rum.run();
}
