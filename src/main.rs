// ประกาศฟังก์ชั่นหลักของโปรแกรม
// main เป็นจุดเริ่มต้นของโปรแกรม Rust ทุกโปรแกรมต้องมีฟังก์ชั่น main เพื่อให้สามารถรันได้
// () หลังจาก main หมายถึงว่าไม่มีพารามิเตอร์ที่ต้องส่งเข้าไปในฟังก์ชั่นนี้
// function block คือส่วนของโค้ดที่อยู่ภายในเครื่องหมายปีกกา {} ซึ่งเป็นส่วนที่กำหนดการทำงานของฟังก์ชั่น
// fn main() {
//     // ! เป็น macro ใน Rust ที่ใช้สำหรับพิมพ์ข้อความออกทางหน้าจอ
//     // ; เป็นตัวแยกระหว่าง expression และ statement ใน Rust
//     println!("Hello, world!");
// }

// ข้อดีของ Rust:
// High Forformance: Rust มีประสิทธิภาพสูงเนื่องจากการจัดการหน่วยความจำที่มีประสิทธิภาพ
// No Garbage Collector: Rust ไม่มีระบบเก็บขยะ (Garbage Collector) ซึ่งช่วยลดการใช้หน่วยความจำและเพิ่มประสิทธิภาพ
// parallelism: Rust มีระบบการจัดการการทำงานพร้อมกันที่มีประสิทธิภาพ ซึ่งช่วยให้สามารถใช้ประโยชน์จากหลายคอร์ของ CPU ได้อย่างเต็มที่
// Memory Safety: Rust มีระบบการจัดการหน่วยความจำที่ปลอดภัย ซึ่งช่วยลดปัญหาเกี่ยวกับการจัดการหน่วยความจำ เช่น การใช้หน่วยความจำที่ไม่ได้รับอนุญาตหรือการรั่วไหลของหน่วยความจำ
// Safe Concurrency: Rust มีระบบการจัดการการทำงานพร้อมกันที่ปลอดภัย ซึ่งช่วยลดปัญหาเกี่ยวกับการทำงานพร้อมกัน เช่น การ
// Scalability: Rust สามารถใช้ในการพัฒนาแอปพลิเคชันที่มีความสามารถในการปรับขนาดได้ดี เช่น เว็บเซิร์ฟเวอร์และระบบเครือข่าย

// Benefits of Rust:
// No Null Pointers: Rust ไม่มี null pointers ซึ่งช่วยลดปัญหาเกี่ยวกับการเข้าถึงข้อมูลที่ไม่ได้รับอนุญาต
// No Double Free: Rust ไม่มีปัญหาเกี่ยวกับการปล่อยหน่วยความจำสองครั้ง ซึ่งช่วยลดปัญหาเกี่ยวกับการจัดการหน่วยความจำ
// No Data Races: Rust ไม่มีปัญหาเกี่ยวกับการเข้าถึงข้อมูลที่ไม่ได้รับอนุญาตในสภาพแวดล้อมที่มีการทำงานพร้อมกัน
// No Use After Free: Rust ไม่มีปัญหาเกี่ยวกับการใช้หน่วยความจำที่ถูกปล่อยแล้ว ซึ่งช่วยลดปัญหาเกี่ยวกับการจัดการหน่วยความจำ

// fn main() {
//     let mut x = 10;
//     print!("x: {x}");
//     x = 20;
//     print!("x: {x}");
// }

// Scalar Types: Integers
// Signed integers: i8, i16, i32, i64, i128
// isize: ขนาดของตัวแปรขึ้นอยู่กับสถาปัตยกรรมของระบบ (32 บิตหรือ 64 บิต)

// Unsigned integers: u8, u16, u32, u64, u128
// usize: ขนาดของตัวแปรขึ้นอยู่กับสถาปัตยกรรมของระบบ (32 บิตหรือ 64 บิต)

