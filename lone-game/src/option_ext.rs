pub trait OptionExt<T: Sized> {
    fn get_or_assign(&mut self, create:&dyn Fn() -> T) -> &T;
    fn get_or_assign_result<F, D>(&mut self, create:F) -> Result<&T, D> where F: FnOnce() -> Result<T, D>;
}

impl<T: Sized> OptionExt<T> for Option<T> {
    fn get_or_assign(&mut self, create:&dyn Fn() -> T) -> &T {
        if self.is_some() {
            return self.as_ref().unwrap()
        }
        //initialize
        *self = Some(create());
        return self.as_ref().unwrap();
    }

    fn get_or_assign_result<F, D>(&mut self, create:F) -> Result<&T, D>
    where F: FnOnce() -> Result<T, D>, {

        if self.is_some() {
            return Ok(self.as_ref().unwrap())
        }
        //initialize
        *self = Some(create()?);
        return Ok(self.as_ref().unwrap());
    }
}