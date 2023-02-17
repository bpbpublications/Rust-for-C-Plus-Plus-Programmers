// Turned our Robot class to empty struct 
struct Robot;
// To show default values 
struct DefaultRobot;
// Instruction trait 
// Either functions can be declared or 
// set like fn foo(param)->return;
pub trait Instructions{
    fn d_up(&self){println!("Moving up!")}
    fn d_down(&self){println!("Moving down!")}
    fn d_left(&self){println!("Moving left!")}
    fn d_right(&self){println!("Moving right!")}
}
// Implemented Instructions to Robot using impl <trait> for <struct>
impl Instructions for Robot{
    // overrided the functions 
    fn d_left(&self){println!("Moving right!")}
    fn d_right(&self){println!("Moving left!")}
}
impl Instructions for DefaultRobot{
    // keeping defaults;
}
fn main(){
    let robot: Robot = Robot;
    let defrobot: DefaultRobot = DefaultRobot;
    println!("Instructions: ");
    defrobot.d_left();
    defrobot.d_right();
    println!("Robot: ");
    robot.d_left();
    robot.d_right();
}
/* Output: 
Instructions: 
Moving left!
Moving right!
Robot: 
Moving right!
Moving left!
 */