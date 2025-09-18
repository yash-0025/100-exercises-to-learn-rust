pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    // The string on stack will only store the data like pointer, length and capacity and as it is a 8 byte each value which means 24 bytes
    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24 );
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        // AS string on stack has only values and not the data so 24+24+24
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
