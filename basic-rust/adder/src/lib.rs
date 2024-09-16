#[derive(Debug)]
struct Rectangle {
    w:u32,
    h:u32,
}

impl Rectangle {
    fn can_hold (&self , other: &Rectangle) -> bool {
          self.w > other.w && self.h > other.w
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l_can_hold_s(){
        let larget = Rectangle {
            w:10,
            h:12,
        };
        let smaller = Rectangle{
            w:8,
            h:10,
        };
        assert!(larget.can_hold(&smaller));
    }

    #[test]
    fn s_can_not_hold_l(){
        let larger = Rectangle {
            w:10,
            h:12,
        };
        let smaller = Rectangle{
            w:8,
            h:10,
        };

        assert!(!smaller.can_hold(&larger));
    }

}

