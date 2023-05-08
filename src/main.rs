struct DataManager {
    my_private_data: Vec<i32>,
}

impl Default for DataManager {
    fn default() -> Self {
        DataManager {
            my_private_data: vec![1, 2, 3, 4, 5],
        }
    }
}

// impl DataManager {
//     fn get_data(&self) -> &Vec<i32> {
//         return &self.my_private_data;
//     }
// }

impl DataManager {
    fn get_data(&mut self) -> &mut Vec<i32> {
        &mut self.my_private_data
    }
}

fn main() {
    let mut data_manager = DataManager::default();

    let mut data = data_manager.get_data();

    data.push(6);
    let data: Vec<&mut i32> = data.into_iter().skip(2).take(4).collect();

    println!(
        "{}",
        data.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );

    println!(
        "{}",
        data_manager
            .get_data()
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
}
