cfg_if::cfg_if! {
    if #[cfg(feature = "32bit")] {
        pub type Word = u32;
        pub type SWord = i32;
        pub type Vaddr = Word;
        pub type Paddr = u32;
    }
}

macro_rules! mux {
    ($name:tt, $a:expr,$b:expr) => {
        if cfg!(feature = $name) {
            $a
        } else {
            $b
        }
    };
}

macro_rules! bitmask {
    ($x:expr) => {
        (1 << $x) - 1
    };
}

macro_rules! bits {
    ($x:expr,$hi:expr,$lo:expr) => {
        ($x >> $lo) & bitmask!($hi - $lo + 1)
    };
}

macro_rules! sext {
    ($x:expr, $len:expr) => {{
        let mask = (1u64 << $len) - 1; // 创建低 $len 位掩码
        let value = ($x as u64) & mask; // 保留低 $len 位
        if value & (1 << ($len - 1)) != 0 {
            // 如果符号位为1，则扩展符号
            (value | !mask) as $crate::common::Word
        } else {
            (value) as $crate::common::Word
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn bits_test() {
        let i: u32 = 0x297;
        let rd = bits!(i, 11, 7);
        assert_eq!(5, rd);
    }

    #[test]
    fn sext_test() {
        let x = sext!(1, 1);
        assert_eq!(u32::MAX,x);
    }
}
