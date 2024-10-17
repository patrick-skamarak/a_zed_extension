use constcat::concat_slices;
use zed_extension_api::serde_json::Value;
mod java;

const WHAT: &'static str = "Hey";
const IDK: &[&'static str] = &[WHAT, "HI"];
const CONCAT: &[&str] = concat_slices!([&str]:IDK, &["HIIIII"]);

struct Wrapper<T>(T);

impl Wrapper<Value> {
    pub fn get(&self, json_tag_list: &[&str]) -> Result<Option<Value>, &str> {
        return self.recursive_get(&mut json_tag_list.into_iter());
    }
    fn recursive_get<'a, T: Iterator<Item = &'a &'a str>>(
        &self,
        json_tag_iterator: &mut T,
    ) -> Result<Option<Value>, &str> {
        println!("{:?}", json_tag_iterator.next());
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use zed_extension_api::serde_json::Number;

    use super::*;

    #[test]
    fn test_name() {
        let something = ["a", "b", "c", "d"];
        println!("{:?}", CONCAT);
        let a_value = Value::Number(Number::from(12));
        _ = Wrapper(a_value).get(&something);
    }
}
