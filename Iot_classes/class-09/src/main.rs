
use std::io;

use mysql::prelude::*;
use mysql::*;


#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}
fn insert(){
// let url = "mysql://root:root@localhost:3306/Rust_testing";

// let pool = Pool::new(url);

let pool = Pool::new("mysql://root:root@localhost:3306/Rust_testing").unwrap();

    println!("Pool : {:?}", pool);

    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.
    // let mut conn=pool.get_conn();
    // println!("conection {:?}", conn);


let mut conn = pool.get_conn();

// Let's create a table for payments.
conn.query_drop(
    r"CREATE TEMPORARY TABLE payment (
        customer_id int not null,
        amount int not null,
        account_name text
    )");

let payments = vec![
    Payment { customer_id: 1, amount: 2, account_name: None },
    Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
    Payment { customer_id: 5, amount: 6, account_name: None },
    Payment { customer_id: 7, amount: 8, account_name: None },
    Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
];

// Now let's insert payments to the database
conn.exec_batch(
    r"INSERT INTO payment (customer_id, amount, account_name)
      VALUES (:customer_id, :amount, :account_name)",
    payments.iter().map(|p| params! {
        "customer_id" => p.customer_id,
        "amount" => p.amount,
        "account_name" => &p.account_name,
    })
);

// Let's select payments from database. Type inference should do the trick here.
let selected_payments = conn
    .query_map(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| {
            Payment { customer_id, amount, account_name }
        },
    );

// Let's make sure, that `payments` equals to `selected_payments`.
// Mysql gives no guaranties on order of returned rows
// without `ORDER BY`, so assume we are lucky.
assert_eq!(payments, selected_payments);
println!("Yay!");

}

fn main(){
    insert();
}

/*
#[derive(Debug, PartialEq, Eq)]
struct Student {
    sid: i32,
    name: Option<String>,
    email: Option<String>,
    age: Option<String>,
}

fn main() {
    // let pool = Pool::new("mysql://root:root@localhost:3306/Rust_testing");

    // println!("Pool : {:?}", pool);

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
    insert(student);
    // fetch();
}

fn insert(student: Student) {
    let pool = Pool::new("mysql://root:root@localhost:3306/Rust_testing").unwrap();

    println!("Pool : {:?}", pool);

    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.
    let mut conn=pool.get_conn();
    println!("conection {:?}", conn);
    let pool=pool.clone();
    let students = vec![
        student
    ];


    let mut result=pool.prep_exec("INSERT INTO student (sid) VALUES (:sid)");
    // conn.query_drop(
    //     r"CREATE TEMPORARY TABLE student1 (
    //         sid int not null,
    //         name text,
    //         email text,
    //         age text
    //     )")?;

    println!("rsutl : {:?}", result);
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


    // let mut stmt1=pool.prepare("")

    // conn.exec_batch(
    //     r"INSERT INTO student1 (sid, name, email, age)
    //       VALUES (:sid, :name, :email)",
    //     students.iter().map(|p| params! {
    //         "sid" => p.sid,
    //         "name" => p.name,
    //         "email" => &p.email,
    //         "age"=>&p.age
    //     })
    // );
    println!("Added succesfully");
}

/* update(student:Student){
    let pool = my::Pool::new("mysql://root:@localhost:3306/test").unwrap();
    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.
    let students = vec![
       student
    ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"Update tblstudent
                                    set name=:name,
                                    email=:email, age=:age
                                    where sid=:sid
                                    ").into_iter() {
        for s in students.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params!{
                "sid" => s.sid,
                "name" => &s.name,
                "email" => &s.email,
                "age" => &s.age,
            }).unwrap();
        }
    }
}




fn delete(student: Student){
    let pool = my::Pool::new("mysql://root:@localhost:3306/test").unwrap();
    // // Let's create payment table.
    // // Unwrap just to make sure no error happened.
    let students = vec![
        student
    ];

    // Let's insert payments to the database
    // We will use into_iter() because we do not need to map Stmt to anything else.
    // Also we assume that no error happened in `prepare`.
    for mut stmt in pool.prepare(r"delete from tblstudent
                                    where sid=:sid
                                    ").into_iter() {
        for s in students.iter() {
            // `execute` takes ownership of `params` so we pass account name by reference.
            // Unwrap each result just to make sure no errors happened.
            stmt.execute(params!{
                "sid" => s.sid,
                "name" => &s.name,
                "email" => &s.email,
                "age" => &s.age,
            }).unwrap();
        }
    }
}






fn fetch(){
    let pool = my::Pool::new("mysql://root:@localhost:3306/test").unwrap();
    // Let's select payments from database
    let selected_students: Vec<Student> =
    pool.prep_exec("SELECT sid, name, email, age from tblstudent", ())
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
                age: age,
            }
        }).collect() // Collect payments so now `QueryResult` is mapped to `Vec<Payment>`
    }).unwrap(); // Unwrap `Vec<Payment>`

    for s in 0..selected_students.len(){
        println!("ID: {} Name: {:?} Email: {:?} Age: {:?}",selected_students[s].sid,selected_students[s].name,
                        selected_students[s].email,selected_students[s].age);

    }
}*/
*/