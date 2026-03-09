use steel::SteelVal;
use steel::steel_vm::register_fn::RegisterFn;
use steel::steel_vm::engine::Engine;

fn steel_val_tester(val: SteelVal) {
    match val{
        SteelVal::Closure(_) => println!("Closure"),
        SteelVal::BoolV(_) => println!("BoolV"),
        SteelVal::NumV(_) => println!("NumV"),
        SteelVal::Custom(_) => println!("Custom"),
        _ => println!("other")
    }
}


pub fn register_steel_functions(vm: &mut Engine){
    vm.register_fn("steel-tester", steel_val_tester);
}
