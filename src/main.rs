mod structures;


fn main() {
    let mut stack: structures::stack::Stack<u64> = structures::stack::Stack::new();
    println!("{:}",stack.size());
    stack.push(2);
    stack.push(24);
    stack.push(25);
    println!("{:}",stack.size());
    stack.pop();
    println!("{:}",stack.size());
    println!("{:}",stack.peek());

}
