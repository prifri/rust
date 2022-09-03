mod outermost {
    pub fn middle_func() {}
    fn middle_secret_func() {}
    mod insid {
        pub fn inner_func() {}
        fn secret_func() {}
    }
}

fn try_me() {
    outermost::middle_func();
    //outermost::middle_secret_func();
    //outermost::outermost::insid::inner_func();
    //outermost::insid::secret_func();
}
