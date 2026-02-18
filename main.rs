// fn main() {
//   let name = "John";
//   let age  = 49.89;
//   println!("My first name is: {}", name);
//   println!("My age is: {}", age);

// }

//   Add number 

// fn main() {
//     let add = 5 + 3;
//     let sub = 10 - 4;
//     let mul = 6 * 2;
//     let div = 12 / 3;
//     let rem = 10 % 3;
  
//     println!("Add: {}", add);
//     println!("Sub: {}", sub);
//     println!("Mul: {}", mul);
//     println!("Div: {}", div);
//     println!("Rem: {}", rem);
//   }


//  Rust if , else 
// fn main() {
//     let a = 5;
//     let b = 10;

//     if a > b {
//         println!("a  is greater than b.");
//     }   else {
//         println!("b is greater than a .");
//     }
// }


//fn main() {
//    println!("Hello World!");
    
//}

//     eng oddiy funksiya yozish 

//println!("Hello World!");         //   bu holatda yozib bolmaydi rust uchun funksiya ichida yozish shart 



// use std::io; 

// fn main() {
//     println!("Hello World!");

//     let mut name = String::new(); 

//     println!("Ismingizni kiriting: ");

//     // 2. Terminaldan ma'lumot o'qiymiz
//     io::stdin()
//         .read_line(&mut name)
//         .expect("Xatolik yuz berdi"); // Xatolikni ushlash uchun

//     println!("My first name is: {}", name.trim());
// }

//        02.18.2026 


// use std::io;

// fn main() {
//     let mut guess = Inted::new();
//     let mut guess1 = String::new();

//     println!("Birinchi so'zni kiriting:");
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("Ikkinchi so'zni kiriting:");
//     io::stdin()
//         .read_line(&mut guess1)
//         .expect("Failed to read line");

//     // trim() Enter (yangi qator) belgisini olib tashlaydi
//     // format! makrosi ikkita stringni birlashtirib yangi String yaratadi
//     let a = format!("{}{}", guess.trim(), guess1.trim());

//     println!("Natija: {}", a);
// }


//   ikki sonni qoshish amalyoti 


// use std::io;

// // Yordamchi funksiya yaratamiz
// fn input_number(label: &str) -> i32 {
//     println!("{}", label);
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("O'qib bo'lmadi");
//     input.trim().parse().expect("Bu son emas!")
// }

// fn main() {
//     println!("--- Rust Kalkulyator ---");

//     // Endi kodingiz ancha qisqa!
//     let a = input_number("Birinchi sonni kiriting:");
//     let b = input_number("Ikkinchi sonni kiriting:");
//     let c = input_number("Uchinchi sonni kiriting:");

    

//     println!("Natija: {} + {} + {}= {}", a, b, c, a + b + c);
// }

//      Hafta kunlarni 

// use std::io;

// // Yordamchi funksiya yaratamiz
// fn input_number(label: &str) -> i32 {
//     println!("{}", label);
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("O'qib bo'lmadi");
//     input.trim().parse().expect("Bu son emas!")
// }

// fn main() {
//     println!("--- Rust Kalkulyator ---");

//     // Endi kodingiz ancha qisqa!
//     let a = input_number("sonni kiriting:");
//     if a == 1 {
//         println!("Dushanba");
//       }
//     if a == 2 {
//     println!("Seshanba");
//     }
//     if a == 3 {
//         println!("Chorshanba");
//     }
//     if a == 4 {
//         println!("Payshanba");
//     }
//     if a == 5 {
//         println!("Juma");
//     }
//     if a == 6 {
//         println!("shanba");
//     }
//     if a == 7 {
//         println!("Yakshanba");
//     }  else {
//         println!("Togri kiriting");
//     }
    

//     //println!("Natija: {} + {} + {}= {}", a);
// }   


//   datatype List


fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("sonlar {a}");
}