// Scalar Types: Float, Char, Bool
// f32, f64: ตัวเลขทศนิยมที่มีความแม่นยำต่างกัน
// char: ตัวอักษร Unicode ที่มีขนาด 4 ไบต์ 32 bits
// bool: ค่าความจริงที่มีค่าเป็น true หรือ false 8 bits

// Literal Syntax & Widths
// Legible Integer Literals: 1_000_000 (สามารถใช้ _ เพื่อเพิ่มความอ่านง่ายของตัวเลข)
// Type Suffixes: 42u8 (สามารถใช้ suffix เพื่อระบุชนิดของตัวแปร เช่น u8, i32, f64 เป็นต้น)
// Bit Widths: i8, i16, i32, i64, i128 (ชนิดของตัวแปรที่มีความกว้างของบิตต่างกัน) 8 bit => 1 byte, 16 bit => 2 byte, 32 bit => 4 byte, 64 bit => 8 byte, 128 bit => 16 byte

// fn main() {
//     let x: i32 = 10; // ประกาศตัวแปร x เป็นชนิด i32 และกำหนดค่าเริ่มต้นเป็น 10
//     let y: f64 = 3.14; // ประกาศตัวแปร y เป็นชนิด f64 และกำหนดค่าเริ่มต้นเป็น 3.14
//     let z: char = 'A'; // ประกาศตัวแปร z เป็นชนิด char และกำหนดค่าเริ่มต้นเป็น 'A'
//     let is_true: bool = true; // ประกาศตัวแปร is_true เป็นชนิด bool และกำหนดค่าเริ่มต้นเป็น true

//     println!("x: {x}, y: {y}, z: {z}, is_true: {is_true}"); // พิมพ์ค่าของตัวแปร x, y, z, และ is_true ออกทางหน้าจอ
// }

// fn interproduct(a: i16, b: i16, c: i16) -> i16 {
//     a * b * c + b * c + c * a + a * b + a + b + c
// }

// fn main() {
//     println!("interproduct: {}", interproduct(120, 300, 400)); // เรียกใช้ฟังก์ชั่น interproduct โดยส่งค่า 2, 3, และ 4 และพิมพ์ผลลัพธ์ออกทางหน้าจอ
// }

// fn takes_u32(x: u32) {
//     println!("takes_u32: {x}"); // พิมพ์ค่าของตัวแปร x ออกทางหน้าจอ
// }

// fn takes_i8(x: i8) {
//     println!("takes_i8: {x}"); // พิมพ์ค่าของตัวแปร x ออกทางหน้าจอ
// }

// fn main() {
//     let x = 10;
//     let y = 20;

//     takes_u32(x);
//     takes_i8(y);
// }

// excercise : Fibonacci (1/3)
// The Fibonacci sequence begins with 0 and 1.
// For n > 1, the next number in the sequence is the sum of the previous two numbers.
// ( fn = fn-1 + fn-2 )

// F(0) = 0
// F(1) = 1
// F(2) = F(1) + F(0) = 1 + 0 = 1
// F(3) = F(2) + F(1) = 1 + 1 = 2
// F(4) = F(3) + F(2) = 2 + 1 = 3
// F(5) = F(4) + F(3) = 3 + 2 = 5

// write a function fib(n: u32) that calculates the nth Fibonacci number
// fn fib(n: u32) -> u32 {
//     if n < 2 {
//         return n;
//     } else {
//         return fib(n - 1) + fib(n - 2);
//     }
// }

// fn fib_iter(n: u32) -> u32 {
//     if n < 2 {
//         return n;
//     }
//     let mut a = 0;
//     let mut b = 1;

//     for _ in 2..=n {
//         let next = a + b;
//         a = b;
//         b = next;
//     }

//     b
// }

// fn main() {
//     println!("fib(0): {}", fib(0)); // fib(0) = 0
//     println!("fib(1): {}", fib(1)); // fib(1) = 1
//     println!("fib(2): {}", fib(2)); // fib(2) = 1
//     println!("fib(3): {}", fib(3)); // fib(3) = 2
//     println!("fib(4): {}", fib(4)); // fib(4) = 3
//     println!("fib(5): {}", fib(5)); // fib(5) = 5
//     println!("fib(5): {}", fib(10));

