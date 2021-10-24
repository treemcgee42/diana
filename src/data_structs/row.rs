/*
 * row.rs
 * 
 * Implements row structure
 */

use crate::data_structs::{
    *,
};

pub struct Row {
    id: u32,
    username: String,
    email: String,
}

impl Row {
    pub fn new() -> Row
    {
        Row {
            id: 0,
            username: String::new(),
            email: String::new(),
        }
    }

    pub fn create <S: Into<String>>(id: u32, username_: S, email_: S) -> Row
    {
        let mut username = username_.into();
        let mut email = email_.into();

        username.truncate(USERNAME_SIZE - 1);
        email.truncate(EMAIL_SIZE - 1);

        Row {
            id,
            username,
            email,
        }
    }
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.id, self.username, self.email)
    }
}

impl Serializable for Row {
    fn serialize(&self) -> Vec<Option<u8>>
    {
        let mut row_as_bytes: Vec<Option<u8>> = (0..ROW_SIZE).map(|_| None).collect();

        copy_bytes_to_vec(&mut row_as_bytes, &self.id.to_ne_bytes(), ID_OFFSET, ID_SIZE);
        copy_bytes_to_vec(&mut row_as_bytes, self.username.as_bytes(), USERNAME_OFFSET, USERNAME_SIZE);
        copy_bytes_to_vec(&mut row_as_bytes, self.email.as_bytes(), EMAIL_OFFSET, EMAIL_SIZE);

        return row_as_bytes;
    }

    fn deserialize(arr: &[Option<u8>]) -> Row
    {
        let id: u32 =   u32::from_ne_bytes(
                            u8_from_option_u8(
                                arr, 
                                ID_OFFSET, 
                                ID_SIZE
                            )
                            .try_into()
                            .unwrap()
                        );

        let username: String =  String::from_utf8(
                                    u8_from_option_u8(
                                        arr,
                                        USERNAME_OFFSET, 
                                        USERNAME_SIZE
                                    )
                                )
                                .unwrap();

        let email: String = String::from_utf8(
                                u8_from_option_u8(
                                    arr, 
                                    EMAIL_OFFSET, 
                                    EMAIL_SIZE
                                )
                            )
                            .unwrap();

        return  Row {
                    id, 
                    username,
                    email,
                }
    }
}

/*********/
/* Tests */
/*********/

#[cfg(test)]
mod tests {
    use super::*;
    use page::{Page};

    #[test]
    fn test_row_serialize_deserialize() 
    {
        let row = Row::create(1, "cstack", "foo@bar.com");

        let row_ser: Vec<Option<u8>> = row.serialize();
        let row_de: Row = Row::deserialize(row_ser.as_slice());

        assert_eq!(row.id, row_de.id);
        assert_eq!(row.email, row_de.email);
        assert_eq!(row.username, row_de.username);
    }
}
