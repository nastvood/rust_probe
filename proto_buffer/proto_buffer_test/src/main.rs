fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use proto_buffer::*;
    use proto_buffer_derive::*;

    #[derive(Debug, PartialEq, ProtoBufferReader, ProtoBufferWriter)]
    struct User {
        name: String,
        email: String,
        age: u8
    }

    //cargo test -- --nocapture

    #[test]
    fn user() {
        let mut b = Buffer::new();

        let user = User {
            name: String::from("Den"),
            email: String::from("nastvood@gmail.com"),
            age: 37
        };

        user.proto_write(&mut b);

        println!("{:?}", b);

        b.pos = 0;

        let readed_user = User::proto_read(&mut b);

        assert_eq!(user, readed_user);
    }
}