//     println!("fib_iter(0): {}", fib_iter(0)); // fib_iter(0) = 0
//     println!("fib_iter(1): {}", fib_iter(1)); // fib_iter(1
//     println!("fib_iter(2): {}", fib_iter(2)); // fib_iter(2) = 1
//     println!("fib_iter(3): {}", fib_iter(3)); // fib_iter(3
//     println!("fib_iter(4): {}", fib_iter(4)); // fib_iter(4) = 3
//     println!("fib_iter(5): {}", fib_iter(5)); // fib_iter(5
// }

// fn main() {
//     let z = 13;
//     let x = {
//         let y = 7;
//         y + z
//     };
//     dbg!(x);
//     dbg!(z);
//     // dbg!(y); // This line would cause an error because y is not in scope here

//     let a = 30;
//     // let size = if a < 10 {
//     //     "small"
//     // } else if a < 20 {
//     //     "medium"
//     // } else {
//     //     "large"
//     // };
//     let size = if a < 10 {
//         "small"
//     } else if a < 20 {
//         "medium"
//     } else {
//         "large"
//     };
//     dbg!(size);
// }

// fn main() {
//     let val = 1;
//     match val {
//         1 => println!("one"),
//         _ => println!("something else"),
//     };

//     let flag = true;
//     match flag {
//         true => println!("true"),
//         false => println!("false"),
//     };
// }

// fn main() {
//     let mut count = 16;
//     while count >= 10 {
//         count = count / 2;
//         // จะออกทุกค่าที่ อยู่ใน loop นี้จนกว่า count จะน้อยกว่า 10
//         dbg!(count);
//     }
//     // จะออกค่าที่อยู่หลัง loop นี้ ซึ่งจะเป็นค่าที่น้อยกว่า 10
//     dbg!(count);
// }

// fn main() {
//     // 0..5 เป็น range ที่เริ่มต้นที่ 0 และสิ้นสุดที่ 5 (ไม่รวม 5)
//     for x in 0..5 {
//         println!("x: {x}");
//         // x: 0
//         // x: 1
//         // x: 2
//         // x: 3
//         // x: 4
//     }

//     // 0..=5 เป็น range ที่เริ่มต้นที่ 0 และสิ้นสุดที่ 5 (รวม 5)
//     for x in 0..=5 {
//         println!("x: {x}");
//         // x: 0
//         // x: 1
//         // x: 2
//         // x: 3
//         // x: 4
//         // x: 5
//     }

//     for elm in [10, 20, 30, 40, 50] {
//         println!("elm: {elm}");
//         // elm: 10
//         // elm: 20
//         // elm: 30
//         // elm: 40
//         // elm: 50
//     }
// }

// fn main() {
//     let mut i = 0;
//     let x = loop {
//         i += 1;
//         dbg!(i);
//         // ถ้าไม่มี break จะทำให้ loop นี้ทำงานไม่รู้จบและจะออกค่าของ i ที่เพิ่มขึ้นเรื่อยๆ จนกว่าโปรแกรมจะถูกหยุดด้วยวิธีอื่น เช่น การกด Ctrl+C หรือการปิดโปรแกรม
//         // จะออกค่าที่อยู่ใน loop นี้จนกว่า i จะมากกว่าหรือเท่ากับ 5
//         if i >= 5 {
//             break "x is 5 or more"; // จะออกค่าที่อยู่หลัง break นี้ ซึ่งจะเป็นค่าที่ถูกกำหนดโดย break ใน loop นี้
//         }
//     };
//     dbg!(x); // จะออกค่าที่อยู่หลัง loop นี้ ซึ่งจะเป็นค่าที่ถูกกำหนดโดย break ใน loop นี้
// }

