
/*
## 1. Todo List Dasturi

### Reja:

Maqsad:  
Foydalanuvchilarga vazifalar ro'yxatini yaratish, ularni qo'shish, ko'rish, va o'chirish imkoniyatini beruvchi dastur.

Bosqichlar:
1. Oddiy interfeys va menyuni yaratish.
2. Vazifalarni qo'shish va ko'rish funksiyalarini qo'shish.
3. Vazifalarni o'chirish imkoniyatini qo'shish (ixtiyoriy).
4. Dasturdan chiqish qismiga ishlov berish.

Algoritm:
1. Bo'sh Vec<String> (vazifalar ro'yxati) yaratish.
2. Foydalanuvchidan tanlovni so'rash:
   - 1. Vazifa qo'shish.
   - 2. Vazifalarni ko'rish.
   - 3. Vazifani o'chirish (ixtiyoriy).
   - 4. Chiqish.

3. Agar foydalanuvchi 1-ni tanlasa:
   - Foydalanuvchidan vazifani kiriting.
   - Vazifani Vec<String> ga qo'shing.
4. Agar foydalanuvchi 2-ni tanlasa:
   - Vec<String> dagi barcha vazifalarni ko'rsatish.
5. Agar foydalanuvchi 3-ni tanlasa (ixtiyoriy):
   - Foydalanuvchidan o'chirilishi kerak bo'lgan vazifaning indeksini kiriting.
   - Vazifani Vec<String> dan o'chirish.
6. Agar foydalanuvchi 4-ni tanlasa:
   - Dasturdan chiqish.  
7. Qadamlarni qayta bajarish.
*/
use std::io;

fn main() {
    println!("Todo list dasturiga xush kelibsiz");

    let mut toso_list:Vec<String> = Vec::new();

    loop {
        println!("Tanlovni kiriting: 1. qo'shish, 2. ko'rish, 3. o'chirish, 4. chiqish");
        let mut tanlash = String::new();
        io::stdin().read_line(&mut tanlash).expect("qiymat kiritishda xatolik");
        let tanlash: u32 = tanlash.trim().parse().expect("raqam kiriting");

        match tanlash {
            1 => {
                println!("Vazifani kiriting:");
                let mut vazifa = String::new();
                io::stdin().read_line(&mut vazifa).expect("vazifa kiritishda xatolik");
                toso_list.push(vazifa.trim().to_string());
            }
            2 => {
                println!("Sizning vazifalaringiz:");
                for (index, vazifa) in toso_list.iter().enumerate() {println!("{} {}", index + 1, vazifa)}
            }
            3 => {
                println!("Vazifalarni o'chirish: idexni kiriting:");
                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("index kiritishda xatolik");
                let index: usize = index.trim().parse().expect("raqam kiriting");

                if index == 0 || index > toso_list.len() {
                    println!("Notugri index kiritilgan")
                }else {
                    toso_list.remove(index -1);
                    println!("Vazifa o'chirildi!\nqolgan vazifalar: {:?}", toso_list)
                }
            }
            4 => {println!("dasturdan toxtadi"); break; }
            _ => println!("Notug'ri tanlov")

        } 
    }


}
