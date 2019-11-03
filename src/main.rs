#[macro_use]
extern crate mysql;


use mysql as my;

#[derive(Debug, PartialEq, Eq)]
struct Student {
    sid: i32,
    name: Option<String>,
    email: Option<String>,
    age: Option<String>,
}

fn main() {
    // See docs on the `OptsBuilder`'s methods for the list of options available via URL.
    let pool = my::Pool::new("mysql://root:@localhost:3306/test").unwrap();

    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.


    // let students = vec![
    //     Student { sid: 1, name:Some("MZ".into()), email:Some("mz@mydomain.com".into()),age: Some("31".into()) },
    // ];

    // // // Let's insert payments to the database
    // // // We will use into_iter() because we do not need to map Stmt to anything else.
    // // // Also we assume that no error happened in `prepare`.
    // for mut stmt in pool.prepare(r"INSERT INTO tblStudent
    //                                    (sid, name, email, age)
    //                                VALUES
    //                                    (:sid, :name, :email, :age)").into_iter() {
    //     for p in students.iter() {
    //         // `execute` takes ownership of `params` so we pass account name by reference.
    //         // Unwrap each result just to make sure no errors happened.
    //         stmt.execute(params!{
    //             "sid" => p.sid,
    //             "name" => &p.name,
    //             "email" => &p.email,
    //             "age" => &p.age,               
    //         }).unwrap();
    //     }
    // }

    // Let's select payments from database
    let selected_students: Vec<Student> =
    pool.prep_exec("SELECT sid, name, email, age from tblStudent", ())
    .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
        // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
        // will map each `MyResult` to contained `row` (no proper error handling)
        // and second call to `map` will map each `row` to `Payment`
        result.map(|x| x.unwrap()).map(|row| {
            // ⚠️ Note that from_row will panic if you don't follow your schema
            let (sid, name, email, age) = my::from_row(row);
            Student {
                sid: sid,
                name: name,
                email: email,
                age : age,
            }
        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    }).unwrap(); // Unwrap `Vec<Payment>`
for s in 0..(selected_students.len()){
println!("Name {:?}",selected_students[s].name);
}

}
