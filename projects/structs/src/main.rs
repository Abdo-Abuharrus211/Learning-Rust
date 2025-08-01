struct Dev {
    specialty: String,
    name: String,
    // in thousands
    total_comp: u64,
    email: String,
    employed: bool,
}

fn main() {
    let dev_one = Dev {
        specialty: String::from("systems programming"),
        name: String::from("Foltest"),
        total_comp: 124,
        email: String::from("foltest@bestcomp.dev"),
        employed: false,
    };

    let mut dev_two = build_dev(
        String::from("Roach"),
        String::from("roach@kaermorhen.ka"),
        254,
    );

    dev_two.specialty = String::from("Security");
    dev_two.employed = true;
}

fn build_dev(name: String, email: String, total_comp: u64) -> Dev {
    Dev {
        specialty: String::new(),
        name,
        email,
        total_comp,
        employed: false,
    }
}
