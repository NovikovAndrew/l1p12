use std::collections::HashSet;

fn main() {
    // Определяем два множества
    let set1: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let set2: HashSet<i32> = vec![4, 5, 6, 7, 8].into_iter().collect();

    // Находим пересечение
    let intersection: HashSet<_> = set1.intersection(&set2).copied().collect();

    // Выводим результат
    println!("Пересечение множеств: {:?}", intersection);
}
