# Casey

Casey converts strings to whatever case.

## Synopsis

```rust
extern crate casey;
use casey::Casey;

fn main() {
    println!("{}", "FooBARBaz".to_snakecase()); // => "foo_bar_baz"
    println!("{}", "foo_bar_baz".to_camelcase()); // => "fooBarBaz"
}
```

## License

Casey is released under the MIT license.

## Contributing

1. Fork it
2. Create your feature branch (`git checkout -b my-new-feature`)
3. Commit your changes (`git commit -am 'Add some feature'`)
4. Push to the branch (`git push origin my-new-feature`)
5. Create new Pull Request
