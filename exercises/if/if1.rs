// if1.rs

// I AM NOT DONE

fn main() {
	let x: i32 = bigger(5,2);
	println!("output is: {}", x);
	}

pub fn bigger(a: i32, b: i32) -> i32 {
	if a < b {
	b
	} else {
	a
	}    

}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
