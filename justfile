run day part:
  cargo run -p {{day}} --bin {{part}}

test day part:
  cargo test -p {{day}} --bin {{part}}

scaffold day:
  cargo run -p cli -- scaffold --day {{day}}