// fn main() {
//     let mut i = 0;
//     loop {
//         i += 1;
//         if i > 5 {
//             break; // จะออกจาก loop นี้เมื่อ i มากกว่า 5
//         }

//         if i % 2 == 0 {
//             continue; // จะข้ามการทำงานใน loop นี้เมื่อ i เป็นเลขคู่
//         }
//     }
//     dbg!(i); // จะออกค่าที่อยู่หลัง loop นี้ ซึ่งจะเป็นค่าที่ถูกกำหนดโดย break ใน loop นี้
// }

// fn main() {
//     let s = [[1, 2], [3, 4], [5, 6]];
//     let mut element_search = 0;
//     let target_value = 4;

//     'outer: for i in 0..=2 {
//         for j in 0..=1 {
//             if s[i][j] == target_value {
//                 element_search += 1;
//                 break 'outer;
//             }
//         }
//     }
//     dbg!(element_search);
// }

// fn main() {
//     todo!(); // เป็น macro ที่ใช้สำหรับแสดงข้อความว่า "ยังไม่เสร็จสมบูรณ์" หรือ "ยังไม่พร้อมใช้งาน" ซึ่งจะทำให้โปรแกรมหยุดทำงานและแสดงข้อความนี้ออกทางหน้าจอ
//     assert!(false, "This is an assertion failure"); // เป็น macro ที่ใช้สำหรับตรวจสอบเงื่อนไขที่กำหนด และถ้าเงื่อนไขนั้นไม่เป็นจริง จะทำให้โปรแกรมหยุดทำงานและแสดงข้อความที่กำหนดออกทางหน้าจอ
//     unreachable!(); // เป็น macro ที่ใช้สำหรับบอกว่าโค้ดที่อยู่หลังนี้ไม่ควรจะถูกเข้าถึงหรือไม่ควรจะเกิดขึ้น และถ้าโค้ดนี้ถูกเข้าถึง จะทำให้โปรแกรมหยุดทำงานและแสดงข้อความว่า "ไม่ควรจะเข้าถึง" หรือ "ไม่ควรจะเกิดขึ้น" ออกทางหน้าจอ
//     assert_eq!(1 + 1, 2); // เป็น macro ที่ใช้สำหรับตรวจสอบว่า expression แรกและ expression ที่สองมีค่าเท่ากันหรือไม่ และถ้าไม่เท่ากัน จะทำให้โปรแกรมหยุดทำงานและแสดงข้อความที่กำหนดออกทางหน้าจอ
//     assert_ne!(1 + 1, 3); // เป็น macro ที่ใช้สำหรับตรวจสอบว่า expression แรกและ expression ที่สองมีค่าไม่เท่ากันหรือไม่ และถ้าเท่ากัน จะทำให้โปรแกรมหยุดทำงานและแสดงข้อความที่กำหนดออกทางหน้าจอ
// }

// excercise: Collatz squence
// for an arbitary n greater than zero

// if n is 1, the sequence terminates
// if n is even, the next number in the sequence is n / 2
// if n is odd, the next number in the sequence is 3 * n + 1

// example (n = 3):
// 3 -> 10 -> 5 -> 16 -> 8 -> 4 -> 2 -> 1 (length = 8)

// fn collatz_length(mut n: u32) -> u32 {
//     let mut length = 1;

//     while n != 1 {
//         if n % 2 == 0 {
//             n = n / 2;
//         } else {
//             n = 3 * n + 1;
//         }
//         length += 1;
//     }

//     length
// }

// fn main() {
//     println!("collatz_length(3): {}", collatz_length(3)); // collatz_length(3) = 8
//     println!("collatz_length(11): {}", collatz_length(11)); // collatz_length(11) = 15
// }

