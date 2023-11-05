// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.



#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
         match optional_target {
            Some(word)=>assert_eq!(word, target),
            _=>println!("fd"),
        };
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor= 10;

        // TODO: make this a while let statement - remember that vector.pop also
        // adds another layer of Option<T>. You can stack `Option<T>`s into
        // while let and if let.

        // for i in 1..(range + 1) {
        //     if let Some(integer)=optional_integers.get(i)
        //     {
        //         assert_eq!(integer, i);
        //     }
        // }
    
        while let Some(i) =  optional_integers.pop(){
            // optional_integers.pop() ;
            if let Some(integer)=i{
                //  println!("{} {}",integer,cursor);
                 assert_eq!(integer, cursor);
                cursor -= 1;
            }
            // 
            
        };
            
        
        

         assert_eq!(cursor, 0);
    }
}
