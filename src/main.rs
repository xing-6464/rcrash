fn main() {
    let myarray: [u32; 5] = [1, 2, 3, 4, 5];
    
    println!("myarray[1]={}", myarray[1]); // 2

    let mybuffer: [u32; 32 * 1024] = [0; 32 * 1024];

    println!("mybuffer[1024] = {}", mybuffer[1024]);

    
}
