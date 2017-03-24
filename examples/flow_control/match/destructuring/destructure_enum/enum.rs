// `allow` необходим, чтобы компилятор не выводил предупреждения,
// т.к используется только один вариант
#[allow(dead_code)]
enum Color {
    // Эти 3 перечисления определяют цвет по названию.
    Red,
    Blue,
    Green,
    // Остальные используют `u32` кортежи для идентификации цветовых моделей.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);
    // ЗАДАНИЕ ^ Попробуйте другие значения для `color`

    println!("Какой это цвет?");
    // `enum` может быть деструктурирован с помощью `match`.
    match color {
        Color::Red   => println!("Красный цвет!"),
        Color::Blue  => println!("Синий цвет!"),
        Color::Green => println!("Зелёный цвет!"),
        Color::RGB(r, g, b) =>
            println!("Красный: {}, зелёный: {}, и синий: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Тон: {}, насыщенность: {}, значение: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Тон: {}, насыщенность: {}, светлота: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Голубой: {}, пурпурный: {}, жёлтый: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Голубой: {}, пурпурный: {}, жёлтый: {}, key (чёрный): {}!",
                c, m, y, k),
        // Нет необходимости в других ветвях, т.к были рассмотрены все варианты
    }
}
