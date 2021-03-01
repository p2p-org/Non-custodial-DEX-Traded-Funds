use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

pub trait Print {
    fn print_in_place(self, msg: impl AsRef<str>) -> Self
    where
        Self: Sized,
    {
        self.print(msg);
        self
    }

    fn print(&self, msg: impl AsRef<str>);
}

impl Print for Keypair {
    fn print(&self, msg: impl AsRef<str>) {
        println!(
            "{}: {}, base58 = {}",
            msg.as_ref(),
            self.pubkey(),
            self.to_base58_string()
        );
    }
}

impl Print for Pubkey {
    fn print(&self, msg: impl AsRef<str>) {
        println!("{}: {}", msg.as_ref(), self);
    }
}
