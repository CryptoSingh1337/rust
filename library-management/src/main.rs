use std::collections::HashMap;

use chrono::{DateTime, Days, Local};

#[derive(Debug, Clone, Copy)]
enum Category {
    ScienceFiction,
    Romance,
    Thriller,
    Autobiography,
    Biography,
}

#[derive(Debug)]
struct Publisher {
    id: u32,
    name: String,
    year_of_publication: u16,
}

#[derive(Debug)]
struct Book {
    id: u32,
    name: String,
    author: String,
    price: f32,
    category: Category,
    isbn: String,
    publisher: Publisher,
}

#[derive(Debug)]
struct Reader {
    id: u32,
    name: String,
    email: String,
    phone_number: String,
}

#[derive(Debug)]
struct Staff {
    id: u32,
    name: String,
}

#[derive(Debug)]
struct Report {
    id: u32,
    reader_id: u32,
    book_id: u32,
    issue_date: DateTime<Local>,
    return_date: DateTime<Local>,
}

#[derive(Debug)]
struct Library {
    name: String,
    staff_members: Vec<Staff>,
    members: Vec<Reader>,
    books: HashMap<String, Book>,
    reports: HashMap<u32, Vec<Report>>,
}

impl Publisher {
    fn new(id: u32, name: String, year_of_publication: u16) -> Self {
        Self {
            id,
            name,
            year_of_publication,
        }
    }

    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            year_of_publication: self.year_of_publication,
        }
    }
}

impl Book {
    fn new(
        id: u32,
        name: String,
        author: String,
        price: f32,
        category: Category,
        isbn: String,
        publisher: Publisher,
    ) -> Self {
        Self {
            id,
            name,
            author,
            price,
            category,
            isbn,
            publisher,
        }
    }
}

impl Reader {
    fn new(id: u32, name: String, email: String, phone_number: String) -> Self {
        Self {
            id,
            name,
            email,
            phone_number,
        }
    }
}

impl Staff {
    fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }
}

impl Report {
    fn new(
        id: u32,
        reader_id: u32,
        book_id: u32,
        issue_date: DateTime<Local>,
        return_date: DateTime<Local>,
    ) -> Self {
        Self {
            id,
            reader_id,
            book_id,
            issue_date,
            return_date,
        }
    }
}

impl Library {
    fn new(name: String) -> Self {
        Self {
            name,
            staff_members: Vec::new(),
            members: Vec::new(),
            books: HashMap::new(),
            reports: HashMap::new(),
        }
    }

    fn add_staff(&mut self, staff_name: String) -> () {
        let id = u32::try_from(&self.staff_members.len() + 1).expect("error while getting the id");
        let staff = Staff::new(id, staff_name);
        self.staff_members.push(staff);
    }

    fn add_book(
        &mut self,
        name: String,
        author: String,
        price: f32,
        category: Category,
        isbn: String,
        publisher: Publisher,
    ) -> () {
        let id = u32::try_from(&self.books.len() + 1).expect("error while getting the id");
        let book = Book::new(id, name.clone(), author, price, category, isbn, publisher);
        self.books.insert(name, book);
    }

    fn add_reader(&mut self, name: String, email: String, phone_number: String) -> () {
        let id = u32::try_from(&self.members.len() + 1).expect("error while getting the id");
        let reader = Reader::new(id, name, email, phone_number);
        self.members.push(reader);
    }

    fn process_entry(&mut self, reader_id: u32, book_id: u32, days_for_return: u64) -> () {
        if !self.reports.contains_key(&reader_id) {
            // This will be borrow request
            let mut user_reports: Vec<Report> = Vec::new();
            let issue_date = Local::now();
            let return_date = issue_date
                .clone()
                .checked_add_days(Days::new(days_for_return))
                .unwrap();
            let report_id =
                u32::try_from(user_reports.len() + 1).expect("error while getting the id");
            let report = Report::new(report_id, reader_id, book_id, issue_date, return_date);
            user_reports.push(report);
            self.reports.insert(reader_id, user_reports);
        } else {
            // Possibility of borrow and return
            let user_reports: &mut Vec<Report> = self.reports.get_mut(&reader_id).unwrap();
            for idx in 0..user_reports.len() {
                if user_reports[idx].book_id == book_id {
                    // Return request
                    user_reports.remove(idx);
                    if user_reports.len() == 0 {
                        self.reports.remove(&reader_id);
                        return;
                    }
                }
            }
            if days_for_return == u64::MAX {
                return;
            }
            let report_id =
                u32::try_from(user_reports.len() + 1).expect("error while getting the id");
            let issue_date = Local::now();
            let return_date = issue_date
                .clone()
                .checked_add_days(Days::new(days_for_return))
                .unwrap();
            let report = Report::new(report_id, reader_id, book_id, issue_date, return_date);
            user_reports.push(report);
        }
    }
}

fn main() {
    let mut library = Library::new("Library".to_owned());

    println!("{:?}", library);

    library.add_staff("John".to_owned());
    library.add_staff("Nena".to_owned());
    library.add_staff("William".to_owned());

    library.add_reader(
        "Constance".to_owned(),
        "constance.robertson@example.com".to_owned(),
        "(379) 218-3024".to_owned(),
    );
    library.add_reader(
        "Michele".to_owned(),
        "michele.richardson@example.com".to_owned(),
        "(760) 419-9840".to_owned(),
    );
    library.add_reader(
        "Zachary".to_owned(),
        "zachary.little@example.com".to_owned(),
        "(339) 527-9505".to_owned(),
    );

    println!("{:?}", library);

    let publisher_one = Publisher::new(1, "PublisherOne".to_owned(), 2024);

    library.add_book(
        "BookOne".to_owned(),
        "AuthorOne".to_owned(),
        1320.0,
        Category::Thriller,
        "1234567890".to_owned(),
        publisher_one.clone(),
    );

    library.add_book(
        "BookTwo".to_owned(),
        "AuthorTwo".to_owned(),
        1500.0,
        Category::ScienceFiction,
        "9876543210".to_owned(),
        publisher_one,
    );

    println!("{:?}", library);

    library.process_entry(1, 1, 3);

    println!("{:?}", library);
    // println!("{:?}", publisherOne);
    // let bookOne = Book::new(
    //     1,
    //     "BookOne".to_owned(),
    //     "AuthorOne".to_owned(),
    //     10.32,
    //     Category::ScienceFiction,
    //     "1234567890".to_owned(),
    //     publisherOne,
    // );
    // println!("{:?}", bookOne);
}
