macro_rules! calculate {
(eval $e:expr) => {{
      {
          let val: usize = $e; // 强制类型为整型
          println!("eval {} = {}", stringify!{$e}, val);
      }
  }};
(eval $e:expr,$(eval $es:expr),+) => {{
    {
        calculate! {eval $e }
        calculate! { $(eval $es),+ }
    }
}};
(pl $e:expr) => {{
    {
        let val: usize = $e; // 强制类型为整型
        println!("pl {} = {}", stringify!{$e}, val);
    }
}};
(pl $p:expr,$(pl $ps:expr),+) => {{
    {
        calculate! {pl $p }
        calculate! { $(pl $ps),+ }
    }
}};


}

#[test]
fn main() {
    calculate! {
        eval 1+2
    }

    calculate! {
        pl 1+2
    }
}
