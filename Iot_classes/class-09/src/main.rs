use std::io;

use mysql::prelude::*;
use mysql::*;

#[derive(Debug, PartialEq, Eq)]
struct Student {
    sid: i32,
    name: Option<String>,
    email: Option<String>,
    age: Option<String>,
}

fn main() {
    println!("Enter your sid");
    let mut sid = String::new();
    io::stdin().read_line(&mut sid);
    let sid: i32 = sid.trim().parse().unwrap();

    println!("Enter your name");
    let mut name = String::new();
    io::stdin().read_line(&mut name);
    let name = name.trim().parse().unwrap();

    println!("Enter your email");
    let mut email = String::new();
    io::stdin().read_line(&mut email);
    let email = email.trim().parse().unwrap();

    println!("Enter your age");
    let mut age = String::new();
    io::stdin().read_line(&mut age);
    let age = age.trim().parse().unwrap();

    let student = Student {
        sid: sid,
        name: Some(name),
        email: Some(email),
        age: Some(age),
    };
    // insert(student);
    fetch();
    // update(student);
    // delete(student);
}

fn insert(student: Student) {
    let pool = Pool::new("mysql://root:root@localhost:3306/Rust_testing").unwrap();

    println!("Pool : {:?}", pool);

    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.
    let mut conn = pool.get_conn().unwrap();
    println!("conection {:?}", conn);
    let students = vec![student];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    // for mut stmt in pool.prepare(r"INSERT INTO student
    //                                    (sid, name, email, age)
    //                                VALUES
    //                                    (:sid, :name, :email, :age)").into_iter() {
    //     for s in students.iter() {
    //         // `execute` takes ownership of `params` so we pass account name by reference.
    //         // Unwrap each result just to make sure no errors happened.
    //         stmt.execute(params!{
    //             "sid" => s.sid,
    //             "name" => &s.name,
    //             "email" => &s.email,
    //             "age" => &s.age,
    //         }).unwrap();
    //     }
    // }

    conn.exec_batch(
        r"INSERT INTO student (sid, name, email, age)
          VALUES (:sid, :name, :email, :age)",
        students.iter().map(|p| {
            params! {
                "sid" => p.sid,
                "name" => &p.name,
                "email" => &p.email,
                "age"=>&p.age
            }
        }),
    )
    .unwrap();
    println!("Added succesfully");
}

fn update(student: Student) {
    let pool = Pool::new("mysql://root:root@localhost:3306/Rust_testing").unwrap();
    let mut conn = pool.get_conn().unwrap();
    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.
    let students = vec![student];

    conn.exec_batch(
        r"UPDATE student 
        set
        name=:name,
        email=:email,
        age=:age 
        where sid=:sid",
        students.iter().map(|p| {
            params! {
                "sid" => p.sid,
                "name" => &p.name,
                "email" => &p.email,
                "age"=>&p.age
            }
        }),
    )
    .unwrap();

    println!("updated successfully");

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    // for mut stmt in pool.prepare(r"Update tblstudent
    //                                 set name=:name,
    //                                 email=:email, age=:age
    //                                 where sid=:sid
    //                                 ").into_iter() {
    //     for s in students.iter() {
    //         // `execute` takes ownership of `params` so we pass account name by reference.
    //         // Unwrap each result just to make sure no errors happened.
    //         stmt.execute(params!{
    //             "sid" => s.sid,
    //             "name" => &s.name,
    //             "email" => &s.email,
    //             "age" => &s.age,
    //         }).unwrap();
    //     }
    // }
}

fn delete(student: Student) {
    let pool = Pool::new("mysql://root:root@localhost:3306/Rust_testing").unwrap();

    println!("Pool : {:?}", pool);

    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.
    let mut conn = pool.get_conn().unwrap();
    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.
    let students = vec![student];

    conn.exec_batch(
        r"delete from student 
        where sid=:sid",
        students.iter().map(|p| {
            params! {
                "sid" => p.sid,
                // "name" => &p.name,
                // "email" => &p.email,
                // "age"=>&p.age
            }
        }),
    )
    .unwrap();
    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    // for mut stmt in pool.prepare(r"delete from tblstudent
    //                                 where sid=:sid
    //                                 ").into_iter() {
    //     for s in students.iter() {
    //         // `execute` takes ownership of `params` so we pass account name by reference.
    //         // Unwrap each result just to make sure no errors happened.
    //         stmt.execute(params!{
    //             "sid" => s.sid,
    //             "name" => &s.name,
    //             "email" => &s.email,
    //             "age" => &s.age,
    //         }).unwrap();
    //     }
    // }
}

fn fetch() {
    let pool = Pool::new("mysql://root:root@localhost:3306/Rust_testing").unwrap();
    // Let's select payments from database

    let mut conn = pool.get_conn().unwrap();
    let selected_payments = conn
        .query_map(
            "SELECT sid, name, email, age from student",
            |(sid, name, email, age)| Student {
                sid,
                name,
                email,
                age,
            },
        )
        .unwrap();

    // println!("rsult : {:?}", json(selected_payments));

    // let selected_students: Vec<Student> =
    // pool.prep_exec("SELECT sid, name, email, age from tblstudent", ())
    // .map(|result| { // In this closure we will map `QueryResult` to `Vec<Payment>`
    //     // `QueryResult` is iterator over `MyResult<row, err>` so first call to `map`
    //     // will map each `MyResult` to contained `row` (no proper error handling)
    //     // and second call to `map` will map each `row` to `Payment`
    //     result.map(|x| x.unwrap()).map(|row| {
    //         // ⚠️ Note that from_row will panic if you don't follow your schema
    //         let (sid, name, email, age) = my::from_row(row);
    //         Student {
    //             sid: sid,
    //             name: name,
    //             email: email,
    //             age: age,
    //         }
    //     }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    // }).unwrap(); // Unwrap `Vec<Payment>`

    // for s in 0..selected_students.len(){
    //     println!("ID: {} Name: {:?} Email: {:?} Age: {:?}",selected_students[s].sid,selected_students[s].name,
    //                     selected_students[s].email,selected_students[s].age);
}
