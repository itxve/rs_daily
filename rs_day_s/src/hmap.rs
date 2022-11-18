use std::collections::HashMap;

#[derive(Debug)]
enum JsonValue<'a> {
    String(&'a str),
    Some(u8),
    None,
    VecOfString(Vec<String>),
    AnotherHashMap(HashMap<&'a str, u32>),
}

#[test]
fn map_use() {
    let mut contacts: HashMap<&str, JsonValue> = HashMap::new();
    let some123 = contacts.insert("a", JsonValue::Some(123));
    let some234 = contacts.insert("a", JsonValue::Some(234));

    println!("some1:{:?},some2:{:?}", some123, some234);
    contacts.insert("b", JsonValue::None);

    match contacts.get("a").unwrap() {
        JsonValue::Some(us) => {
            println!("VV,{}", us);
        }
        JsonValue::None => {
            println!("None:");
        }
        _ => {}
    }
}

#[test]
fn map_custom_use() {
    let mut contacts: HashMap<String, u8> = HashMap::new();
    let some123 = contacts.insert(String::from("a"), 123);

    let ef = contacts.get("a").unwrap();
    println!("{}", ef)
}

// Eq 要求你对此类型推导 PartiaEq。
#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_login<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logon = Account {
        username: username,
        password: password,
    };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        }
        _ => println!("Login failed!"),
    }
}

#[test]
fn try_login_test() {
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };

    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };

    accounts.insert(account, account_info);

    try_login(&accounts, "j.everyman", "psasword123");

    println!("##########The second account login##########");

    try_login(&accounts, "j.everyman", "password123");
}
