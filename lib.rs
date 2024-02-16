// lib.rs

use ink_lang::contract;

#[contract]
mod simple_contract {
    #[ink(storage)]
    pub struct SimpleContract {
        value: i32,
    }

    impl SimpleContract {
        #[ink(constructor)]
        pub fn new(initial_value: i32) -> Self {
            Self { value: initial_value }
        }

        #[ink(message)]
        pub fn get_value(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn set_value(&mut self, new_value: i32) {
            self.value = new_value;
        }

        #[ink(message)]
        pub fn send_message(&self, message: String) {
            // Burada ağa mesaj gönderme işlemini gerçekleştirebilirsiniz.
            // Bu örnekte, sadece bir print yapılıyor.
            ink_env::debug_println(&format!("Received message: {}", message));
        }
    }
}
