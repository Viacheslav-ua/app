
fn main() {
  let nums = [10,3,2,10,3,5,7,5,8,9,8,3,0,10,1,2,3,4,5,6,7,8,9,10];
  let chars = ['a','b','c','t','d','e','f','d','g','h','i','j','k','l','m','n','a','o','g','p','q','r','s','t'];

  let duplicates = find_duplicates(&nums);
  let duplicates_char = find_duplicates(&chars);
  println!("Duplicates: {:?}", duplicates);
  println!("Duplicates: {:?}", duplicates_char);
}

fn find_duplicates<T>(list: &[T]) -> Vec<T> 
where T: PartialEq + Copy
{
  let mut duplicates: Vec<T> = Vec::new();
  
  for i in 0..list.len() {
    for j in (i+1)..list.len() {
      if list[i] == list[j] && !duplicates.contains(&list[i]) {
        duplicates.push(list[i]);
      }
    }
  }

  duplicates
}