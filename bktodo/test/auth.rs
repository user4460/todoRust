//authのテストを書く

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::establish_connection;
    use crate::models::user::User;
    use crate::schema::users::dsl::*;
    use diesel::prelude::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    #[test]
    fn test_login() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let conn = establish_connection();
        let user = User::new("test", "test")
            .save(&conn)
            .expect("valid user");
        let response = client
            .post("/login")
            .header(ContentType::Form)
            .body(format!("username={}&password={}", user.username, user.password))
            .dispatch();
        assert_eq!(response.status(), Status::SeeOther);
        let cookie = response.headers().get_one("Set-Cookie").unwrap();
        let response = client
            .get("/todo")
            .header(ContentType::Form)
            .header(rocket::http::Header::new("Cookie", cookie))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        diesel::delete(users.filter(id.eq(user.id))).execute(&conn).unwrap();
    }
}
