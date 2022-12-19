


/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {
	n == ((n << (64 - width)) as i64) >> (64 - width)
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
	n == (n << (64 - width)) >> (64 - width)
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    return ((word << (64 - width - lsb)) as i64) >> (64 - width);
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
	// return 0x value
    return (word << (64 - width - lsb)) >> (64 - width);
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    let word_width = word.count_zeros() + word.count_ones();
    if width > word_width.into()  || width + lsb >  word_width.into() {
        panic!("You cannot retrieve that value from the bit field and/or word of that length");
    }else{
        if fitsu(value,width){
            let left = (word >> (lsb+width)) <<(lsb+width); // clears everything on right
            let right = shr(shl(word,word_width as u64 - lsb),word_width as u64 - lsb); //clears everything on left
            let val = value <<lsb; //moves value to position of lsb with trailing 0s to the right
            return Some(left | right as u64 | val);
        } else {
            return None;
        }
    }  
}

// a reasonable approach for shifting by 64 or more would be to set the result to be all zeroes/all ones?
#[inline]
pub fn shl(word: u64, shift: u64)->u64{
        return word << shift;
}
#[inline]
pub fn shr(word:u64,shift:u64)->u64{
        return word >> shift;
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    if fitss(value, width) {
    	Some((!((!0_u64 >> (64 - width)) << lsb) & word) | (((value & (!(-1_i128 << width)) as i64) as u64) << lsb))
    } else {
    	None
    }
}


#[cfg(test)]
mod tests {
	use crate::bitpack::*;
	// 8 bit width tests

	// fitss
	// n bounded by {-128, 127}
	#[test]
	fn fitss_lower_bound() {
		assert!(fitss(-128, 8));
		assert!(!fitss(-129, 8));
	}
	// n bounded by {0, 255}
	#[test]
	fn fitss_upper_bound() {
		assert!(fitss(127, 8));
		assert!(!fitss(128, 8));
	}

	//fitsu
	// n bounded by {0, 255}
	#[test]
	fn fitsu_lower_wound() {
		assert!(fitsu(0, 8));
	}
	#[test]
	fn fitsu_upper_bound() {
		assert!(fitsu(255, 8));
		assert!(!fitsu(256, 8));
	}

	// build_word and get_word are general tests
	#[test]
	fn build_word() {
		// original word
		// 0 x 32 ... 1 x 32
		// numbers to input (left to right)
		// -3 4 1 15 2 6 -8 -1
		// result
		// 0xD41F286
		let mut word: u64 = !0_u32 as u64;
		// news and newu should delete the 1's in their way
		word = news(word, 4, 28, -3).unwrap();
		word = newu(word, 4, 24, 4 ).unwrap();
		word = newu(word, 4, 20, 1 ).unwrap();
		word = newu(word, 4, 16, 15).unwrap();
		word = newu(word, 4, 12, 2 ).unwrap();
		word = newu(word, 4, 8, 6  ).unwrap();
		word = news(word, 4, 4, -8 ).unwrap();
		word = news(word, 4, 0, -1 ).unwrap();

		assert_eq!(word, 0xD41F268F);
	}

	#[test]
	fn get_word() {
		let word: u64 = 0xD41F268F;
		assert_eq!(gets(word, 4, 28), -3);
		assert_eq!(getu(word, 4, 24),  4);
		assert_eq!(getu(word, 4, 20),  1);
		assert_eq!(getu(word, 4, 16), 15);
		assert_eq!(getu(word, 4, 12),  2);
		assert_eq!(getu(word, 4,  8),  6);
		assert_eq!(gets(word, 4,  4), -8);
		assert_eq!(gets(word, 4,  0), -1);
	}

	// follwing tests check if user gives maximum width of 64
	#[test]
	fn newu_bounds() {
		let mut word: u64 = 0;
		word = newu(word, 64, 0, !0_u64).unwrap();
		assert_eq!(word, !0_u64);
	}
	#[test]
	fn news_bounds() {
		let mut word: u64 = 0;
		word = news(word, 64, 0, -1_i64).unwrap();
		assert_eq!(word, !0_u64);
	}

	#[test]
	fn getu_bounds() {
		let word: u64 = !0_u64;
		assert_eq!(getu(word, 64, 0), !0_u64);
	}
	#[test]
	fn geti_bounds() {
		let word: u64 = !0_u64;
		assert_eq!(gets(word, 64, 0), -1_i64);
	}

}