// fn get_index() -> usize {
//     6
// }
// fn main() {
//     // i32; 5
//     let mut a = [1, 2, 3, 4, 5];
//     a[0] = 10; // การเข้าถึงและแก้ไขค่าของอาร์เรย์ a ที่ตำแหน่งที่ 0 โดยกำหนดค่าใหม่เป็น 10
//     // a[6] = 20; // การเข้าถึงและแก้ไขค่าของอาร์เรย์ a ที่ตำแหน่งที่ 6 ซึ่งอยู่นอกขอบเขตของอาร์เรย์ a ที่มีขนาด 5 ทำให้เกิดข้อผิดพลาดในการรันโปรแกรม (index out of bounds) เพราะตำแหน่งที่ 6 ไม่มีอยู่ในอาร์เรย์ a

//     // get_index() เป็นฟังก์ชั่นที่คืนค่า 6 ซึ่งเป็นค่าที่อยู่นอกขอบเขตของอาร์เรย์ a ที่มีขนาด 5 ทำให้เกิดข้อผิดพลาดในการรันโปรแกรม (index out of bounds) เพราะตำแหน่งที่ 6 ไม่มีอยู่ในอาร์เรย์ a
//     // a[get_index()] = 20; // การเข้าถึงและแก้ไขค่าของอาร์เรย์ a ที่ตำแหน่งที่ได้จากฟังก์ชั่น get_index() ซึ่งจะคืนค่า 6 ทำให้เกิดข้อผิดพลาดในการรันโปรแกรม (index out of bounds) เพราะตำแหน่งที่ 6 ไม่มีอยู่ในอาร์เรย์ a
//     // i32; 10 ถือว่าเป็นคนละ type กับ [i32; 5] เพราะมีขนาดและชนิดที่แตกต่างกัน
//     let mut b = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     b[0] = 20; // การเข้าถึงและแก้ไขค่าของอาร์เรย์ b ที่ตำแหน่งที่ 0 โดยกำหนดค่าใหม่เป็น 20

//     // println! ต้องการของที่สามารถ display ได้
//     println!("a: {a:?}"); // พิมพ์ค่าของตัวแปร a ออกทางหน้าจอ โดยใช้รูปแบบ debug
//     dbg!(a); // พิมพ์ค่าของตัวแปร a ออกทางหน้าจอ โดยใช้รูปแบบ debug และแสดงตำแหน่งของโค้ดที่เรียกใช้ dbg! ด้วย
//     println!("b: {b:?}"); // พิมพ์ค่าของตัวแปร b ออกทางหน้าจอ โดยใช้รูปแบบ debug
//     dbg!(b); // พิมพ์ค่าของตัวแปร b ออกทางหน้าจอ โดยใช้รูปแบบ debug และแสดงตำแหน่งของโค้ดที่เรียกใช้ dbg! ด้วย
// }

// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     for x in a {
//         println!("x: {x}");
//     }
// }

// fn main() {
//     //tuple เป็นชนิดข้อมูลที่สามารถเก็บค่าหลายๆ ค่าได้ในตัวเดียว โดยแต่ละค่าจะมีชนิดข้อมูลที่แตกต่างกันได้
//     let t = (1, 3.14, 'A', true); // ประ
//     // กาศตัวแปร t เป็น tuple ที่เก็บค่าต่างๆ ได้แก่ 1 (i32), 3.14 (f64), 'A' (char), และ true (bool)
//     println!("t: {t:?}"); // พิมพ์ค่าของตัวแปร t ออกทางหน้าจอ โดยใช้รูปแบบ debug

//     dbg!(t.0); // 1
//     dbg!(t.1); // 3.14
//     dbg!(t.2); // 'A'
//     dbg!(t.3); // true
//     // dbg!(t.6); // การเข้าถึงค่าที่อยู่นอกขอบเขตของ tuple t ที่มีขนาด 4 ทำให้เกิดข้อผิดพลาดในการรันโปรแกรม (index out of bounds) เพราะตำแหน่งที่ 6 ไม่มีอยู่ใน tuple t
// }

