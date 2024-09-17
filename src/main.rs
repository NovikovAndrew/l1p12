use std::collections::HashSet;

fn main() {
    // Создаем два множества
    let mut set1 = HashSet::new();
    set1.insert(1);
    set1.insert(2);
    set1.insert(3);

    let mut set2 = HashSet::new();
    set2.insert(2);
    set2.insert(3);
    set2.insert(4);

    // Находим пересечение множеств
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();

    // Выводим результат
    println!("Пересечение множеств: {:?}", intersection);
}
