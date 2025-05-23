fn main() {
    let mut list = vec![1, 2, 3, 4, 5];

    println!("{}", list.capacity());

    list.push(6);
    println!("{}", list.capacity());

    let mut list1: Vec<i32> = Vec::with_capacity(5);

    println!("{}", &list1.capacity());
    list1.push(1);
    println!("{}", &list1.capacity());
    list1.push(1);
    list1.push(1);
    list1.push(1);
    list1.push(1);
    list1.push(1);
    println!("{}", &list1.capacity());

    println!("{:?}", list);
    list.clear();
    println!("{:?}", list);
    println!("{}", list.capacity());

    let mut list2 = vec![1, 2, 3, 4, 5];

    let el = list2.remove(2);
    list2.remove(2);
    println!("Deleted is {}", el);
    println!("Capacity is {} vector {:?}", list2.capacity(), list2);

    list2.pop();
    println!("Capacity is {} vector {:?}", list2.capacity(), list2);
    list2.push(3);
    list2.push(4);
    list2.push(5);
    println!("Capacity is {} vector {:?}", list2.capacity(), list2);
    list2.truncate(2);
    println!("Capacity is {} vector {:?}", list2.capacity(), list2);

    list1.append(&mut list2);

    println!("Capacity is {} vector1 {:?}", list1.capacity(), list1);
    println!("Capacity is {} vector2 {:?}", list2.capacity(), list2);
}