// fn check_order(tuple: (i32, i32, i32)) -> bool {
//     let (left, middle, right) = tuple; // การแยกค่าจาก tuple ออกมาเป็นตัวแปร left, middle, และ right
//     left < middle && middle < right // การตรวจสอบว่า left น้อยกว่า middle และ middle น้อยกว่า right หรือไม่ โดยใช้ตัวดำเนินการเปรียบเทียบ <
// }

// fn main() {
//     let tuple = (1, 5, 3);

//     let result = check_order(tuple); // เรียกใช้ฟังก์ชั่น check_order โดยส่งค่า tuple และเก็บผลลัพธ์ในตัวแปร result
//     println!("Is the tuple in order? {}", result);

//     let tuple2 = (1, 3, 6);

//     let result2 = check_order(tuple2); // เรียกใช้ฟังก์ชั่น check_order โดยส่งค่า tuple2 และเก็บผลลัพธ์ในตัวแปร result2
//     println!("Is the tuple2 in order? {}", result2);
// }

// excercise: array transpose
// implement transpose to turn rows into columns
// example
// 123    147
// 456 => 258
// 789    369

// fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
//     // [[0; 3]; 3]; เป็นการสร้างอาร์เรย์สองมิติที่มีขนาด 3x3 โดยที่แต่ละค่าในอาร์เรย์จะถูกกำหนดเป็น 0
//     let mut transposed = [[0; 3]; 3]; // สร้างอาร์เรย์ใหม่ที่มีขนาดเท่ากับอาร์เรย์ต้นฉบับ แต่ค่าทั้งหมดถูกกำหนดเป็น 0
//     println!("Original matrix: {:?}", matrix); // พิมพ์ค่าของอาร์เรย์ต้นฉบับออกทางหน้าจอ โดยใช้รูปแบบ debug
//     println!("Transposed intitial matrix: {:?}", transposed); // พิมพ์ค่าของอาร์เรย์ transposed ออกทางหน้าจอ โดยใช้รูปแบบ debug

//     for i in 0..3 {
//         // วนลูปผ่านแถวของอาร์เรย์ต้นฉบับ
//         for j in 0..3 {
//             // วนลูปผ่านคอลัมน์ของอาร์เรย์ต้นฉบับ
//             transposed[j][i] = matrix[i][j]; // กำหนดค่าของอาร์เรย์ transposed ที่ตำแหน่ง [j][i] ให้เท่ากับค่าของอาร์เรย์ต้นฉบับที่ตำแหน่ง [i][j]
//         }
//     }

//     transposed // คืนค่าอาร์เรย์ transposed ที่ได้จากการแปลง
// }

// fn main() {
//     let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
//     // เรียกใช้ฟังก์ชั่น transpose โดยส่งค่า array และเก็บผลลัพธ์ในตัวแปร transposed
//     // ทำงานโดยการวนลูปผ่านแถวและคอลัมน์ของอาร์เรย์ต้นฉบับ และกำหนดค่าของอาร์เรย์ transposed ให้เท่ากับค่าของอาร์เรย์ต้นฉบับที่ตำแหน่งที่สลับกัน (j, i) แทน (i, j)
//     // ผลลัพธ์ที่ได้จะเป็นอาร์เรย์ที่มีแถวและคอลัมน์สลับกันจากอาร์เรย์ต้นฉบับ
//     let transposed = transpose(array);
//     println!("Original array: {:?}", array);
//     println!("Transposed array: {:?}", transposed);
// }

// fn main() {
//     let a = 'A';
//     let b = 'B';

//     let r = &a; // r เป็น reference ที่ชี้ไปยังตัวแปร a
//     // เอาออกมาอ่านเฉยๆ
//     println!("r: {r}"); // พิมพ์ค่าของ reference r อ
//     dbg!(r); // พิมพ์ค่าของ reference r ออกทางหน้าจอ
//     //     r: A
//     // [src/main.rs:404:5] r = 'A'

