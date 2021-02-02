fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use proto_buffer::*;
    use proto_buffer_derive::*;

    #[derive(Debug, PartialEq, ProtoBufferWriter, ProtoBufferReader)]
    enum UserStatus {
        Student (u8),
        Worker (String),
        Nothing
    }

    #[derive(Debug, PartialEq, ProtoBufferWriter, ProtoBufferReader)]
    struct User {
        name: String,
        email: String,
        age: u8
    }

    //cargo test -- --nocapture
    
    #[test]
    fn user_status() {
        let mut b = Buffer::new();

        let us_student = UserStatus::Student(4);
        let us_worker = UserStatus::Worker(String::from("Horns and hooves"));
        let us_nothing = UserStatus::Nothing;

        us_student.proto_write(&mut b);
        us_worker.proto_write(&mut b);
        us_nothing.proto_write(&mut b);

        //println!("{:?}", b);

        b.pos = 0;

        assert_eq!(us_student, UserStatus::proto_read(&mut b));
        assert_eq!(us_worker, UserStatus::proto_read(&mut b));
        assert_eq!(us_nothing, UserStatus::proto_read(&mut b));
    }

    #[test]
    fn user() {
        let mut b = Buffer::new();

        let user = User {
            name: String::from("Den"),
            email: String::from("nastvood@gmail.com"),
            age: 37
        };

        user.proto_write(&mut b);

        //println!("{:?}", b);

        b.pos = 0;

        let readed_user = User::proto_read(&mut b);

        assert_eq!(user, readed_user);
    }
}

