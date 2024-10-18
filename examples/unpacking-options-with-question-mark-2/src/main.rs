struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    // Gets the area code of the phone number of the person's job, if it exists
    fn work_phone_area_code_v1(&self) -> Option<u8> {
        match self.job {
            Some(val) => match val.phone_number {
                Some(x) => x.area_code,
                _ => None,
            },
            _ => None,
        }
    }

    // Gets the area code of the phone number of the person's job, simpler
    fn work_phone_area_code_v2(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439,
            }),
        }),
    };
    let ph = p.work_phone_area_code_v1().unwrap();
    println!("{}", ph);

    let p2 = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(21),
                number: 123,
            }),
        }),
    };
    let ph2 = p2.work_phone_area_code_v2().unwrap();
    println!("{}", ph2);
}