//     let r2 = &a;
//     println!("r2: {r2}"); // พิมพ์ค่าของ reference r2 ออกทางหน้าจอ
//     dbg!(r2); // พิมพ์ค่าของ reference r2 ออกทางจอ
//     // r2: A
//     // [src/main.rs:408:5] r2 = 'A'

//     let mut r3 = &a; // r3 เป็น mutable reference ที่ชี้ไปยังตัวแปร a
//     println!("r3: {r3}"); // พิมพ์ค่าของ mutable reference r3 ออกทางหน้าจอ
//     dbg!(r3); // พิมพ์ค่าของ mutable reference r3 ออกทางหน้าจอ
//     // r3: A
//     // [src/main.rs:413:5] r3 = 'A'
//     // ยืมเฉยๆ ไม่ได้เปลี่ยนแปลงค่าใดๆ ของตัวแปร a
//     r3 = &b; // เปลี่ยน reference r ให้ชี้ไปยังตัวแปร b แทน a
//     println!("r3: {r3}"); // พิมพ์ค่าของ mutable
//     // r3: B
//     // [src/main.rs:415:5] r3 = 'B'
//     dbg!(r3); // พิมพ์ค่าของ mutable reference r3 ออกทางหน้าจอ

//     dbg!(a); // พิมพ์ค่าของตัวแปร a ออกทางหน้าจอ
//     // a: A
//     dbg!(b); // พิมพ์ค่าของตัวแปร b ออกทางหน้าจอ
//     // b: B
// }

// fn main() {
//     let mut point = (3, 4); // ประกาศตัวแปร point เป็น tuple ที่เก็บค่าตำแหน่ง x และ y ของจุดในรูปแบบ (x, y)
//     println!("Original point: {:?}", point); // พิมพ์ค่าของตัวแปร point
//     let x_coordinate = &mut point.0; // สร้าง mutable reference ที่ชี้ไปยังค่าตำแหน่ง x ของ point
//     // dereference mutable reference เพื่อแก้ไขค่าของ x ใน point ผ่าน reference นี้
//     *x_coordinate += 1; // เพิ่มค่า x ของ point โดยการ dereference mutable
//     println!("Updated point: {:?}", point); // พิมพ์ค่าของตัวแปร point หลังจากที่ได้แก้ไขค่า x แล้ว

//     let y_coordinate = &mut point.1; // สร้าง mutable reference ที่ชี้ไปยังค่าตำแหน่ง y ของ point
//     *y_coordinate += 5; // เพิ่มค่า y ของ point โดยการ dereference mutable
//     println!("Updated point: {:?}", point); // พิมพ์ค่าของตัวแปร point หลังจากที่ได้แก้ไขค่า y แล้ว
// }

// fn main() {
//     let a = [10, 20, 30, 40, 50];
//     let s = &a[0..2]; // สร้าง slice s ที่ชี้ไปยังส่วนของอาร์เรย์ a ตั้งแต่ตำแหน่งที่ 0 ถึงตำแหน่งที่ 2 (ไม่รวมตำแหน่งที่ 3)
//     println!("Slice s: {:?}", s); // พิมพ์ค่าของ slice s ออกทางหน้าจอ โดยใช้รูปแบบ debug

//     // slice ไม่สามารถ append หรือ push ค่าใหม่เข้าไปได้ เพราะ slice เป็นเพียงการอ้างอิงถึงส่วนของอาร์เรย์ที่มีอยู่แล้ว และไม่มีความสามารถในการจัดการหน่วยความจำเพื่อเพิ่มขนาดของตัวเอง
// }

// fn main() {
//     // binary literals เป็นค่าคงที่ที่เก็บข้อมูลในรูปแบบของไบนารี ซึ่งมีชนิดเป็น &[u8] และมีขนาดคงที่
//     // binary literals จะถูกเก็บในส่วนของโปรแกรมที่เรียกว่า "data segment" ซึ่งเป็นส่วนที่ใช้เก็บข้อมูลคงที่ของโปรแกรม และจะถูกโหลดเข้าส
//     let s1 = "world"; // s1 เป็น string literal ที่มีชนิดเป็น &str และมีขนาดคงที่

