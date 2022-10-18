//mypage作成する関数
pub fn mypage() -> Template {
    let context = MyPageContext {
        title: "MyPage".to_string(),
    };
    Template::render("mypage", &context)
}