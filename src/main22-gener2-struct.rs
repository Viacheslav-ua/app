struct Data<T, K> {
    d1: T,
    d2: K,
}

impl<T, K> Data<T, K> {
    fn get_data_1(&self) -> &T {
        &self.d1
    }

    fn get_data_2(&self) -> &K {
        &self.d2
    }
}

fn main() {
    let data = Data { d1: 10, d2: 20 };
    let data2 = Data { d1: "atuyt", d2: 67.45 };

    println!("Data 1: {:?}", data.get_data_1());
    println!("Data 2: {:?}", data2.get_data_2());
    println!("Data 1: {:?}", data2.get_data_1());
    println!("Data 2: {:?}", data.get_data_2());
}