//     let s2 = String::from("hello"); // s2 เป็น String ที่มีชนิดเป็น String และมีขนาดที่สามารถเปลี่ยนแปลงได้
//     let s3 = s2 + " " + s1; // การเชื่อมต่อ String s2 กับ string literal s1 โดยใช้ operator + ซึ่งจะสร้าง String ใหม่ที่มีค่าของ s2 และ s1 รวมกัน
//     println!("s3: {s3}"); // พิมพ์ค่าของตัวแปร s3 ออกทางหน้าจอ

//     let mut s4 = String::from("hello"); // s4 เป็น String ที่มีชนิดเป็น String และมีขนาดที่สามารถเปลี่ยนแปลงได้
//     s4.push_str(s1);
//     println!("s4: {s4}"); // พิมพ์ค่าของตัวแปร s4 ออกทางหน้าจอ หลังจากที่ได้เพิ่ม string literal s1 เข้าไปใน s4 ด้วยฟังก์ชั่น push_str
//     // dbg!(s4); // พิมพ์ค่าของตัวแปร s4 ออกทางหน้าจอ โดยใช้รูปแบบ debug

//     // ถ้า debug  dbg!(s4);  จะทำให้มัน borrowed และไม่สามารถใช้ s4 ได้อีกต่อไป เพราะ dbg! จะทำการยืมค่า s4 เพื่อพิมพ์ออกทางหน้าจอ และเมื่อ dbg! ทำงานเสร็จแล้ว s4 จะถูกคืนค่าและไม่สามารถใช้งานได้อีกต่อไป
//     // borrowing checker ของ Rust จะตรวจสอบว่าเมื่อมีการยืมค่า s4 ไปใช้ใน dbg! แล้ว จะไม่อนุญาตให้ใช้ s4 ในส่วนอื่นของโค้ดหลังจากนั้น เพราะ s4 ถูกยืมไปแล้วและยังไม่ได้คืนค่า
//     let s5 = &s4[0..3]; // s5 เป็น slice ของ string literal s1 ที่ชี้ไปยังส่วนของ s1 ตั้งแต่ตำแหน่งที่ 0 ถึงตำแหน่งที่ 3 (ไม่รวมตำแหน่งที่ 3)
//     println!("s5: {s5}"); // พิมพ์ค่าของตัวแปร s5 ออกทางหน้าจอ
// }

// fn main() {
//     let x_ref = {
//         let x = 10; // x เป็นตัวแปรที่มีชนิด i32 และมีค่าเป็น 10
//         // ทำไม reference ถึงไม่สามารถใช้งานได้หลังจาก block นี้
//         // เพราะ x เป็นตัวแปรที่ถูกสร้างขึ้นภายใน block นี้ และเมื่อ block นี้สิ้นสุดลง ตัวแปร x จะถูกทำลายและไม่สามารถเข้าถึงได้อีกต่อไป
//         &x // คืนค่า reference ที่ชี้ไปยังตัวแปร x
//     };
//     dbg!(x_ref); // พิมพ์ค่าของ reference x_ref ออกทางหน้าจอ โดยใช้รูปแบบ debug
// }

struct Person {
    name: String,
    age: u8,
}

fn describe_person(person: &Person) {
    println!("Name: {}, Age: {}", person.name, person.age); // พิมพ์ค่าของตัวแปร person.name และ person.age ออกทางหน้าจอ
}

fn main() {
    let peter = Person {
        name: String::from("Peter"),
        age: 30,
    };
    describe_person(&peter); // เรียกใช้ฟังก์ชั่น describe_person โดยส่ง reference ของตัวแปร peter

    let jake = Person {
        name: String::from("Jake"),
        ..peter
    };
    describe_person(&jake); // เรียกใช้ฟังก์ชั่น describe_person โดยส่ง reference ของตัวแปร jake
}
